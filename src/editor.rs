//! What the splines plugin registers for the editor (the `editor` feature):
//! its level editor tool and the assistant's spline tools.

use splines_editor::level_editor::{SplineMode, ai_tools, splines};
use ui_level_editor::extend::{AiToolNamespace, SelectionFollowUp, ToolModeRegistration};

// ── Level editor ────────────────────────────────────────────────────────────

// The Spline tool mode and its panel, after Level Edit in the mode picker.
inventory::submit! {
    ToolModeRegistration {
        origin: pulsar_plugins::origin!(),
        id: "splines.spline",
        order: 10,
        make: || Box::new(SplineMode),
    }
}

// The Spline tool follows a curve selected elsewhere (the hierarchy, the
// viewport).
inventory::submit! {
    SelectionFollowUp {
        origin: pulsar_plugins::origin!(),
        id: "splines.follow",
        follow: splines::sync_selection,
    }
}

// ── Assistant ───────────────────────────────────────────────────────────────

// The assistant's spline tools (create, edit, list curves).
inventory::submit! {
    AiToolNamespace {
        origin: pulsar_plugins::origin!(),
        namespace: ai_tools::AI_TOOL_NAMESPACE,
    }
}
