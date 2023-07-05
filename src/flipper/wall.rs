use flipperspiel::*;

pub struct Wall {
    root: Vec2,
    width: f32,
    height: f32,
    color: Color,
}

impl Wall {
    pub fn new(root: Vec2, width: f32, height: f32, color: Color) -> Self {
        Self {
            root,
            width,
            height,
            color,
        }
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.root.x * screen_width(),
            self.root.y * screen_height(),
            self.width * screen_width(),
            self.height * screen_height(),
            self.color,
        )
    }
}
