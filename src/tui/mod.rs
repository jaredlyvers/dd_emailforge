use std::io;
use std::path::PathBuf;
use std::time::Duration;

pub(super) use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
pub(super) use ratatui::layout::{Constraint, Direction, Layout, Rect};
pub(super) use ratatui::style::{Modifier, Style};
pub(super) use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Terminal;

use crate::model::Template;
use crate::storage;

pub(in crate::tui) use draw::centered_rect;

mod draw;
mod events;
mod help;
mod modals;
mod theme;
mod toasts;
#[cfg(test)]
mod tests;

use theme::*;
use toasts::*;

pub(in crate::tui) use modals::{ConfirmKind, Modal, ModalResult};

pub(super) const AUTOSAVE_DEBOUNCE: Duration = Duration::from_secs(2);

pub fn run_tui(path: Option<PathBuf>) -> anyhow::Result<()> {
    let (theme, theme_source, load_warning) = AppTheme::load();

    let resolved = match path {
        None => None,
        Some(p) => Some(storage::resolve_template_path(&p)?),
    };

    let mut loaded: Option<Template> = None;
    let mut load_error: Option<String> = None;
    if let Some(p) = resolved.as_ref() {
        match storage::load_template(p) {
            Ok(t) => loaded = Some(t),
            Err(e) => load_error = Some(e.to_string()),
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let keep_path = load_error.is_none();
    let mut app = App::new(
        theme,
        theme_source,
        load_warning.clone(),
        loaded,
        if keep_path { resolved } else { None },
    );
    if let Some(msg) = load_warning {
        app.push_toast(ToastLevel::Warning, msg);
    }
    if app.template.is_none() && load_error.is_none() {
        app.push_toast(
            ToastLevel::Info,
            "No template open. Run: dd_emailforge init <dir>",
        );
    }
    if let Some(msg) = load_error {
        app.modal = Some(Modal::LoadError { message: msg });
    }
    if let (Some(p), Some(_)) = (app.path.as_ref(), app.template.as_ref()) {
        let backup = storage::backup_path_for(p);
        if backup.exists() && p.exists() {
            if let (Ok(main), Ok(bak)) = (std::fs::read_to_string(p), std::fs::read_to_string(&backup))
            {
                if main != bak {
                    app.push_toast(
                        ToastLevel::Info,
                        "Loaded state differs from last manual save.",
                    );
                }
            }
        }
    }

    let run_res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    run_res
}

pub(super) struct App {
    template: Option<Template>,
    path: Option<PathBuf>,
    theme: AppTheme,
    theme_source: String,
    header_copy: String,
    toasts: Vec<Toast>,
    body_area: Rect,
    should_quit: bool,
    show_help: bool,
    help_scroll: u16,
    help_scroll_max: u16,
    show_theme: bool,
    theme_scroll: u16,
    theme_scroll_max: u16,
    theme_status: Option<String>,
    modal: Option<Modal>,
    dirty: bool,
    dirty_since: Option<std::time::Instant>,
    last_saved_json: String,
}

impl App {
    pub(super) fn new(
        theme: AppTheme,
        theme_source: String,
        theme_status: Option<String>,
        template: Option<Template>,
        path: Option<PathBuf>,
    ) -> Self {
        let header_copy = choose_header_copy(&theme.header_quotes);
        let last_saved_json = template
            .as_ref()
            .and_then(|t| serde_json::to_string(t).ok())
            .unwrap_or_default();
        Self {
            template,
            path,
            theme,
            theme_source,
            header_copy,
            toasts: Vec::new(),
            body_area: Rect::default(),
            should_quit: false,
            show_help: false,
            help_scroll: 0,
            help_scroll_max: 0,
            show_theme: false,
            theme_scroll: 0,
            theme_scroll_max: 0,
            theme_status,
            modal: None,
            dirty: false,
            dirty_since: None,
            last_saved_json,
        }
    }

    pub(super) fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> anyhow::Result<()> {
        while !self.should_quit {
            self.tick_autosave(std::time::Instant::now());
            terminal.draw(|f| self.draw(f))?;

            if event::poll(Duration::from_millis(100))? {
                let evt = event::read()?;
                self.handle_event(evt)?;
                self.mark_dirty_if_changed();
            }
        }

        Ok(())
    }

    pub(super) fn footer_hint(&self, width: u16) -> String {
        let dirty = if self.dirty { "*  " } else { "" };
        let parts: &[&str] = if self.modal.is_some() || self.show_help || self.show_theme {
            if width < 80 {
                &["F1:Help", "Esc:Close", "C-q:Quit"]
            } else {
                &["F1:Help", "Esc:Close", "Ctrl+Q:Quit"]
            }
        } else if width < 80 {
            &["F1:Help", "F2:Theme", "F3:Val", "s:Save", "C-q:Quit"]
        } else if width < 120 {
            &[
                "F1: Help",
                "F2: Theme",
                "F3: Validate",
                "s: Save",
                "Ctrl+Q: Quit",
            ]
        } else {
            &[
                "F1: Help",
                "F2: Theme",
                "F3: Validate",
                "s: Save",
                "Ctrl+Q: Quit",
                "(mouse: click/scroll)",
            ]
        };
        format!("{dirty}{}", parts.join("  "))
    }

    pub(super) fn mark_dirty_if_changed(&mut self) {
        let Some(template) = self.template.as_ref() else {
            return;
        };
        let current = match serde_json::to_string(template) {
            Ok(s) => s,
            Err(_) => return,
        };
        if current != self.last_saved_json {
            if !self.dirty {
                self.dirty_since = Some(std::time::Instant::now());
            }
            self.dirty = true;
        }
    }

    pub(super) fn commit_save_with_backup(
        &mut self,
        path: &std::path::Path,
    ) -> anyhow::Result<()> {
        let Some(template) = self.template.as_ref() else {
            anyhow::bail!("no template open");
        };
        storage::save_template(path, template)?;
        let backup = storage::backup_path_for(path);
        std::fs::copy(path, &backup)?;
        self.last_saved_json = serde_json::to_string(template).unwrap_or_default();
        self.dirty = false;
        self.dirty_since = None;
        self.path = Some(path.to_path_buf());
        Ok(())
    }

    pub(super) fn tick_autosave(&mut self, now: std::time::Instant) {
        if !self.dirty {
            return;
        }
        let Some(since) = self.dirty_since else {
            self.dirty_since = Some(now);
            return;
        };
        if now.duration_since(since) < AUTOSAVE_DEBOUNCE {
            return;
        }
        let Some(path) = self.path.clone() else {
            return;
        };
        let Some(template) = self.template.as_ref() else {
            return;
        };
        match storage::save_template(&path, template) {
            Ok(()) => {
                self.last_saved_json = serde_json::to_string(template).unwrap_or_default();
                self.dirty = false;
                self.dirty_since = None;
            }
            Err(e) => {
                self.push_toast(ToastLevel::Error, format!("Autosave failed: {e}"));
            }
        }
    }

    pub(super) fn begin_save(&mut self) {
        if self.template.is_none() {
            self.push_toast(
                ToastLevel::Warning,
                "No template open. Run: dd_emailforge init <dir>",
            );
            return;
        }
        if let Some(path) = self.path.clone() {
            match self.commit_save_with_backup(&path) {
                Ok(()) => {
                    self.push_toast(ToastLevel::Success, format!("Saved {}", path.display()));
                }
                Err(e) => {
                    self.push_toast(ToastLevel::Error, format!("Failed to save: {e}"));
                }
            }
            return;
        }
        self.modal = Some(Modal::SavePrompt {
            path: "template.json".to_string(),
        });
    }

    pub(super) fn request_quit(&mut self) {
        if self.dirty {
            self.modal = Some(Modal::ConfirmPrompt {
                message: "Unsaved changes. Quit anyway?".to_string(),
                on_confirm: ConfirmKind::QuitUnsaved,
            });
        } else {
            self.should_quit = true;
        }
    }
}
