mod framework;
#[cfg(feature = "livereload")]
mod livereload;

use framework::FrameworkLayer;

pub fn framework() -> FrameworkLayer {
    FrameworkLayer
}
