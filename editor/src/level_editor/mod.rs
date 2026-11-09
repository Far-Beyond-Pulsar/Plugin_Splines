//! Splines in the level editor, which the plugin's root crate registers
//! through its extension points (`ui_level_editor::extend`): the Spline
//! tool mode and its panel ([`mode`], [`panel`]), the tool following
//! selections made elsewhere ([`splines::sync_selection`]), and the
//! assistant's spline tools ([`ai_tools`], [`ai_tools::AI_TOOL_NAMESPACE`]). Curves are the splines
//! plugin's `SplineComponent`s, edited through undoable commands
//! ([`splines`]).

pub mod ai_tools;
pub mod mode;
pub mod panel;
pub mod preview;
pub mod splines;
pub mod state;

use ui_level_editor::ToolModeId;

pub use mode::SplineMode;
pub use state::{SplineDomain, SplineStateExt};

/// The Spline mode's id.
pub const SPLINE_MODE: ToolModeId = ToolModeId("spline");
