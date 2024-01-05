#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::QuickCaptureApp;

// pub use screenshot_utils::{ScreenshotMetadata, ScreenshotType};

mod screenshot_utils;
mod save_utils;
mod image_utils;
mod painting_utils;