# dd_emailforge

Terminal-UI email template builder. Edit a typed `template.json`, export strict MJML, compile HTML with official **MJML 5**.

**Tutorial (setup, install, TUI, screenshots):** [docs/tutorial/index.html](docs/tutorial/index.html)

## Install

```bash
./install.sh
```

Binary → `$HOME/.local/bin/dd_emailforge`. Theme → `$HOME/.config/ldnddev/` (existing theme is left alone). Honors `PREFIX`, `BIN_DIR`, `CONFIG_DIR`, `XDG_CONFIG_HOME`.

MJML 5 needs **Node 20+**. `init` pins `mjml ^5.4.0` in `package.json` but does not run `npm install`. Skip that step if `mjml` is already on your PATH.

## Quick start

```bash
dd_emailforge init ./my-email --from welcome
# only if mjml is not already on PATH:
cd my-email && npm install
dd_emailforge tui ./my-email
```

`tui` with no path still launches chrome. A directory argument means `dir/template.json`. Starters: `welcome` (default) · `newsletter` · `promo` · `transactional`.

Quit is **Ctrl+Q** only. Full keys, preview, and export: the [tutorial](docs/tutorial/index.html) and `F1` in the TUI.

## Docs

- [Tutorial](docs/tutorial/index.html) — setup, screenshots, how to recapture them
- `Architecture.md` — crate map and pipeline
- `docs/SPEC.md` — living conventions
- `docs/DESIGN.md` — locked design
- `components/*.md` — field contracts
- `LDNDDEV_TUI_VISUAL_STANDARD.md` — theme tokens

```bash
cargo test -q
```

Compiler-backed tests are `#[ignore]` (`cargo test -- --ignored`; needs `mjml` on PATH).

## License

MIT License.
