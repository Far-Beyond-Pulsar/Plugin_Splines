//! What the splines plugin registers for a game (and the editor): every
//! runtime contribution, grouped by the engine seam it plugs into.

use engine_backend::subsystems::render::EditorLinesRegistration;
use splines_component::SplineComponent;

// ── Component classes ───────────────────────────────────────────────────────

pulsar_world_registry::submit_world_component!(SplineComponent);

// ── Rendering ───────────────────────────────────────────────────────────────

// Every visible curve in the renderer's editor debug lines.
inventory::submit! {
    EditorLinesRegistration {
        origin: pulsar_plugins::origin!(),
        name: "splines",
        new: splines_component::new_spline_lines,
    }
}

#[cfg(test)]
mod tests {
    use engine_backend::subsystems::render::EditorLinesRegistration;

    #[test]
    fn registers_its_editor_lines() {
        assert!(
            pulsar_plugins::registered::<EditorLinesRegistration>()
                .any(|registration| registration.name == "splines")
        );
    }

    /// Every registration of the plugin names this root, so it resolves to
    /// the plugin's id.
    #[test]
    fn every_registration_resolves_to_the_plugin() {
        let ours: Vec<_> = pulsar_plugins::contributions()
            .into_iter()
            .filter(|summary| summary.origin == pulsar_plugins::origin!())
            .collect();
        assert!(
            ours.iter()
                .any(|summary| summary.category == "component.class")
        );
        for summary in &ours {
            assert_eq!(
                pulsar_plugins::plugin_of(summary.origin).map(|plugin| plugin.id.as_str()),
                Some("pulsar.splines"),
                "{summary:?}"
            );
        }
    }
}
