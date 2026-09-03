# Tutorial screenshots

User-facing walkthrough: [`index.html`](index.html).

## Recapture

From the **repo root**:

```bash
./docs/tutorial/capture.sh
```

Needs `cargo` and Chromium/Chrome (`chromium`, `google-chrome`, … or `CHROME=/path/to/chrome`).

```bash
EMAILFORGE_TUTORIAL_SHOTS=/tmp/shots CHROME=/usr/bin/chromium ./docs/tutorial/capture.sh
```

Pipeline:

1. Ignored test `tui::tutorial_shots::capture_tutorial_frames` draws the real TUI (`App::draw` + `TestBackend`) and writes `_frame-<name>.html`.
2. Headless Chromium screenshots each frame to `images/<name>.png`.
3. Temporary HTML is deleted.

Do not paint over the PNGs. After UI changes, recapture and commit.

## Shot list

| PNG | Built in `src/tui/tutorial_shots.rs` |
|---|---|
| `tui-empty.png` | chrome, no template, init toast |
| `tui-welcome.png` | welcome starter, BODY selected |
| `tui-insert.png` | `/` on `mj-column` |
| `tui-formedit.png` | Enter on `email-header` |
| `tui-help.png` | F1 |
| `tui-selected.png` | first `mj-text` selected |

Header tagline is pinned to `600 pixels wide. Infinite opinions.` so captures are deterministic.

## Add a shot

1. Add a helper + `write_frame(dir, "tui-foo", &mut …)` in `capture_tutorial_frames`.
2. Mention the file in `index.html` and the table above.
3. Run `./docs/tutorial/capture.sh`.
4. Commit the new PNG.
