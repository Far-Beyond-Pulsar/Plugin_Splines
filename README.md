# Plugin_Splines

Authored curves: their component, its methods and the editor lines that show them.

`pulsar.splines` ("Splines") is a plugin of
[Pulsar-Native](https://github.com/Far-Beyond-Pulsar/Pulsar-Native), the engine and editor that renders with
[Helio](https://github.com/Far-Beyond-Pulsar/Helio). Pulsar-Native vendors this repository as a git submodule
at `plugins/vendor/splines`.

## What it owns

Every object kind is a plugin. A plugin owns its SceneDB components (their
data and scriptable methods; the editor calls the same methods scripts do),
the contracts other kinds read (its `api` crate: graph keys, row ABIs, WGSL
snippets), the Helio feature that renders it (passes, scene derivations,
settings), its file formats and its editor UI. Helio knows no object kind,
and other kinds meet this one only through frame contracts, extension points,
draw phases and api crates.

| Crate | Path | What |
|---|---|---|
| `splines_component` | `component/` | Splines (pulsar.splines): the spline component, its script methods and the editor lines it draws through the renderer's debug draw |
| `splines_plugin` | `.` | Splines (`pulsar.splines`): the plugin's root crate, the one place it registers everything it contributes (runtime registrations always, the editor's with `editor`) |
| `splines_editor` | `editor/` | Splines (pulsar.splines): the editor crate -- the Spline tool mode, its panel and the assistant's spline tools, over the plugin's component crate |

The root crate (`splines_plugin`, this directory's package) is the one place
the plugin registers what it contributes: its component classes, every
registration that names an origin (`pulsar_plugins::origin!()`) and the
descriptor. The other crates hold the code and register nothing but their
types' own attachments, so `src/runtime.rs` (and `src/editor.rs`, with
the `editor` feature) lists everything the plugin brings. The plugin's
integration tests (`tests/`) and the editor crate's unit tests link the
root (without `editor`), as a game does.

- **Runtime module** (what a game built for a project links): `splines_plugin`.
- **Editor module** (what the editor links in addition): `splines_plugin` with its
  `editor` feature (`splines_editor`).
- **Requires**: nothing. A project enables or disables the plugin in its
  `Pulsar.toml` (`[plugins]`); disabled, the kind is absent and the frame
  draws without it.
- **Reads other kinds through**: none.
- **Its standalone hosts also compose**: none (demos and GPU tests
  draw a whole scene; the plugin itself does not depend on these).

## Building and testing

Like [Plugin_Physics](https://github.com/Far-Beyond-Pulsar/Plugin_Physics), this plugin's crates are members
of Pulsar-Native's workspace: their dependencies are `workspace = true`, and
Helio comes from Pulsar-Native's `crates/renderer/helio` submodule. It builds
only inside a Pulsar-Native checkout:

```bash
git clone --recursive https://github.com/Far-Beyond-Pulsar/Pulsar-Native
cd Pulsar-Native
cargo check -p splines_plugin -p splines_component -p splines_editor --features splines_plugin/editor --tests
cargo test -p splines_plugin -p splines_component
```

Suites that render real frames need a GPU; CI leaves them out by name
(Pulsar-Native's `.config/nextest.toml`, `ci` profile). This repository's CI
checks the plugin inside a Pulsar-Native checkout: rustfmt, clippy and the
`ci` profile's tests.

A change that spans this plugin and Pulsar-Native (or Helio) merges
dependency-first: merge here, re-pin `plugins/vendor/splines` in Pulsar-Native to
this repository's `main`, then merge the Pulsar-Native PR (Pulsar-Native's
`.agents/CRATES.md`, "Submodule pins").

## Architecture

- [Every object kind is a plugin](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/docs/architecture/plugin-migration.md): the
  rule, what Helio is, the plugin table and the "As built" notes of each
  phase.
- [The layered rewrite](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/docs/architecture/layered-rewrite.md): layers, buffer
  contracts, extension points and why the renderer has this shape.
- [The `helio-architecture` skill](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/.agents/skills/helio-architecture/SKILL.md): required reading for any change
  here (see `AGENTS.md`).
- Pulsar-Native's `.agents/PLUGIN_SYSTEM.md` and `.agents/CRATES.md`: plugin
  descriptors, the installed set and the submodule pins.

History before 2026-11-02 (when the plugin moved into Pulsar-Native) is in
Helio's repository; this repository's history starts with that move
(`git subtree split` of `plugins/vendor/splines`).

## License

MIT, as the rest of Pulsar (`LICENSE.md`).
