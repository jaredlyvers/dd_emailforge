# dd_emailforge

A terminal-UI email template builder. Single Rust binary; edit a template in a TUI, export strict MJML, compile HTML with official MJML 5.

This slice is family chrome only (`tui`). The JSON model, MJML emitter, and live preview land in later PRs. See `docs/DESIGN.md`.

## Install

```bash
./install.sh
```

Builds release, drops the binary at `$HOME/.local/bin/dd_emailforge`, installs the default theme at `$HOME/.config/ldnddev/dd_emailforge_theme.yml` (only when no theme is already there), and creates `~/.config/ldnddev/dd_emailforge/templates/`. Override paths via `PREFIX`, `BIN_DIR`, or `CONFIG_DIR`.

```bash
cargo build --release
```

## Usage

```bash
dd_emailforge tui
dd_emailforge tui path/to/template.json
dd_emailforge tui path/to/folder/
dd_emailforge validate path/to/folder/
dd_emailforge show path/to/template.json
```

**TUI:** `F1` help · `F2` theme · `F3` validate · `s` save · `Ctrl+Q` quit (confirms if unsaved).

## Theme

Customize colors by writing the first of these that exists:
- `./dd_emailforge_theme.yml`
- `$XDG_CONFIG_HOME/ldnddev/dd_emailforge_theme.yml` (if `XDG_CONFIG_HOME` is set)
- `~/.config/ldnddev/dd_emailforge_theme.yml`

Schema: `LDNDDEV_TUI_VISUAL_STANDARD.md`. Every theme file must declare `version: 1`.

## Tests

```bash
cargo test -q
```

## License

MIT License.
