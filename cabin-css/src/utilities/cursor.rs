//! Set the cursor style (`cursor`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>

use crate::Property;

const CURSOR: &str = "cursor";

/// ```css
/// cursor: auto;
/// ```
pub const AUTO: Property = Property(CURSOR, "auto");

/// ```css
/// cursor: default;
/// ```
pub const DEFAULT: Property = Property(CURSOR, "default");

/// ```css
/// cursor: pointer;
/// ```
pub const POINTER: Property = Property(CURSOR, "pointer");

/// ```css
/// cursor: wait;
/// ```
pub const WAIT: Property = Property(CURSOR, "wait");

/// ```css
/// cursor: text;
/// ```
pub const TEXT: Property = Property(CURSOR, "text");

/// ```css
/// cursor: move;
/// ```
pub const MOVE: Property = Property(CURSOR, "move");

/// ```css
/// cursor: help;
/// ```
pub const HELP: Property = Property(CURSOR, "help");

/// ```css
/// cursor: not-allowed;
/// ```
pub const NOT_ALLOWED: Property = Property(CURSOR, "not-allowed");

/// ```css
/// cursor: none;
/// ```
pub const NONE: Property = Property(CURSOR, "none");

/// ```css
/// cursor: context-menu;
/// ```
pub const CONTEXT_MENU: Property = Property(CURSOR, "context-menu");

/// ```css
/// cursor: progress;
/// ```
pub const PROGRESS: Property = Property(CURSOR, "progress");

/// ```css
/// cursor: cell;
/// ```
pub const CELL: Property = Property(CURSOR, "cell");

/// ```css
/// cursor: crosshair;
/// ```
pub const CROSSHAIR: Property = Property(CURSOR, "crosshair");

/// ```css
/// cursor: vertical-text;
/// ```
pub const VERTICAL_TEXT: Property = Property(CURSOR, "vertical-text");

/// ```css
/// cursor: alias;
/// ```
pub const ALIAS: Property = Property(CURSOR, "alias");

/// ```css
/// cursor: copy;
/// ```
pub const COPY: Property = Property(CURSOR, "copy");

/// ```css
/// cursor: no-drop;
/// ```
pub const NO_DROP: Property = Property(CURSOR, "no-drop");

/// ```css
/// cursor: grab;
/// ```
pub const GRAB: Property = Property(CURSOR, "grab");

/// ```css
/// cursor: grabbing;
/// ```
pub const GRABBING: Property = Property(CURSOR, "grabbing");

/// ```css
/// cursor: all-scroll;
/// ```
pub const ALL_SCROLL: Property = Property(CURSOR, "all-scroll");

/// ```css
/// cursor: col-resize;
/// ```
pub const COL_RESIZE: Property = Property(CURSOR, "col-resize");

/// ```css
/// cursor: row-resize;
/// ```
pub const ROW_RESIZE: Property = Property(CURSOR, "row-resize");

/// ```css
/// cursor: n-resize;
/// ```
pub const N_RESIZE: Property = Property(CURSOR, "n-resize");

/// ```css
/// cursor: e-resize;
/// ```
pub const E_RESIZE: Property = Property(CURSOR, "e-resize");

/// ```css
/// cursor: s-resize;
/// ```
pub const S_RESIZE: Property = Property(CURSOR, "s-resize");

/// ```css
/// cursor: w-resize;
/// ```
pub const W_RESIZE: Property = Property(CURSOR, "w-resize");

/// ```css
/// cursor: ne-resize;
/// ```
pub const NE_RESIZE: Property = Property(CURSOR, "ne-resize");

/// ```css
/// cursor: nw-resize;
/// ```
pub const NW_RESIZE: Property = Property(CURSOR, "nw-resize");

/// ```css
/// cursor: se-resize;
/// ```
pub const SE_RESIZE: Property = Property(CURSOR, "se-resize");

/// ```css
/// cursor: sw-resize;
/// ```
pub const SW_RESIZE: Property = Property(CURSOR, "sw-resize");

/// ```css
/// cursor: ew-resize;
/// ```
pub const EW_RESIZE: Property = Property(CURSOR, "ew-resize");

/// ```css
/// cursor: ns-resize;
/// ```
pub const NS_RESIZE: Property = Property(CURSOR, "ns-resize");

/// ```css
/// cursor: nesw-resize;
/// ```
pub const NESW_RESIZE: Property = Property(CURSOR, "nesw-resize");

/// ```css
/// cursor: nwse-resize;
/// ```
pub const NWSE_RESIZE: Property = Property(CURSOR, "nwse-resize");

/// ```css
/// cursor: zoom-in;
/// ```
pub const ZOOM_IN: Property = Property(CURSOR, "zoom-in");

/// ```css
/// cursor: zoom-out;
/// ```
pub const ZOOM_OUT: Property = Property(CURSOR, "zoom-out");
