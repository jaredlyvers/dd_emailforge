use super::super::*;

impl App {
    pub(in crate::tui) fn render_modal(&self, frame: &mut ratatui::Frame) {
        let Some(modal) = &self.modal else {
            return;
        };
        match modal {
            Modal::LoadError { message } => {
                self.render_message_modal(frame, " Error ", message, "Enter / Esc: dismiss");
            }
            Modal::SavePrompt { path } => {
                self.render_message_modal(
                    frame,
                    " Save template ",
                    &format!("Save file path:\n{path}"),
                    "Enter or Ctrl+S: save  |  Esc: cancel",
                );
            }
            Modal::ConfirmPrompt { message, .. } => {
                self.render_message_modal(
                    frame,
                    " Confirm ",
                    &format!("{message}\n\ny = confirm, n / Esc = cancel"),
                    "",
                );
            }
            Modal::ValidationErrors {
                errors,
                scroll_offset,
            } => {
                self.render_validation_errors_modal(frame, errors, *scroll_offset);
            }
        }
    }

    fn render_message_modal(
        &self,
        frame: &mut ratatui::Frame,
        title: &str,
        body: &str,
        footer: &str,
    ) {
        let area = centered_rect(70, 35, frame.area());
        frame.render_widget(Clear, area);
        let block = Block::default()
            .title(title.to_string())
            .borders(Borders::ALL)
            .style(Style::default().bg(self.theme.modal_background))
            .border_style(Style::default().fg(self.theme.border_active))
            .title_style(
                Style::default()
                    .fg(self.theme.modal_header)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(block.clone(), area);
        let inner = block.inner(area);
        let content = if footer.is_empty() {
            body.to_string()
        } else {
            format!("{body}\n\n{footer}")
        };
        frame.render_widget(
            Paragraph::new(content)
                .style(
                    Style::default()
                        .fg(self.theme.modal_text)
                        .bg(self.theme.modal_background),
                )
                .wrap(Wrap { trim: false }),
            inner,
        );
    }

    fn render_validation_errors_modal(
        &self,
        frame: &mut ratatui::Frame,
        errors: &[String],
        scroll_offset: usize,
    ) {
        let area = centered_rect(70, 60, frame.area());
        frame.render_widget(Clear, area);
        let block = Block::default()
            .title(format!(" Validation — {} error(s) ", errors.len()))
            .borders(Borders::ALL)
            .style(Style::default().bg(self.theme.modal_background))
            .border_style(Style::default().fg(self.theme.border_active))
            .title_style(
                Style::default()
                    .fg(self.theme.modal_header)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(block.clone(), area);
        let inner = block.inner(area);
        if inner.width < 4 || inner.height < 3 {
            return;
        }
        let padding_x: u16 = 2;
        let content_x = inner.x + padding_x;
        let content_w = inner.width.saturating_sub(padding_x * 2);
        let footer_height: u16 = 1;
        let list_height = inner.height.saturating_sub(footer_height);
        let lines: Vec<String> = errors.iter().map(|e| format!("- {e}")).collect();
        let visible: Vec<String> = lines
            .iter()
            .skip(scroll_offset)
            .take(list_height as usize)
            .cloned()
            .collect();
        frame.render_widget(
            Paragraph::new(visible.join("\n")).style(
                Style::default()
                    .fg(self.theme.modal_text)
                    .bg(self.theme.modal_background),
            ),
            Rect {
                x: content_x,
                y: inner.y,
                width: content_w,
                height: list_height,
            },
        );
        frame.render_widget(
            Paragraph::new("Enter / Esc: close  |  j/k: scroll").style(
                Style::default()
                    .fg(self.theme.modal_labels)
                    .bg(self.theme.modal_background),
            ),
            Rect {
                x: content_x,
                y: inner.y + list_height,
                width: content_w,
                height: 1,
            },
        );
    }
}
