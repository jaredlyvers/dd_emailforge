//! TUI export (Shift+E) and preview (`p`).
use std::time::{Duration, Instant};

use super::*;
use crate::emit::{EmitMode, write_mjml};
use crate::mjml::{self, MjmlError, gmail_clip_warning};
use crate::preview::{PreviewMeta, PreviewSession};
use crate::storage;
use crate::validate::validate_template_for_export;

impl App {
    pub(in crate::tui) fn begin_export(&mut self) {
        let Some(template) = self.template.clone() else {
            self.push_toast(
                ToastLevel::Warning,
                "No template open. Run: dd_emailforge init <dir>",
            );
            return;
        };
        let Some(path) = self.path.clone() else {
            self.push_toast(ToastLevel::Warning, "Save the template before exporting.");
            return;
        };
        let root = storage::template_root(&path);
        let report = validate_template_for_export(&template, Some(&root));
        if !report.errors.is_empty() {
            self.modal = Some(Modal::ValidationErrors {
                errors: report.errors,
                scroll_offset: 0,
            });
            return;
        }
        for w in report.warnings {
            self.push_toast(ToastLevel::Warning, w);
        }
        let bin = match mjml::discover_mjml(&root) {
            Ok(p) => p,
            Err(MjmlError::NotFound { searched }) => {
                self.modal = Some(Modal::MjmlMissing { searched });
                return;
            }
            Err(e) => {
                self.modal = Some(Modal::MjmlCompileError {
                    stderr: e.to_string(),
                    scroll: 0,
                });
                return;
            }
        };
        let mjml_path = root.join("template.mjml");
        let html_path = root.join("template.html");
        if let Err(e) = write_mjml(&template, &mjml_path, EmitMode::Export) {
            self.push_toast(ToastLevel::Error, format!("Failed to write MJML: {e}"));
            return;
        }
        match mjml::compile_one_shot_captured(&bin, &root, &mjml_path, &html_path) {
            Ok(result) => {
                self.push_toast(
                    ToastLevel::Success,
                    format!("Exported {}", html_path.display()),
                );
                if let Some(w) = gmail_clip_warning(result.html_bytes) {
                    self.push_toast(ToastLevel::Warning, w);
                }
            }
            Err(MjmlError::NotFound { searched }) => {
                self.modal = Some(Modal::MjmlMissing { searched });
            }
            Err(MjmlError::Compile { stderr }) => {
                self.modal = Some(Modal::MjmlCompileError { stderr, scroll: 0 });
            }
            Err(e) => {
                self.modal = Some(Modal::MjmlCompileError {
                    stderr: e.to_string(),
                    scroll: 0,
                });
            }
        }
    }

    pub(in crate::tui) fn begin_preview(&mut self) {
        let Some(template) = self.template.clone() else {
            self.push_toast(
                ToastLevel::Warning,
                "No template open. Run: dd_emailforge init <dir>",
            );
            return;
        };
        let Some(path) = self.path.clone() else {
            self.push_toast(ToastLevel::Warning, "Save the template before previewing.");
            return;
        };
        let root = storage::template_root(&path);
        let report = crate::validate::validate_template_with_root(&template, Some(&root));
        if !report.errors.is_empty() {
            self.modal = Some(Modal::ValidationErrors {
                errors: report.errors,
                scroll_offset: 0,
            });
            return;
        }
        for w in report.warnings {
            self.push_toast(ToastLevel::Warning, w);
        }

        if let Some(session) = self.preview.as_ref() {
            if let Ok(mut meta) = session.meta.lock() {
                meta.subject = template.subject.clone();
                meta.preheader = template.preheader.clone();
            }
            let origin = format!("http://127.0.0.1:{}", session.port);
            let mjml_path = root.join("template.mjml");
            let _ = write_mjml(&template, &mjml_path, EmitMode::Preview { origin });
            let url = session.url();
            match super::util::open_in_browser(&url) {
                Ok(()) => self.push_toast(ToastLevel::Success, format!("Preview {url}")),
                Err(e) => self.push_toast(ToastLevel::Warning, format!("Open browser failed: {e}")),
            }
            return;
        }

        let bin = match mjml::discover_mjml(&root) {
            Ok(p) => p,
            Err(MjmlError::NotFound { searched }) => {
                self.modal = Some(Modal::MjmlMissing { searched });
                return;
            }
            Err(e) => {
                self.modal = Some(Modal::MjmlCompileError {
                    stderr: e.to_string(),
                    scroll: 0,
                });
                return;
            }
        };

        let preview_dir = root.join(".preview");
        if let Err(e) = std::fs::create_dir_all(&preview_dir) {
            self.push_toast(
                ToastLevel::Error,
                format!("Could not create .preview/: {e}"),
            );
            return;
        }
        let compiled = preview_dir.join("template.html");
        let meta = PreviewMeta {
            subject: template.subject.clone(),
            preheader: template.preheader.clone(),
        };
        let session = match PreviewSession::start_tui(root.clone(), compiled.clone(), meta, None) {
            Ok(s) => s,
            Err(e) => {
                self.push_toast(ToastLevel::Error, format!("Preview server failed: {e}"));
                return;
            }
        };
        let origin = format!("http://127.0.0.1:{}", session.port);
        let mjml_path = root.join("template.mjml");
        if let Err(e) = write_mjml(&template, &mjml_path, EmitMode::Preview { origin }) {
            self.push_toast(ToastLevel::Error, format!("Failed to write MJML: {e}"));
            return;
        }
        let watch = match mjml::MjmlWatch::spawn(&bin, &root, &mjml_path, &compiled) {
            Ok(w) => w,
            Err(e) => {
                self.modal = Some(Modal::MjmlCompileError {
                    stderr: e.to_string(),
                    scroll: 0,
                });
                return;
            }
        };
        let mut session = session;
        session.watch = Some(watch);

        let ready = wait_for_file(&compiled, Duration::from_secs(2));
        let url = session.url();
        self.preview = Some(session);
        if !ready {
            self.push_toast(ToastLevel::Info, "waiting for mjml");
        }
        match super::util::open_in_browser(&url) {
            Ok(()) => self.push_toast(ToastLevel::Success, format!("Preview {url}")),
            Err(e) => self.push_toast(ToastLevel::Warning, format!("Open browser failed: {e}")),
        }
    }

    pub(in crate::tui) fn drain_watch_errors(&mut self) {
        let mut lines = Vec::new();
        if let Some(session) = self.preview.as_ref() {
            if let Some(watch) = session.watch.as_ref() {
                while let Ok(line) = watch.errors.try_recv() {
                    lines.push(line);
                }
            }
        }
        if let Some(line) = lines.pop() {
            self.push_toast(ToastLevel::Warning, line);
        }
    }

    pub(in crate::tui) fn write_mjml_sidecar(&self) {
        let Some(template) = self.template.as_ref() else {
            return;
        };
        let Some(path) = self.path.as_ref() else {
            return;
        };
        let root = storage::template_root(path);
        let mjml_path = root.join("template.mjml");
        let mode = if let Some(session) = self.preview.as_ref() {
            EmitMode::Preview {
                origin: format!("http://127.0.0.1:{}", session.port),
            }
        } else {
            EmitMode::Export
        };
        let _ = write_mjml(template, &mjml_path, mode);
        if let Some(session) = self.preview.as_ref() {
            if let Ok(mut meta) = session.meta.lock() {
                meta.subject = template.subject.clone();
                meta.preheader = template.preheader.clone();
            }
        }
    }
}

fn wait_for_file(path: &std::path::Path, timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if path.is_file() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
}
