use flipperspiel::*;

pub struct Ball {
    x: f32,
    y: f32,
    scale: f32,
    color: Color,
}

impl Ball {
    pub fn new(position: (f32, f32), color: Color, size: f32) -> Self {
        Self {
            x: position.0,
            y: position.1,
            scale: size,
            color,
        }
    }
    pub fn draw(&self) {
        draw_circle(
            self.x * screen_width() - (screen_width() * self.scale / 2.0),
            self.y * screen_height() + (screen_height() * self.scale / 2.0),
            screen_width() * self.scale,
            self.color,
        );
    }
}
