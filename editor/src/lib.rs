//! Splines (`pulsar.splines`): the editor crate -- the Spline tool mode,
//! its panel and the assistant's spline tools ([`level_editor`]), acting
//! through the component's methods. It registers nothing itself: the
//! plugin's root crate (`splines_plugin`, with its `editor` feature) submits
//! them through the level editor's extension points. The inspector edits
//! `SplineComponent` through its reflected properties.

// The level editor UI's text (`locales/`).
rust_i18n::i18n!("locales", fallback = "en");

pub mod level_editor;

// The unit tests attach curves to worlds, which takes a registered class.
// A test build of this crate does not link the root crate (the root links
// this crate under `editor`, so a dev-dependency on it would link two
// copies), so it registers the class itself; every other build gets it from
// the root only.
#[cfg(test)]
pulsar_world_registry::submit_world_component!(splines_component::SplineComponent);
