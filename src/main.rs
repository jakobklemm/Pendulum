use nannou::prelude::*;

struct Point {
    point: XY,

    angle: f32,
    acc: f32,
    vel: f32,
    gravity: f32,
    len: f32
}

fn main() {
    nannou::app(value).simple_window(view).update(update).run();
}

fn value(_app: &App) -> Point {
    let distance = 200.0;
    Point {
	point: Polar::new(distance, 0.0).to_xy(),

	angle: -1.2 * PI,
	acc: 0.0,
	vel: 0.0,
	gravity: 0.5,
	len: distance
    }
}

fn update(_app: &App, m: &mut Point, _update: Update) {
    m.acc = (m.gravity / m.len) * m.angle.sin();
    m.vel = m.vel + m.acc;
    m.angle = m.angle + m.vel;
    m.point.x = m.len * m.angle.sin();
    m.point.y = m.len * m.angle.cos();

}

fn view(app: &App, m: &Point, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let line_s = pt2(0.0, 0.0);
    let line_e = pt2(m.point.x, m.point.y);

    draw.ellipse()
	.x_y(m.point.x, m.point.y)
        .radius(10.0)
        .color(BLACK);

    draw.line()
	.start(line_s)
	.end(line_e)
	.weight(1.0)
	.color(BLACK);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

#[derive(Debug, PartialEq)]
pub struct XY {
    x: f32,
    y: f32,
}

impl XY {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    #[allow(dead_code)]
    fn round(mut self) -> Self {
        self.x = self.x.round();
        self.y = self.y.round();
        self
    }

    pub fn to_nannou(&self) -> nannou::geom::point::Point2 {
        nannou::geom::Vector2::new(
            -self.x as nannou::geom::scalar::Default,
            -self.y as nannou::geom::scalar::Default,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Polar {
    pub length: f32,
    pub angle: f32,
}

impl Polar {
    pub fn new(length: f32, angle: f32) -> Self {
        Self { length, angle }
    }

    pub fn from_xy(xy: XY) -> Self {
        let length = (xy.x * xy.x + xy.y * xy.y).sqrt();
        let angle = xy.y.atan2(xy.x);

        Self { angle, length }
    }

    pub fn to_xy(&self) -> XY {
        let x = self.angle.cos() * self.length;
        let y = self.angle.sin() * self.length;
        XY { x, y }
    }
}
