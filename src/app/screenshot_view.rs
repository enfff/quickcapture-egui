use super::ScreenshotType;
use crate::app;
use display_info::DisplayInfo;
use egui::*;

#[derive(Clone)]
pub struct ScreenshotView {
    id: Option<LayerId>,
    pub started_selection: bool,
    pub starting_point: Pos2,
    pub middle_point: Pos2,
    pub ending_point: Pos2,
    pub dimension_selected: Vec2,
    pub finished_selection: bool,
    pub screen_selected: u32,
    pub timer_delay: i32,
}

impl Default for ScreenshotView {
    fn default() -> Self {
        Self {
            id: None,
            started_selection: false,
            starting_point: Default::default(),
            middle_point: Default::default(),
            ending_point: Default::default(),
            dimension_selected: Default::default(),
            finished_selection: false,
            screen_selected: 0,
            timer_delay: 0,
        }
    }
}
impl ScreenshotView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(
        &mut self,
        ctx: &Context,
        _frame: &mut eframe::Frame,
        _view: &mut app::Views,
        _type: &mut Option<ScreenshotType>,
    ) {
        ctx.set_cursor_icon(CursorIcon::Crosshair);
        let width = _frame.info().window_info.monitor_size.unwrap().x;
        let height = _frame.info().window_info.monitor_size.unwrap().y;
        _frame.set_decorations(false);
        _frame.set_window_size(vec2(width + 1., height + 1.));
        _frame.set_window_pos(Pos2::ZERO);

        Area::new("screen")
        .order(Order::Background)
        .show(ctx, |ui| {
            ui.label("Screenshot");
            let rect = ui.max_rect();
            ui.painter()
                .rect_filled(rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 30));
            let response = ui.allocate_response(rect.size(), Sense::drag());
            let bound = response.rect.size();
            if response.drag_started() {
                self.starting_point = ctx.pointer_interact_pos().unwrap();
                self.started_selection = true;
            }
            if response.dragged() {
                self.middle_point = ctx.pointer_interact_pos().unwrap();
                if self.middle_point != self.starting_point && self.started_selection {
                    let selected_area = Rect::from_two_pos(self.starting_point, self.middle_point);
                    let selected = ui.painter().add(Shape::Noop);
                    /*let where_is_selected = */
                    ui.painter().set(
                        selected,
                        epaint::RectShape {
                            rounding: Rounding::none(),
                            fill: Color32::from_rgba_unmultiplied(255, 255, 255, 0),
                            stroke: Stroke::new(2.0, Color32::WHITE),
                            rect: selected_area,
                        },
                    );
                }
            }
            if response.drag_released() {
                self.ending_point = ctx.pointer_interact_pos().unwrap();

                self.dimension_selected.x = self.ending_point.x - self.starting_point.x;
                self.dimension_selected.y = self.ending_point.y - self.starting_point.y;

                if self.dimension_selected.x.is_sign_negative()
                    || self.dimension_selected.y.is_sign_negative()
                {
                    self.dimension_selected.x = self.dimension_selected.x.abs();
                    self.dimension_selected.y = self.dimension_selected.y.abs();
                    let tmp = self.starting_point;
                    self.starting_point = self.ending_point;
                    self.ending_point = tmp;
                }

                if self.dimension_selected.x > 50.0 && self.dimension_selected.y > 50.0 {
                    if self.dimension_selected.x > bound.x {
                        self.dimension_selected.x = bound.x;
                    }
                    if self.dimension_selected.y > bound.y {
                        self.dimension_selected.y = bound.y;
                    }
                    self.finished_selection = true;
                    let disp = DisplayInfo::from_point(
                        (self.starting_point.x as u32).try_into().unwrap(),
                        (self.starting_point.y as u32).try_into().unwrap(),
                    )
                    .unwrap();
                    self.screen_selected = disp.id;
                    *_type = Some(ScreenshotType::PartialScreen);
                } else {
                    println!("ups!");
                    self.started_selection = false;
                }
                if _type.is_some() {
                    ui.set_visible(false);
                    ctx.request_repaint();
                }
            }
        });

        Window::new("Screenshot")
            .title_bar(false)
            .default_pos(pos2(250.0, 150.0))
            .show(ctx, |ui| {
                self.id = Some(ui.layer_id());
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("◀ Go back").clicked() {
                            _frame.set_window_size(vec2(640.0, 400.0));
                            _frame.set_centered();
                            *_view = app::Views::Home;
                            println!("Go back button pressed");
                        }

                        if ui.add_enabled(false, Button::new("⛶")).clicked() {
                            *_type = Some(ScreenshotType::PartialScreen);
                            println!("PartialScreen button pressed");
                        }
                        ui.separator();

                        if ui.button("🖵 Fullscreen").clicked() {
                            *_type = Some(ScreenshotType::FullScreen);
                            println!("FullScreen button pressed");
                        }
                        ui.separator();


                        if _type.is_some() {
                            ui.set_visible(false);
                            ctx.request_repaint();
                        }

                        let mut _timer_delay = self.timer_delay + 150;

                        ui.add(egui::DragValue::new(&mut _timer_delay).speed(50).max_decimals(2).clamp_range(0..=10000).prefix("Delay Timer (ms): "));
                    });
                });
            });
    }
}
