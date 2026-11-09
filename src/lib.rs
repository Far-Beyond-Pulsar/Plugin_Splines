//! Splines (`pulsar.splines`): the plugin's root crate, the one place the
//! plugin registers everything it contributes.
//!
//! The plugin's other crates hold the code: `splines_component` (the
//! SceneDB component, its script methods and the editor lines that draw
//! curves) and `splines_editor` (the Spline tool mode, its panel and the
//! assistant's spline tools). Splines have no Helio feature. Neither crate
//! registers a contribution; this crate imports them and submits their
//! registrations, so this file and the two modules below are the whole list
//! of what the plugin brings:
//!
//! - [`runtime`]: what a game links. The component class and the editor
//!   lines (curves in the renderer's editor debug lines).
//! - `editor` (with the `editor` feature): what the editor adds. The Spline
//!   tool mode, the tool following selections made elsewhere and the
//!   assistant's spline tools.
//!
//! Every registration names this crate as its origin
//! (`pulsar_plugins::origin!()`), so the plugin gate skips all of them while
//! a project disables the plugin, and the plugin catalog lists them under
//! `pulsar.splines`. Other plugins never depend on this crate; hosts reach
//! it through the installed set or a generated game, a game without the
//! `editor` feature.

// The plugin's descriptor (`pulsar-plugin.toml`): linking this crate installs
// the plugin; a project enables it or not (`pulsar_plugins`).
pulsar_plugins::installed!("pulsar-plugin.toml");

pub use splines_component;
#[cfg(feature = "editor")]
pub use splines_editor;

pub mod runtime;

#[cfg(feature = "editor")]
pub mod editor;
