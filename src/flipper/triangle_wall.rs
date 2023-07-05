use flipperspiel::*;

pub struct TriangleWall {
    vertices: [Vec2; 3],
    color: Color,
}

impl TriangleWall {
    pub fn new(vertices: [Vec2; 3], color: Color) -> Self {
        Self { vertices, color }
    }
    pub fn draw(&self) {
        draw_triangle(
            Vec2 {
                x: self.vertices[0].x * screen_width(),
                y: self.vertices[0].y * screen_height(),
            },
            Vec2 {
                x: self.vertices[1].x * screen_width(),
                y: self.vertices[1].y * screen_height(),
            },
            Vec2 {
                x: self.vertices[2].x * screen_width(),
                y: self.vertices[2].y * screen_height(),
            },
            self.color,
        );
    }
}
