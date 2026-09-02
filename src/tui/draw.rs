//! Frame layout: header, empty body, footer, F1/F2 overlays, toasts.
use super::help::{build_help_text, build_theme_text, count_wrapped_lines};
use super::*;

impl App {
    pub(super) fn draw(&mut self, frame: &mut ratatui::Frame) {
        self.prune_toasts();

        frame.render_widget(Block::default().style(self.theme.app_shell), frame.area());

        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let header_block = Block::default()
            .title("dd_emailforge")
            .borders(Borders::ALL)
            .border_style(self.theme.active_border)
            .style(self.theme.app_shell)
            .title_style(
                Style::default()
                    .fg(self.theme.text_active_focus)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(header_block.clone(), root[0]);

        if root[0].height >= 3 {
            let inner = header_block.inner(root[0]);
            let quote = Paragraph::new(self.header_copy.as_str()).style(
                Style::default()
                    .fg(self.theme.text_primary)
                    .bg(self.theme.base_background),
            );
            frame.render_widget(quote, inner);
        }

        self.body_area = root[1];
        let body = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border_default))
            .style(
                Style::default()
                    .fg(self.theme.text_primary)
                    .bg(self.theme.body_background),
            );
        frame.render_widget(body, root[1]);

        let footer_text = self.footer_hint(root[2].width);
        let footer = Paragraph::new(footer_text).style(self.theme.app_shell);
        frame.render_widget(footer, root[2]);

        self.render_modal(frame);

        if self.show_help {
            self.render_scroll_modal(
                frame,
                "Key & Mouse bindings (F1 / Esc to close, j/k or arrows to scroll)",
                true,
            );
        }

        if self.show_theme {
            self.render_scroll_modal(
                frame,
                "Theme (F2 / Esc to close, j/k or arrows to scroll)",
                false,
            );
        }

        self.render_toasts(frame, frame.area());
    }

    fn render_scroll_modal(&mut self, frame: &mut ratatui::Frame, title: &str, is_help: bool) {
        let area = centered_rect(80, 80, frame.area());
        frame.render_widget(Clear, area);
        let block = Block::default()
            .title(title.to_string())
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .fg(self.theme.modal_text)
                    .bg(self.theme.modal_background),
            )
            .border_style(Style::default().fg(self.theme.border_active))
            .title_style(
                Style::default()
                    .fg(self.theme.modal_header)
                    .add_modifier(Modifier::BOLD),
            );
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let scrollbar_width: u16 = 1;
        let body_w = inner.width.saturating_sub(scrollbar_width + 1);
        let body_area = Rect {
            x: inner.x,
            y: inner.y,
            width: body_w,
            height: inner.height,
        };

        let text = if is_help {
            build_help_text(&self.theme, body_w as usize)
        } else {
            build_theme_text(
                &self.theme,
                &self.theme_source,
                &self.theme_status,
                body_w as usize,
            )
        };
        let wrapped_total = count_wrapped_lines(&text, body_w as usize);
        let visible = inner.height as usize;
        let max_scroll = wrapped_total.saturating_sub(visible) as u16;
        if is_help {
            self.help_scroll_max = max_scroll;
            if self.help_scroll > max_scroll {
                self.help_scroll = max_scroll;
            }
        } else {
            self.theme_scroll_max = max_scroll;
            if self.theme_scroll > max_scroll {
                self.theme_scroll = max_scroll;
            }
        }
        let scroll = if is_help {
            self.help_scroll
        } else {
            self.theme_scroll
        };

        let body = Paragraph::new(text)
            .style(
                Style::default()
                    .fg(self.theme.modal_text)
                    .bg(self.theme.modal_background),
            )
            .wrap(Wrap { trim: false })
            .scroll((scroll, 0));
        frame.render_widget(body, body_area);

        if (wrapped_total as u16) > inner.height {
            let track_x = inner.x + inner.width.saturating_sub(1);
            for row in 0..inner.height {
                let cell = Paragraph::new("│").style(
                    Style::default()
                        .fg(self.theme.scrollbar)
                        .bg(self.theme.modal_background),
                );
                frame.render_widget(
                    cell,
                    Rect {
                        x: track_x,
                        y: inner.y + row,
                        width: 1,
                        height: 1,
                    },
                );
            }
            let total_h = inner.height as usize;
            let thumb_h = ((total_h * total_h) / wrapped_total.max(1)).max(1);
            let scroll_range = wrapped_total.saturating_sub(total_h).max(1);
            let thumb_top =
                ((scroll as usize) * total_h.saturating_sub(thumb_h)) / scroll_range;
            for i in 0..thumb_h {
                let cell = Paragraph::new("█").style(
                    Style::default()
                        .fg(self.theme.scrollbar_hover)
                        .bg(self.theme.modal_background),
                );
                frame.render_widget(
                    cell,
                    Rect {
                        x: track_x,
                        y: inner.y + (thumb_top + i) as u16,
                        width: 1,
                        height: 1,
                    },
                );
            }
        }
    }
}

pub(super) fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
