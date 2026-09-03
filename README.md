# dd_emailforge

Terminal-UI email template builder. Edit a typed `template.json` in a Ratatui TUI, export strict MJML, compile HTML with official **MJML 5**.

JSON is the source of truth. One folder per template. Brand tokens live on the template as `mj-attributes`. Preview is a loopback wrapper (600px + 320px); exported HTML has none of that chrome.

## Install

```bash
./install.sh
```

Builds release, drops the binary at `$HOME/.local/bin/dd_emailforge`, installs the default theme at `$HOME/.config/ldnddev/dd_emailforge_theme.yml` (only when no theme is already there), and creates `~/.config/ldnddev/dd_emailforge/templates/`. Override paths via `PREFIX`, `BIN_DIR`, or `CONFIG_DIR`. Honors `XDG_CONFIG_HOME`.

```bash
cargo build --release
```

MJML 5 needs **Node 20+**. `init` writes a `package.json` pin (`mjml ^5.4.0`); it does not run `npm install`.

## Usage

```bash
dd_emailforge init <dir> [--from welcome|newsletter|promo|transactional]
cd <dir> && npm install
dd_emailforge tui <dir>
dd_emailforge validate <dir>
dd_emailforge show <dir>/template.json
dd_emailforge export <dir>
dd_emailforge export <dir> --out other/
dd_emailforge preview <dir> --port 8766
```

`tui` with no path still launches chrome (info toast: run `init`). A directory argument means `dir/template.json`.

**Starters**

| `--from` | Intent | Footer |
|---|---|---|
| `welcome` (default) | Post-signup | address + Unsubscribe + `*|UNSUB|*` |
| `newsletter` | Digest | same |
| `promo` | Sale | same |
| `transactional` | Receipt / reset | address; empty unsub label **and** href (F3 clean) |

Welcome ships one Google Font (Raleway). Images in starters are `https://dummyimage.com/…` placeholders.

**TUI:** `F1` help · `F2` theme · `F3` validate · `p` preview · `Shift+E` export · `s` save · `/` insert · `Enter` edit · `Ctrl+Q` quit (confirms if unsaved; bare `q` does not).

Export writes `template.mjml` + `template.html` next to the JSON (or `--out`). Preview serves a loopback wrapper and watches the MJML file. Official `mjml` must be on `PATH` or in `node_modules/.bin/mjml`.

## Theme

Customize colors by writing the first of these that exists:

- `./dd_emailforge_theme.yml`
- `$XDG_CONFIG_HOME/ldnddev/dd_emailforge_theme.yml` (if `XDG_CONFIG_HOME` is set)
- `~/.config/ldnddev/dd_emailforge_theme.yml`

Schema: `LDNDDEV_TUI_VISUAL_STANDARD.md`. Every theme file must declare `version: 1`.

## Docs

- `Architecture.md` — crate map, pipeline, keys
- `docs/SPEC.md` — living conventions
- `docs/DESIGN.md` — locked design
- `components/*.md` — field contracts

## Tests

```bash
cargo test -q
```

Compiler-backed tests are `#[ignore]` and need `mjml` on `PATH` (`cargo test -- --ignored`).

## License

MIT License.
