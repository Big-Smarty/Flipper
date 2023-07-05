use flipperspiel::*;

pub struct Puncher {
    root: Vec2,
    angle: f32,
    length: f32,
}

impl Puncher {
    pub fn new(root: Vec2, angle: f32) -> Self {
        Self {
            root,
            angle,
            length: 0.12,
        }
    }
    pub fn draw(&self) {
        draw_line(
            self.root.x * screen_width(),
            self.root.y * screen_height(),
            self.root.x * screen_width()
                + (self.angle.to_radians().cos() * self.length * screen_width()),
            self.root.y * screen_height()
                - (self.angle.to_radians().sin() * self.length * screen_height()),
            5.0,
            BLACK,
        );
    }
}
