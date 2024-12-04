mod framework;
#[cfg(feature = "livereload")]
mod livereload;

use cabin_tailwind::registry::StyleSheet;
use framework::FrameworkLayer;

pub fn framework() -> FrameworkLayer {
    FrameworkLayer { stylesheet: None }
}

pub fn framework_with_stylesheet(stylesheet: &'static StyleSheet) -> FrameworkLayer {
    FrameworkLayer {
        stylesheet: Some(stylesheet),
    }
}
