use egui::*;
use egui::epaint::RectShape;

#[derive(Clone)]
enum Side {
    None,
    Left,
    Right,
    Up,
    Down,
    Center
}

#[derive(Clone)]
pub struct Crop {
    cut_rect: Rect,
    scaled_rect: Rect,
    pub offset_x_right: f32,
    pub offset_x_left: f32,
    pub offset_y_up: f32,
    pub offset_y_down: f32,
    last_click: Pos2,
    limit_reached: bool,
    side: Side,
}

impl Default for Crop {
    fn default() -> Self {
        Self {
            cut_rect: Rect::NOTHING,
            scaled_rect: Rect::NOTHING,
            offset_x_right: 0.0,
            offset_x_left: 0.0,
            offset_y_up: 0.0,
            offset_y_down: 0.0,
            side: Side::None,
            limit_reached: false,
            last_click: Pos2::default(),
        }
    }
}

impl Crop {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn crop_img(&mut self, ui: &mut Ui, response: Response, dim: Vec2){
        self.cut_rect.min.x = response.rect.min.x + (response.rect.width() * self.offset_x_left) as f32;
        self.cut_rect.min.y = response.rect.min.y + (response.rect.height() * self.offset_y_up) as f32;
        self.cut_rect.max.x = response.rect.max.x - (response.rect.width() * self.offset_x_right) as f32;
        self.cut_rect.max.y = response.rect.max.y - (response.rect.height() * self.offset_y_down) as f32;

        if self.cut_rect.is_positive() {
            ui.painter().add(Shape::Rect(RectShape {
                rect: self.cut_rect,
                fill: Color32::from_rgba_unmultiplied(128, 128, 128, 20),
                stroke: Stroke::new(3.0, Color32::GRAY),
                rounding: Rounding::none()
            }));

            let xr = response.rect.min.x + (response.rect.width() * self.offset_x_left) as f32;
            let yr = response.rect.min.y + (response.rect.height() * self.offset_y_up) as f32;

            let xthird = self.cut_rect.size().x / 3.0;
            let ythird = self.cut_rect.size().y / 3.0;

            let point1thirdw = Pos2::new(xr + xthird, yr);
            let point2thirdw = Pos2::new(xr + xthird * 2.0, yr);

            let point1thirdh = Pos2::new(xr, yr + ythird);
            let point2thirdh = Pos2::new(xr, yr + ythird * 2.0);

            let point1thirdwh = Pos2::new(xr + xthird, yr + self.cut_rect.size().y);
            let point2thirdwh = Pos2::new(xr + xthird * 2.0, yr + self.cut_rect.size().y);

            let point1thirdhw = Pos2::new(xr + self.cut_rect.size().x, yr + ythird);
            let point2thirdhw = Pos2::new(xr + self.cut_rect.size().x, yr + ythird * 2.0);

            let line1 = Shape::dashed_line(&[point1thirdw, point1thirdwh], Stroke::new(1.0, Color32::WHITE), 2.5, 5.);
            let line2 = Shape::dashed_line(&[point2thirdw, point2thirdwh], Stroke::new(1.0, Color32::WHITE), 2.5, 5.);
            let line3 = Shape::dashed_line(&[point1thirdh, point1thirdhw], Stroke::new(1.0, Color32::WHITE), 2.5, 5.);
            let line4 = Shape::dashed_line(&[point2thirdh, point2thirdhw], Stroke::new(1.0, Color32::WHITE), 2.5, 5.);

            ui.painter().add(line1);
            ui.painter().add(line2);
            ui.painter().add(line3);
            ui.painter().add(line4);
        }

        let ctx = ui.ctx();
        
        let bound = 10.;
        let inner_rect = self.cut_rect.shrink(bound);

        if let Some(p_hover) = ctx.pointer_hover_pos() {

            //sopra e sotto
            if (p_hover.y <= self.cut_rect.min.y + bound && p_hover.y >= self.cut_rect.min.y - bound) || (p_hover.y <= self.cut_rect.max.y + bound && p_hover.y >= self.cut_rect.max.y - bound) {
                ctx.set_cursor_icon(CursorIcon::ResizeVertical);
            }
            //sinistra
            else if (p_hover.x <= self.cut_rect.min.x + bound && p_hover.x >= self.cut_rect.min.x - bound) || (p_hover.x <= self.cut_rect.max.x + bound && p_hover.x >= self.cut_rect.max.x - bound) {
                ctx.set_cursor_icon(CursorIcon::ResizeEast);
            } else if inner_rect.contains(p_hover) {
                ctx.set_cursor_icon(CursorIcon::AllScroll);
            }
        }
        if let Some(p_interact) = response.interact_pointer_pos() {
            if (p_interact.x <= (dim.x + response.rect.min.x) && p_interact.x >= response.rect.min.x) && (p_interact.y <= (dim.y + response.rect.min.y) && p_interact.y >= response.rect.min.y) {
                match self.side {
                    Side::None => {
                        if (p_interact.y - (response.rect.min.y + (response.rect.height() * self.offset_y_up))).abs() <= bound {
                            //println!("difference {:?} response {:?}", (p_interact.y - response.rect.min.y + self.offsetyu), response.rect.min);
                            ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                            self.side = Side::Up;
                        } else if (p_interact.y - (response.rect.max.y - (response.rect.height() * self.offset_y_down))).abs() <= bound {
                            ctx.set_cursor_icon(CursorIcon::ResizeVertical);
                            self.side = Side::Down;
                        } else if (p_interact.x - (response.rect.max.x - (response.rect.width() * self.offset_x_right))).abs() <= bound {
                            ctx.set_cursor_icon(CursorIcon::ResizeWest);
                            self.side = Side::Right;
                        } else if (p_interact.x - (response.rect.min.x + (response.rect.width() * self.offset_x_left))).abs() <= bound {
                            ctx.set_cursor_icon(CursorIcon::ResizeEast);
                            self.side = Side::Left;
                        } else if inner_rect.contains(p_interact) {
                            ctx.set_cursor_icon(CursorIcon::AllScroll);
                            self.side = Side::Center;
                        }
                    }
                    Side::Up => {
                        if p_interact != self.last_click {
                            let tmp = compute_offset(response.rect.min, p_interact).y / response.rect.height();
                            if (self.cut_rect.max.y - self.cut_rect.min.y) >= 50. || (self.limit_reached == true && tmp < self.offset_y_up) {
                                self.offset_y_up = tmp;
                                self.limit_reached = false;
                            } else {
                                self.limit_reached = true;
                            }
                            self.last_click = p_interact;
                        }
                    }
                    Side::Down => {
                        if p_interact != self.last_click {
                            let tmp = compute_offset(response.rect.max, p_interact).y / response.rect.height();

                            if (self.cut_rect.max.y - self.cut_rect.min.y) >= 50. || (self.limit_reached == true && tmp < self.offset_y_down) {
                                self.offset_y_down = tmp;
                                self.limit_reached = false;
                            } else {
                                self.limit_reached = true;
                            }
                            self.last_click = p_interact;
                        }
                    }
                    Side::Right => {
                        if p_interact != self.last_click {
                            //println!("click destra");
                            let tmp = compute_offset(response.rect.max, p_interact).x / response.rect.width();
                            if (self.cut_rect.max.x - self.cut_rect.min.x) >= 50. || (self.limit_reached == true && tmp < self.offset_x_right) {
                                self.offset_x_right = tmp;
                                self.limit_reached = false;
                            } else {
                                self.limit_reached = true;
                            }
                            self.last_click = p_interact;
                        }
                    }
                    Side::Left => {
                        if p_interact != self.last_click {
                            let tmp = compute_offset(response.rect.min, p_interact).x / response.rect.width();
                            if (self.cut_rect.max.x - self.cut_rect.min.x) >= 50. || (self.limit_reached == true && tmp < self.offset_x_left) {
                                self.offset_x_left = tmp;
                                self.limit_reached = false;
                            } else {
                                self.limit_reached = true;
                            }
                            self.last_click = p_interact;
                        }
                    }
                    Side::Center => {
                        if p_interact != self.last_click {
                            if p_interact.x < self.last_click.x {
                                let tmp = compute_offset(self.last_click, p_interact).x / response.rect.width();

                                if (response.rect.max.x - (response.rect.width() * (self.offset_x_right + tmp))) <= (dim.x + response.rect.min.x) && (response.rect.min.x + (response.rect.width() * (self.offset_x_left - tmp))) >= response.rect.min.x {
                                    self.offset_x_left -= tmp;
                                    self.offset_x_right += tmp;
                                }
                            }

                            if p_interact.x > self.last_click.x {
                                let tmp = compute_offset(self.last_click, p_interact).x / response.rect.width();

                                if (response.rect.max.x - (response.rect.width() * (self.offset_x_right - tmp))) <= (dim.x + response.rect.min.x) && (response.rect.min.x + (response.rect.width() * (self.offset_x_left + tmp))) >= response.rect.min.x {
                                    self.offset_x_left += tmp;
                                    self.offset_x_right -= tmp;
                                }
                            }

                            if p_interact.y < self.last_click.y {
                                let tmp = compute_offset(self.last_click, p_interact).y / response.rect.height();

                                if (response.rect.max.y - (response.rect.height() * (self.offset_y_down + tmp))) <= (dim.y + response.rect.min.y) && (response.rect.min.y + (response.rect.height() * (self.offset_y_up - tmp))) >= response.rect.min.y {
                                    self.offset_y_up -= tmp;
                                    self.offset_y_down += tmp;
                                }
                            }

                            if p_interact.y > self.last_click.y {
                                let tmp = compute_offset(self.last_click, p_interact).y / response.rect.height();

                                if (response.rect.max.y - (response.rect.height() * (self.offset_y_down - tmp))) <= (dim.y + response.rect.min.y) && (response.rect.min.y + (response.rect.height() * (self.offset_y_up + tmp))) >= response.rect.min.y {
                                    self.offset_y_up += tmp;
                                    self.offset_y_down -= tmp;
                                }
                            }

                            self.last_click = p_interact;
                        }
                    }
                }
            }
            self.last_click = p_interact;
        } else {
            self.side = Side::None;
        }
    }
    pub fn get_cut_rect(&mut self, originalsize: Vec2) -> Rect {
        let initialpoint = Pos2::new(originalsize.x * self.offset_x_left, originalsize.y * self.offset_y_up);

        let mut scaledsize = Vec2::ZERO;

        scaledsize.y = originalsize.y - originalsize.y * (self.offset_y_down + self.offset_y_up);
        scaledsize.x = originalsize.x - originalsize.x * (self.offset_x_left + self.offset_x_right);


        self.scaled_rect = Rect::from_min_size( initialpoint, scaledsize);
        self.scaled_rect
    }
}

fn compute_offset(standard: Pos2, current: Pos2) -> Pos2 { //standard: dove si trovano i bordi, current: dove e' stato cliccato
    Pos2::new((standard.x - current.x).abs(), (standard.y - current.y).abs())
}
