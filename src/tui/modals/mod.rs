//! Modals: load/save/confirm/validate, mjml, and FormEdit.

mod events;
mod form_edit;
mod paint;
mod pickers;

pub(in crate::tui) use form_edit::DrillFrame;

#[derive(Debug)]
pub(in crate::tui) enum Modal {
    LoadError {
        message: String,
    },
    SavePrompt {
        path: String,
    },
    ConfirmPrompt {
        message: String,
        on_confirm: ConfirmKind,
    },
    ValidationErrors {
        errors: Vec<String>,
        scroll_offset: usize,
    },
    MjmlMissing {
        searched: Vec<String>,
    },
    MjmlCompileError {
        stderr: String,
        scroll: u16,
    },
    FormEdit {
        state: crate::tui::editform::EditFormState,
        cursor: crate::tui::tree::TreeId,
        cursor_pos: usize,
        scroll_offset: u16,
        drill_stack: Vec<DrillFrame>,
    },
    ComponentPicker {
        query: String,
        selected: usize,
    },
    ImagePicker {
        state: ImagePickerState,
    },
}

#[derive(Debug, Clone)]
pub(in crate::tui) struct ImagePickerState {
    pub(in crate::tui) root: std::path::PathBuf,
    pub(in crate::tui) cwd: std::path::PathBuf,
    pub(in crate::tui) filter: String,
    pub(in crate::tui) selected: usize,
    pub(in crate::tui) binding: ImagePickBinding,
}

#[derive(Debug, Clone)]
pub(in crate::tui) enum ImagePickBinding {
    FormEditField { field_id: String },
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
