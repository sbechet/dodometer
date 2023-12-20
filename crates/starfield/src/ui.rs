use super::StarField;
use egui::{Color32, Pos2, Rect, Shape, Ui};

pub struct StarFieldUi {
    pub sf: StarField,
}

impl Default for StarFieldUi {
    fn default() -> Self {
        Self {
            sf: StarField::default(),
        }
    }
}

impl StarFieldUi {
    pub fn background(&mut self, ui: &mut Ui) {
        let rect = ui.painter().clip_rect();
        let mut shapes: Vec<Shape> = Vec::new();

        self.sf.screen_size = (rect.width() as u32, rect.height() as u32);
        self.sf.frame();

        let mut put_pixel = |point: Pos2, color: Color32| {
            let p = [point, Pos2::new(point.x + 1.0, point.y + 1.0)];
            if rect.intersects(Rect::from_two_pos(p[0], p[1])) {
                shapes.push(Shape::line_segment(p, (1.0, color)));
            }
        };

        if ui.is_rect_visible(rect) {
            for pix in self.sf.position.iter() {
                put_pixel(pix.pos2, pix.c);
            }
        }
        ui.painter().extend(shapes);
    }
}
