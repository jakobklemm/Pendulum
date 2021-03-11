use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(WHITE);

    // Draw a purple triangle in the top left half of the window.
    //let win = app.window_rect();

    // Draw an ellipse to follow the mouse.
    let base_x = 0.0;
    let base_y = 0.0;

    draw.ellipse()
	.x_y(base_x, base_y)
        .radius(10.0)
        .color(BLACK);

    let t = app.time / 3.0;
    let len = 100.0;
    let w = 6.3;
    let _polar = Polar::new(len, 0.0).to_xy();
    let target_x = len * (w * t).cos().sin();
    let target_y = len * (1.0 - ((w * t).cos()).cos()) - len;
    let line_s = pt2(base_x, base_y);
    let line_e = pt2(target_x, target_y);

    draw.ellipse()
	.x_y(target_x, target_y)
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
