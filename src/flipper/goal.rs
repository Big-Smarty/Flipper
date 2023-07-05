use flipperspiel::*;

pub struct Goal {
    vertices: [Vec2; 2],
    color: Color,
}

impl Goal {
    pub fn new(vertices: [Vec2; 2], color: Color) -> Self {
        Self { vertices, color }
    }
    pub fn draw(&self) {
        draw_line(
            self.vertices[0].x * screen_width(),
            self.vertices[0].y * screen_height(),
            self.vertices[1].x * screen_width(),
            self.vertices[1].y * screen_height(),
            10.0,
            self.color,
        );
    }
}
