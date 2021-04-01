use nannou::prelude::*;

struct Point {
    point_1: Polar,
    point_2: Polar,

    mass_1: f32,
    acc_1: f32,
    vel_1: f32,

    mass_2: f32,
    acc_2: f32,
    vel_2: f32,

    gravity: f32,
}

fn main() {
    nannou::app(value).simple_window(view).update(update).run();
}

fn value(_app: &App) -> Point {
    Point {
	point_1: Polar::new(200.0, PI / 7.0),
	point_2: Polar::new(200.0, PI / 3.0),

	mass_1: 30.0,
	acc_1: 0.0,
	vel_1: 0.0,

	mass_2: 30.0,
	acc_2: 0.0,
	vel_2: 0.0,

	gravity: 1.0,
    }
}

fn update(_app: &App, p: &mut Point, _update: Update) {
    let upper_1 = -p.gravity * (2.0 * p.mass_1 + p.mass_2) * p.point_1.angle.sin()
	- p.mass_2 * p.gravity * (p.point_1.angle - 2.0 * p.point_2.angle).sin()
	- 2.0 * (p.point_1.angle - p.point_2.angle).sin()
	* p.mass_2
	* (p.vel_2 * p.vel_2 * p.point_2.length
	   + p.vel_1 * p.vel_1 * p.point_1.length
	   * (p.point_1.angle - p.point_2.angle).cos());

    let lower_1 = p.point_1.length
	* (2.0 * p.mass_1 + p.mass_2 * 
	   (- p.mass_2 * (2.0 * p.point_1.angle - 2.0 * p.point_2.angle).cos()));

    let upper_2 = 2.0
	* (p.point_1.angle - p.point_2.angle).sin()
	* (p.vel_1 * p.vel_1 * p.point_1.length * (p.mass_1 + p.mass_2)
	   + p.gravity * (p.mass_1 + p.mass_2) * p.point_1.angle.cos()
	   + p.vel_2 * p.vel_2 * p.point_2.length * p.mass_2
	   * (p.point_1.angle - p.point_2.angle).cos());

    let lower_2 = p.point_2.length
	* (2.0 * p.mass_1 + (p.mass_2
			     * (2.0 * p.point_1.angle - 2.0 * p.point_2.angle).cos()));

    p.acc_1 = upper_1 / lower_1;
    p.acc_2 = upper_2 / lower_2;
    
    p.vel_1 += p.acc_1;
    p.vel_2 += p.acc_2;

    p.point_1.angle += p.vel_1;
    p.point_2.angle += p.vel_2;
}

fn view(app: &App, p: &Point, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let win = app.window_rect().pad_top(300.0);

    let offset_1 = p.point_1.to_xy().to_nannou();
    let b1 = Rect::from_w_h(p.mass_1, p.mass_1)
	.mid_top_of(win)
	.shift(offset_1);

    let offset_2 = p.point_2.to_xy().to_nannou();
    let b2 = Rect::from_w_h(p.mass_2, p.mass_2)
	.middle_of(b1)
	.shift(offset_2);

    draw.line()
	.start(win.mid_top())
	.end(b1.xy())
	.stroke_weight(3.0)
	.color(BLACK);

    draw.line()
	.start(b1.xy())
	.end(b2.xy())
	.stroke_weight(3.0)
	.color(BLACK);

    draw.ellipse().xy(b1.xy()).wh(b1.wh()).color(BLACK);
    draw.ellipse().xy(b2.xy()).wh(b2.wh()).color(BLACK);

	
    /*
    let p1 = p.point_1.to_xy();
    let p2 = p.point_2.to_xy();

    let line_s_1 = pt2(0.0, 0.0);
    let line_e_1 = pt2(p1.x, p1.y);

    let line_s_2 = pt2(p1.x, p1.y);
    let line_e_2 = pt2(p1.x + p2.x, p1.y + p2.y);

    draw.ellipse()
	.x_y(p1.x, p1.y)
        .radius(10.0)
        .color(BLACK);

    draw.line()
	.start(line_s_1)
	.end(line_e_1)
	.weight(1.0)
	.color(BLACK);

    draw.ellipse()
	.x_y(p1.x + p2.x, p2.y + p2.y)
        .radius(10.0)
        .color(BLACK);

    draw.line()
	.start(line_s_2)
	.end(line_e_2)
	.weight(1.0)
	.color(BLACK);

    */

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
