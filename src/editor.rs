//! What the splines plugin registers for the editor (the `editor` feature):
//! its level editor tool and the assistant's spline tools.

use splines_editor::level_editor::{ai_tools, splines, SplineMode};
use ui_level_editor::extend::{AiToolNamespace, SelectionFollowUp, ToolModeRegistration};

/// Everything the splines plugin registers for the editor.
pub(crate) fn build(p: &mut pulsar_plugins::PluginBuilder) {
    // ── Level editor ────────────────────────────────────────────────────────

    // The Spline tool mode and its panel, after Level Edit in the mode picker.
    p.add(ToolModeRegistration {
        origin: p.origin(),
        id: "splines.spline",
        order: 10,
        make: || Box::new(SplineMode),
    });

    // The Spline tool follows a curve selected elsewhere (the hierarchy, the
    // viewport).
    p.add(SelectionFollowUp {
        origin: p.origin(),
        id: "splines.follow",
        follow: splines::sync_selection,
    });

    // ── Assistant ───────────────────────────────────────────────────────────

    // The assistant's spline tools (create, edit, list curves).
    p.add(AiToolNamespace {
        origin: p.origin(),
        namespace: ai_tools::AI_TOOL_NAMESPACE,
    });
}
