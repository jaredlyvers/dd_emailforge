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
    MjmlMissing { searched: Vec<String> },
    MjmlCompileError { stderr: String, scroll: u16 },
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
