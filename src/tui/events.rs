//! Keyboard and mouse dispatch for chrome (F1/F2/F3/s/Ctrl+Q).
use super::*;

impl App {
    pub(super) fn handle_event(&mut self, evt: Event) -> anyhow::Result<()> {
        if self.show_help {
            match evt {
                Event::Key(k) => match k.code {
                    KeyCode::F(1) | KeyCode::Esc => {
                        self.show_help = false;
                        self.help_scroll = 0;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.help_scroll = self
                            .help_scroll
                            .saturating_add(1)
                            .min(self.help_scroll_max);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.help_scroll = self.help_scroll.saturating_sub(1);
                    }
                    KeyCode::PageDown => {
                        self.help_scroll = self
                            .help_scroll
                            .saturating_add(10)
                            .min(self.help_scroll_max);
                    }
                    KeyCode::PageUp => {
                        self.help_scroll = self.help_scroll.saturating_sub(10);
                    }
                    KeyCode::Home | KeyCode::Char('g') => {
                        self.help_scroll = 0;
                    }
                    KeyCode::End | KeyCode::Char('G') => {
                        self.help_scroll = self.help_scroll_max;
                    }
                    _ => {}
                },
                Event::Mouse(m) => match m.kind {
                    MouseEventKind::ScrollUp => {
                        self.help_scroll = self.help_scroll.saturating_sub(3);
                    }
                    MouseEventKind::ScrollDown => {
                        self.help_scroll = self
                            .help_scroll
                            .saturating_add(3)
                            .min(self.help_scroll_max);
                    }
                    _ => {}
                },
                _ => {}
            }
            return Ok(());
        }

        if self.show_theme {
            match evt {
                Event::Key(k) => match k.code {
                    KeyCode::F(2) | KeyCode::Esc => {
                        self.show_theme = false;
                        self.theme_scroll = 0;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.theme_scroll = self
                            .theme_scroll
                            .saturating_add(1)
                            .min(self.theme_scroll_max);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.theme_scroll = self.theme_scroll.saturating_sub(1);
                    }
                    KeyCode::PageDown => {
                        self.theme_scroll = self
                            .theme_scroll
                            .saturating_add(10)
                            .min(self.theme_scroll_max);
                    }
                    KeyCode::PageUp => {
                        self.theme_scroll = self.theme_scroll.saturating_sub(10);
                    }
                    KeyCode::Home | KeyCode::Char('g') => {
                        self.theme_scroll = 0;
                    }
                    KeyCode::End | KeyCode::Char('G') => {
                        self.theme_scroll = self.theme_scroll_max;
                    }
                    _ => {}
                },
                Event::Mouse(m) => match m.kind {
                    MouseEventKind::ScrollUp => {
                        self.theme_scroll = self.theme_scroll.saturating_sub(3);
                    }
                    MouseEventKind::ScrollDown => {
                        self.theme_scroll = self
                            .theme_scroll
                            .saturating_add(3)
                            .min(self.theme_scroll_max);
                    }
                    _ => {}
                },
                _ => {}
            }
            return Ok(());
        }

        if let Some(result) = self.handle_modal_event(evt.clone()) {
            match result {
                ModalResult::Continue | ModalResult::CloseSuccess | ModalResult::CloseCancel => {
                    return Ok(());
                }
            }
        }

        match evt {
            Event::Key(k) => match k.code {
                KeyCode::F(1) => self.show_help = true,
                KeyCode::F(2) => self.show_theme = true,
                KeyCode::F(3) => self.open_validation_modal(),
                KeyCode::Char('s') if !k.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.begin_save();
                }
                KeyCode::Char('q') if k.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.request_quit();
                }
                _ => {}
            },
            Event::Mouse(_) => {}
            _ => {}
        }

        Ok(())
    }
}
