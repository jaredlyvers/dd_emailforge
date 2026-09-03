//! Component and image picker events.
use super::super::util::{filter_entries, list_dir_entries};
use super::super::*;
use super::{ImagePickBinding, ImagePickerState};

impl App {
    pub(in crate::tui) fn handle_component_picker_event(
        &mut self,
        key: event::KeyEvent,
    ) -> Option<ModalResult> {
        let (mut query, mut selected) =
            if let Some(Modal::ComponentPicker { query, selected }) = self.modal.take() {
                (query, selected)
            } else {
                return Some(ModalResult::CloseCancel);
            };
        let class = crate::tui::component_kind::classify(
            self.template.as_ref(),
            &self
                .selected_tree_id()
                .unwrap_or(crate::tui::tree::TreeId::Body),
        );
        let rows = crate::tui::component_kind::picker_rows(class, &query);

        match key.code {
            KeyCode::Esc => {
                self.push_toast(ToastLevel::Info, "Component picker cancelled.");
                return Some(ModalResult::CloseCancel);
            }
            KeyCode::Up => {
                selected = crate::tui::component_kind::move_picker_selection(&rows, selected, -1);
            }
            KeyCode::Down => {
                selected = crate::tui::component_kind::move_picker_selection(&rows, selected, 1);
            }
            KeyCode::Char('k') if query.is_empty() => {
                selected = crate::tui::component_kind::move_picker_selection(&rows, selected, -1);
            }
            KeyCode::Char('j') if query.is_empty() => {
                selected = crate::tui::component_kind::move_picker_selection(&rows, selected, 1);
            }
            KeyCode::Backspace => {
                query.pop();
                let rows = crate::tui::component_kind::picker_rows(class, &query);
                selected = crate::tui::component_kind::first_kind_index(&rows);
            }
            KeyCode::Enter => match rows.get(selected) {
                Some(crate::tui::component_kind::PickerRow::Kind(kind)) => {
                    self.insert_kind(*kind);
                    return Some(ModalResult::CloseSuccess);
                }
                _ => {
                    self.push_toast(ToastLevel::Warning, "No component selected.");
                    self.modal = Some(Modal::ComponentPicker { query, selected });
                    return Some(ModalResult::Continue);
                }
            },
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                query.push(c);
                let rows = crate::tui::component_kind::picker_rows(class, &query);
                selected = crate::tui::component_kind::first_kind_index(&rows);
            }
            _ => {}
        }
        self.modal = Some(Modal::ComponentPicker { query, selected });
        Some(ModalResult::Continue)
    }

    pub(in crate::tui) fn handle_image_picker_event(
        &mut self,
        key: event::KeyEvent,
    ) -> Option<ModalResult> {
        let Some(Modal::ImagePicker { state }) = self.modal.as_mut() else {
            return Some(ModalResult::CloseCancel);
        };
        match key.code {
            KeyCode::Esc => {
                self.modal = self.paused_form_edit_modal.take();
                self.push_toast(ToastLevel::Info, "Image pick cancelled.");
                Some(ModalResult::CloseCancel)
            }
            KeyCode::Up => {
                state.selected = state.selected.saturating_sub(1);
                Some(ModalResult::Continue)
            }
            KeyCode::Char('k') if state.filter.is_empty() => {
                state.selected = state.selected.saturating_sub(1);
                Some(ModalResult::Continue)
            }
            KeyCode::Down => {
                let entries = list_dir_entries(&state.cwd);
                let filtered = filter_entries(&entries, &state.filter);
                if !filtered.is_empty() {
                    state.selected = (state.selected + 1).min(filtered.len() - 1);
                }
                Some(ModalResult::Continue)
            }
            KeyCode::Char('j') if state.filter.is_empty() => {
                let entries = list_dir_entries(&state.cwd);
                let filtered = filter_entries(&entries, &state.filter);
                if !filtered.is_empty() {
                    state.selected = (state.selected + 1).min(filtered.len() - 1);
                }
                Some(ModalResult::Continue)
            }
            KeyCode::Left => {
                if state.cwd != state.root {
                    if let Some(parent) = state.cwd.parent() {
                        if parent.starts_with(&state.root) || parent == state.root {
                            state.cwd = parent.to_path_buf();
                            state.filter.clear();
                            state.selected = 0;
                        }
                    }
                }
                Some(ModalResult::Continue)
            }
            KeyCode::Char('h') if state.filter.is_empty() => {
                if state.cwd != state.root {
                    if let Some(parent) = state.cwd.parent() {
                        if parent.starts_with(&state.root) || parent == state.root {
                            state.cwd = parent.to_path_buf();
                            state.filter.clear();
                            state.selected = 0;
                        }
                    }
                }
                Some(ModalResult::Continue)
            }
            KeyCode::Right | KeyCode::Enter => {
                self.image_picker_descend_or_pick();
                Some(ModalResult::Continue)
            }
            KeyCode::Char('l') if state.filter.is_empty() => {
                self.image_picker_descend_or_pick();
                Some(ModalResult::Continue)
            }
            KeyCode::Backspace => {
                state.filter.pop();
                state.selected = 0;
                Some(ModalResult::Continue)
            }
            KeyCode::Char(c)
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && (c.is_alphanumeric() || c == '-' || c == '_' || c == '.') =>
            {
                state.filter.push(c);
                state.selected = 0;
                Some(ModalResult::Continue)
            }
            _ => Some(ModalResult::Continue),
        }
    }

    pub(in crate::tui) fn image_picker_descend_or_pick(&mut self) {
        let (cwd, root, selected_name, is_dir, binding) = {
            let Some(Modal::ImagePicker { state }) = self.modal.as_ref() else {
                return;
            };
            let entries = list_dir_entries(&state.cwd);
            let filtered = filter_entries(&entries, &state.filter);
            let Some(entry) = filtered.get(state.selected) else {
                return;
            };
            (
                state.cwd.clone(),
                state.root.clone(),
                entry.name.clone(),
                entry.is_dir,
                state.binding.clone(),
            )
        };
        if is_dir {
            if let Some(Modal::ImagePicker { state }) = self.modal.as_mut() {
                state.cwd = cwd.join(&selected_name);
                state.filter.clear();
                state.selected = 0;
            }
            return;
        }
        let target_full = cwd.join(&selected_name);
        let rel = target_full
            .strip_prefix(&root)
            .unwrap_or(&target_full)
            .to_string_lossy()
            .replace('\\', "/");
        self.commit_image_pick(rel, binding);
    }

    pub(in crate::tui) fn commit_image_pick(&mut self, value: String, binding: ImagePickBinding) {
        match binding {
            ImagePickBinding::FormEditField { field_id } => {
                self.modal = self.paused_form_edit_modal.take();
                if let Some(Modal::FormEdit {
                    state, cursor_pos, ..
                }) = self.modal.as_mut()
                {
                    state.set(&field_id, value.clone());
                    *cursor_pos = state.get(&field_id).chars().count();
                    self.push_toast(ToastLevel::Success, format!("Picked image: {value}"));
                } else {
                    self.push_toast(
                        ToastLevel::Warning,
                        "Image pick lost: parent form modal closed.",
                    );
                }
            }
        }
    }

    pub(in crate::tui) fn open_image_picker(&mut self, field_id: String) {
        let Some(path) = self.path.as_ref() else {
            self.push_toast(
                ToastLevel::Warning,
                "Save the template first to pick local images.",
            );
            return;
        };
        let root = crate::storage::template_root(path);
        let cwd = root.join("images");
        let paused = self.modal.take();
        self.paused_form_edit_modal = paused;
        self.modal = Some(Modal::ImagePicker {
            state: ImagePickerState {
                root,
                cwd,
                filter: String::new(),
                selected: 0,
                binding: ImagePickBinding::FormEditField { field_id },
            },
        });
    }
}

pub(super) fn is_image_url_field(field_id: &str) -> bool {
    field_id.contains("src") || field_id == "background_url"
}
