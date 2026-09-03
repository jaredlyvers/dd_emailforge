use super::*;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::backend::TestBackend;
use ratatui::Terminal;

fn send_key(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    app.handle_event(Event::Key(KeyEvent::new(code, modifiers)))
        .expect("key event should be handled");
}

fn chrome_app() -> App {
    App::new(
        AppTheme::default(),
        "default".to_string(),
        None,
        None,
        None,
    )
}

fn app_with_template() -> App {
    App::new(
        AppTheme::default(),
        "default".to_string(),
        None,
        Some(crate::model::Template::minimal()),
        None,
    )
}

#[test]
fn f1_opens_and_closes() {
    let mut app = chrome_app();
    send_key(&mut app, KeyCode::F(1), KeyModifiers::NONE);
    assert!(app.show_help);
    send_key(&mut app, KeyCode::F(1), KeyModifiers::NONE);
    assert!(!app.show_help);
}

#[test]
fn f2_opens_and_closes_with_f2_and_esc() {
    let mut app = chrome_app();
    send_key(&mut app, KeyCode::F(2), KeyModifiers::NONE);
    assert!(app.show_theme);
    assert_eq!(app.theme_scroll, 0);
    send_key(&mut app, KeyCode::Esc, KeyModifiers::NONE);
    assert!(!app.show_theme);
}

#[test]
fn f2_scroll_keys_update_theme_scroll() {
    let mut app = chrome_app();
    send_key(&mut app, KeyCode::F(2), KeyModifiers::NONE);
    app.theme_scroll_max = 5;
    send_key(&mut app, KeyCode::Down, KeyModifiers::NONE);
    assert_eq!(app.theme_scroll, 1);
    send_key(&mut app, KeyCode::Char('g'), KeyModifiers::NONE);
    assert_eq!(app.theme_scroll, 0);
}

#[test]
fn ctrl_q_quits() {
    let mut app = chrome_app();
    send_key(&mut app, KeyCode::Char('q'), KeyModifiers::CONTROL);
    assert!(app.should_quit);
}

#[test]
fn q_without_modifiers_does_not_quit() {
    let mut app = chrome_app();
    send_key(&mut app, KeyCode::Char('q'), KeyModifiers::NONE);
    assert!(!app.should_quit);
}

#[test]
fn footer_hint_starts_with_f1_help_and_ctrl_q() {
    let app = chrome_app();
    for width in [40u16, 80, 160] {
        let hint = app.footer_hint(width);
        assert!(
            hint.starts_with("F1:Help") || hint.starts_with("F1: Help"),
            "width {width}: {hint}"
        );
        assert!(
            hint.contains("C-q:Quit") || hint.contains("Ctrl+Q"),
            "width {width}: {hint}"
        );
        assert!(
            !hint.contains("q:Quit") || hint.contains("C-q:Quit") || hint.contains("Ctrl+Q:Quit") || hint.contains("Ctrl+Q: Quit"),
            "bare q:Quit must not appear, width {width}: {hint}"
        );
        let stripped = hint.replace("C-q:Quit", "").replace("Ctrl+Q: Quit", "").replace("Ctrl+Q:Quit", "");
        assert!(!stripped.contains("q:Quit"), "width {width}: {hint}");
        assert!(!hint.to_lowercase().contains("error"), "{hint}");
        assert!(!hint.to_lowercase().contains("warning"), "{hint}");
    }
}

#[test]
fn footer_hint_wide_mentions_mouse() {
    let app = chrome_app();
    let hint = app.footer_hint(160);
    assert!(hint.contains("mouse"), "{hint}");
}

#[test]
fn draw_smoke_header_is_three_rows() {
    let mut app = chrome_app();
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal
        .draw(|f| {
            app.draw(f);
            assert_eq!(f.area().height, 24);
        })
        .expect("draw");
}

#[test]
fn draw_help_modal_does_not_panic() {
    let mut app = chrome_app();
    app.show_help = true;
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal.draw(|f| app.draw(f)).expect("draw help");
}

#[test]
fn all_toast_levels_render() {
    let mut app = chrome_app();
    app.push_toast(ToastLevel::Success, "ok");
    app.push_toast(ToastLevel::Info, "info");
    app.push_toast(ToastLevel::Warning, "warn");
    app.push_toast(ToastLevel::Error, "err");
    assert_eq!(app.toasts.len(), 4);
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal.draw(|f| app.draw(f)).expect("draw toasts");
}

#[test]
fn warning_toast_from_theme_status_does_not_panic_draw() {
    let mut app = App::new(
        AppTheme::default(),
        "default".to_string(),
        Some("theme 'foo.yml' declares version 99 (expected 1); using built-in defaults".into()),
        None,
        None,
    );
    app.push_toast(
        ToastLevel::Warning,
        app.theme_status.clone().unwrap(),
    );
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal.draw(|f| app.draw(f)).expect("draw toast");
}

#[test]
fn p_without_template_toasts_warning() {
    let mut app = chrome_app();
    send_key(&mut app, KeyCode::Char('p'), KeyModifiers::NONE);
    assert!(
        app.toasts
            .iter()
            .any(|t| t.message.contains("No template open"))
    );
}

#[test]
fn shift_e_without_template_toasts_warning() {
    let mut app = chrome_app();
    send_key(&mut app, KeyCode::Char('E'), KeyModifiers::SHIFT);
    assert!(
        app.toasts
            .iter()
            .any(|t| t.message.contains("No template open"))
    );
}

#[test]
fn f3_without_template_toasts_warning() {
    let mut app = chrome_app();
    send_key(&mut app, KeyCode::F(3), KeyModifiers::NONE);
    assert!(app.modal.is_none());
    assert!(
        app.toasts
            .iter()
            .any(|t| t.message.contains("No template open"))
    );
}

#[test]
fn f3_on_minimal_template_passes() {
    let mut app = app_with_template();
    send_key(&mut app, KeyCode::F(3), KeyModifiers::NONE);
    assert!(app.modal.is_none());
    assert!(
        app.toasts
            .iter()
            .any(|t| t.message.contains("Validation passed"))
    );
}

#[test]
fn f3_on_invalid_template_opens_modal() {
    let mut t = crate::model::Template::minimal();
    t.name.clear();
    let mut app = App::new(
        AppTheme::default(),
        "default".to_string(),
        None,
        Some(t),
        None,
    );
    send_key(&mut app, KeyCode::F(3), KeyModifiers::NONE);
    match &app.modal {
        Some(Modal::ValidationErrors { errors, .. }) => {
            assert!(errors.iter().any(|e| e.contains("name is empty")));
        }
        other => panic!("expected ValidationErrors, got {other:?}"),
    }
}

#[test]
fn dirty_ctrl_q_opens_confirm() {
    let mut app = app_with_template();
    if let Some(t) = app.template.as_mut() {
        t.subject = "changed".to_string();
    }
    app.mark_dirty_if_changed();
    assert!(app.dirty);
    send_key(&mut app, KeyCode::Char('q'), KeyModifiers::CONTROL);
    assert!(!app.should_quit);
    match &app.modal {
        Some(Modal::ConfirmPrompt { .. }) => {}
        other => panic!("expected ConfirmPrompt, got {other:?}"),
    }
    send_key(&mut app, KeyCode::Char('y'), KeyModifiers::NONE);
    assert!(app.should_quit);
}

#[test]
fn save_without_path_opens_prompt() {
    let mut app = app_with_template();
    send_key(&mut app, KeyCode::Char('s'), KeyModifiers::NONE);
    match &app.modal {
        Some(Modal::SavePrompt { path }) => assert_eq!(path, "template.json"),
        other => panic!("expected SavePrompt, got {other:?}"),
    }
}

#[test]
fn footer_marks_dirty() {
    let mut app = app_with_template();
    app.dirty = true;
    let hint = app.footer_hint(80);
    assert!(hint.starts_with("*  "), "{hint}");
    assert!(hint.contains("F3"), "{hint}");
}
