//! Minimal PR 2 modals: load error, save path, confirm quit, validation errors.

mod events;
mod paint;

#[derive(Debug)]
pub(in crate::tui) enum Modal {
    LoadError { message: String },
    SavePrompt { path: String },
    ConfirmPrompt {
        message: String,
        on_confirm: ConfirmKind,
    },
    ValidationErrors {
        errors: Vec<String>,
        scroll_offset: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tui) enum ConfirmKind {
    QuitUnsaved,
}

#[derive(Debug)]
pub(in crate::tui) enum ModalResult {
    Continue,
    CloseSuccess,
    CloseCancel,
}
