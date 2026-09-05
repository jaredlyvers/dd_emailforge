//! FormEdit data model: one `EditForm` per node kind, live `EditFormState`.
use std::collections::HashMap;

mod forms;
pub use forms::*;

#[derive(Debug)]
pub struct EditForm {
    pub title: &'static str,
    pub fields: &'static [FormField],
}

#[derive(Debug)]
pub struct FormField {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: FieldKind,
    #[allow(dead_code)]
    pub required: bool,
    pub visible_when: Option<FieldPredicate>,
    pub hint: Option<&'static str>,
    pub placeholder: Option<&'static str>,
}

#[derive(Debug)]
pub enum FieldKind {
    Text {
        default: &'static str,
    },
    Textarea {
        rows: u16,
        default: &'static str,
    },
    Url {
        default: &'static str,
    },
    Enum {
        options: &'static [&'static str],
        default: &'static str,
    },
    SubForm {
        template: &'static EditForm,
        min_items: usize,
        summary_field_id: &'static str,
    },
}

#[derive(Debug)]
pub enum FieldPredicate {
    FieldEquals {
        other_id: &'static str,
        value: &'static str,
    },
}

#[derive(Debug, Clone)]
pub struct EditFormState {
    pub form: &'static EditForm,
    pub values: HashMap<String, String>,
    pub sub_state: HashMap<String, Vec<EditFormState>>,
    pub selected_sub_item: HashMap<String, usize>,
    pub focused_field: usize,
    pub textarea_cursor: (usize, usize),
}

impl EditFormState {
    pub fn new(form: &'static EditForm) -> Self {
        let mut values = HashMap::new();
        let mut sub_state = HashMap::new();
        let mut selected_sub_item = HashMap::new();
        for field in form.fields {
            match &field.kind {
                FieldKind::Text { default } | FieldKind::Url { default } => {
                    values.insert(field.id.to_string(), default.to_string());
                }
                FieldKind::Textarea { default, .. } => {
                    values.insert(field.id.to_string(), default.to_string());
                }
                FieldKind::Enum { default, .. } => {
                    values.insert(field.id.to_string(), default.to_string());
                }
                FieldKind::SubForm { .. } => {
                    sub_state.insert(field.id.to_string(), Vec::new());
                    selected_sub_item.insert(field.id.to_string(), 0);
                }
            }
        }
        Self {
            form,
            values,
            sub_state,
            selected_sub_item,
            focused_field: 0,
            textarea_cursor: (0, 0),
        }
    }

    pub fn new_sub_item(&self, subform_field_id: &str) -> Option<EditFormState> {
        for field in self.form.fields {
            if field.id == subform_field_id {
                if let FieldKind::SubForm { template, .. } = &field.kind {
                    return Some(EditFormState::new(*template));
                }
            }
        }
        None
    }

    pub fn get(&self, id: &str) -> &str {
        self.values.get(id).map(String::as_str).unwrap_or("")
    }

    pub fn set(&mut self, id: &str, value: impl Into<String>) {
        self.values.insert(id.to_string(), value.into());
    }

    pub fn field_visible(&self, field: &FormField) -> bool {
        match &field.visible_when {
            None => true,
            Some(FieldPredicate::FieldEquals { other_id, value }) => self.get(other_id) == *value,
        }
    }

    pub fn visible_field_indices(&self) -> Vec<usize> {
        self.form
            .fields
            .iter()
            .enumerate()
            .filter_map(|(idx, field)| self.field_visible(field).then_some(idx))
            .collect()
    }

    pub fn focus_next(&mut self) {
        let visible = self.visible_field_indices();
        if visible.is_empty() {
            return;
        }
        let current_pos = visible
            .iter()
            .position(|&i| i == self.focused_field)
            .unwrap_or(0);
        self.focused_field = visible[(current_pos + 1) % visible.len()];
        self.textarea_cursor = (0, 0);
    }

    pub fn focus_prev(&mut self) {
        let visible = self.visible_field_indices();
        if visible.is_empty() {
            return;
        }
        let current_pos = visible
            .iter()
            .position(|&i| i == self.focused_field)
            .unwrap_or(0);
        let prev_pos = if current_pos == 0 {
            visible.len() - 1
        } else {
            current_pos - 1
        };
        self.focused_field = visible[prev_pos];
        self.textarea_cursor = (0, 0);
    }

    pub fn focused(&self) -> Option<&FormField> {
        self.form.fields.get(self.focused_field)
    }

    pub fn cycle_enum(&mut self, forward: bool) {
        let Some(field) = self.focused() else {
            return;
        };
        let FieldKind::Enum { options, .. } = &field.kind else {
            return;
        };
        if options.is_empty() {
            return;
        }
        let current = self.get(field.id).to_string();
        let idx = options
            .iter()
            .position(|opt| *opt == current.as_str())
            .unwrap_or(0);
        let next = if forward {
            (idx + 1) % options.len()
        } else if idx == 0 {
            options.len() - 1
        } else {
            idx - 1
        };
        self.set(field.id, options[next].to_string());
    }

    #[cfg(test)]
    pub fn field_index(&self, id: &str) -> Option<usize> {
        self.form.fields.iter().position(|f| f.id == id)
    }
}
