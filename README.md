# QuickCapture
A multi platform screenshot utility written in Rust. 
Optional project for Programmazione di sistema [02GRSOV] course at Politecnico di Torino (Italy). Final evaluation: 6 out of 6.

## Features
- Multiscreen capture (partial or full-screen)
- Delay timer: delays the capture for the desired time in milliseconds
- Easily accessible User Interface - egui 0.22.0 (egui-extras, egui-toast, egui-modal)
- Take notes on screenshot
- Crop: it is possible to crop the capture afterwards
- Multi-format save to drive (PNG, JPEG, GIF)
- Clipboard: it is possible to copy and paste the screenshot in another application
- Hotkeys support (not global)

## Docs
[Egui Docs](https://docs.rs/egui/latest/egui/)
[Tutorial](https://youtu.be/NtUkr_z7l84)

## Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`
