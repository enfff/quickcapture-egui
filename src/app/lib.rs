#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::QuickCaptureApp;

mod screenshot_utils;
mod save_utils;
mod image_utils;
mod painting_utils;
mod pathlib;
mod save_view;