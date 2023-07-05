use flipperspiel::*;

mod ball;
mod goal;
mod puncher;
mod triangle_wall;
mod wall;

pub struct Flipper {
    ball: ball::Ball,
    goals: Vec<goal::Goal>,
    punchers: Vec<puncher::Puncher>,
    triangle_walls: Vec<triangle_wall::TriangleWall>,
    walls: Vec<wall::Wall>,
}

impl Flipper {
    pub fn new() -> Self {
        info!("Created flipper!");
        let out = Self {
            ball: ball::Ball::new((0.5, 0.5), BLACK, 0.01),
            goals: {
                let mut out = Vec::new();
                out.push(goal::Goal::new(
                    [Vec2 { x: 0.22, y: 0.0 }, Vec2 { x: 0.78, y: 0.0 }],
                    BLUE,
                ));
                out.push(goal::Goal::new(
                    [Vec2 { x: 0.22, y: 1.0 }, Vec2 { x: 0.78, y: 1.0 }],
                    RED,
                ));
                out
            },
            punchers: {
                let mut out = Vec::new();
                out.push(puncher::Puncher::new(Vec2 { x: 0.33, y: 0.1 }, 20.0));
                out.push(puncher::Puncher::new(Vec2 { x: 0.67, y: 0.1 }, 160.0));
                out.push(puncher::Puncher::new(Vec2 { x: 0.33, y: 0.9 }, -20.0));
                out.push(puncher::Puncher::new(Vec2 { x: 0.67, y: 0.9 }, -160.0));
                out
            },
            triangle_walls: {
                let mut out = Vec::new();
                out.push(triangle_wall::TriangleWall::new(
                    [
                        Vec2::new(0.25, 0.1),
                        Vec2::new(0.25, 0.15),
                        Vec2::new(0.35, 0.095),
                    ],
                    BLACK,
                ));
                out.push(triangle_wall::TriangleWall::new(
                    [
                        Vec2::new(0.75, 0.1),
                        Vec2::new(0.75, 0.15),
                        Vec2::new(0.65, 0.095),
                    ],
                    BLACK,
                ));
                out.push(triangle_wall::TriangleWall::new(
                    [
                        Vec2::new(0.25, 0.9),
                        Vec2::new(0.25, 0.85),
                        Vec2::new(0.35, 0.905),
                    ],
                    BLACK,
                ));
                out.push(triangle_wall::TriangleWall::new(
                    [
                        Vec2::new(0.75, 0.9),
                        Vec2::new(0.75, 0.85),
                        Vec2::new(0.65, 0.905),
                    ],
                    BLACK,
                ));
                out
            },
            walls: {
                let mut out = Vec::new();
                out.push(wall::Wall::new(Vec2 { x: 0.2, y: 0.0 }, 0.05, 1.0, BLACK));
                out.push(wall::Wall::new(Vec2 { x: 0.75, y: 0.0 }, 0.05, 1.0, BLACK));
                out
            },
        };
        info!("Finished initializing flipper!");
        out
    }
    pub fn draw(&self) {
        self.ball.draw();
        for i in &self.goals {
            i.draw();
        }
        for i in &self.punchers {
            i.draw();
        }
        for i in &self.triangle_walls {
            i.draw();
        }
        for i in &self.walls {
            i.draw();
        }
    }
}
