use nannou::prelude::*;

struct Point {
    p1: Polar,
    p2: Polar,

    m1: f32,
    a1: f32,
    v1: f32,

    m2: f32,
    a2: f32,
    v2: f32,

    gravity: f32,

    history: Vec<(Polar, Polar)>
}

fn main() {
    nannou::app(value).simple_window(view).update(update).run();
}

fn value(_app: &App) -> Point {
    Point {
	p1: Polar::new(200.0, PI / 2.0),
	p2: Polar::new(200.0, PI / 2.0),

	m1: 30.0,
	a1: 0.0,
	v1: 0.0,

	m2: 30.0,
	a2: 0.0,
	v2: 0.0,

	gravity: 1.0,

	history: Vec::new(),
    }
}

fn update(_app: &App, p: &mut Point, _update: Update) {
    let u1 = -p.gravity * (2.0 * p.m1 + p.m2) * p.p1.angle.sin()
        - p.m2 * p.gravity * (p.p1.angle - 2.0 * p.p2.angle).sin()
        - 2.0
        * (p.p1.angle - p.p2.angle).sin()
        * p.m2
        * (p.v2 * p.v2 * p.p2.length
           + p.v1 * p.v1 * p.p1.length * (p.p1.angle - p.p2.angle).cos());
    let l1 = p.p1.length * (2.0 * p.m1 + p.m2 - p.m2 * (2.0 * p.p1.angle - 2.0 * p.p2.angle).cos());
    
    let u2 = 2.0
        * (p.p1.angle - p.p2.angle).sin()
        * (p.v1 * p.v1 * p.p1.length * (p.m1 + p.m2)
           + p.gravity * (p.m1 + p.m2) * p.p1.angle.cos()
                + p.v2 * p.v2 * p.p2.length * p.m2 * (p.p1.angle - p.p2.angle).cos());
    let l2 = p.p2.length * (2.0 * p.m1 + p.m2 - p.m2 * (2.0 * p.p1.angle - 2.0 * p.p2.angle).cos());


    p.a1 = u1 / l1;
    p.a2 = u2 / l2;
    
    p.v1 += p.a1;
    p.v2 += p.a2;
    p.p1.angle += p.v1;
    p.p2.angle += p.v2;

    let limit = 150;
    
    p.history.push((p.p1, p.p2));
    if p.history.len() > limit {
	p.history.reverse();
	p.history.truncate(limit);
	p.history.reverse();
    }
}

fn view(app: &App, p: &Point, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let win = app.window_rect().pad_top(100.0);

    let offset1 = p.p1.to_xy().to_nannou();
    let b1 = Rect::from_w_h(p.m1, p.m1)
        .mid_top_of(win)
        .shift(offset1);

    let offset2 = p.p2.to_xy().to_nannou();
    let b2 = Rect::from_w_h(p.m2, p.m2)
        .middle_of(b1)
        .shift(offset2);

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

    for (pos, _) in p.history.iter().enumerate() {
	if pos != 0 {
	    let base_x = 0.0;
	    let base_y = 200.0;

	    let (mut curr1, mut curr2) = p.history[pos];
	    let (mut last1, mut last2) = p.history[pos - 1];

	    curr1.angle += PI;
	    curr2.angle += PI;
	    last1.angle += PI;
	    last2.angle += PI;

	    let curr_x1 = base_x + curr1.angle.sin() * curr1.length;
	    let curr_y1 = base_y + curr1.angle.cos() * curr1.length;

	    let curr_x2 = curr_x1 + (curr2.angle.sin() * curr2.length);
	    let curr_y2 = curr_y1 + (curr2.angle.cos() * curr2.length); 

	    let last_x1 = base_x + last1.angle.sin() * last1.length;
	    let last_y1 = base_y + last1.angle.cos() * last1.length;

	    let last_y2 = last_y1 + (last2.angle.cos() * last2.length); 
	    let last_x2 = last_x1 + (last2.angle.sin() * last2.length);

	    let s1 = pt2(curr_x1, curr_y1);
	    let e1  = pt2(last_x1, last_y1);

	    let length = p.history.len() as f32;
	    let o = pos as f32 / (length * 2.0);

	    draw.line()
		.start(s1)
		.end(e1)
		.stroke_weight(o)
		.color(BLUE);

	    let s2 = pt2(curr_x2, curr_y2);
	    let e2  = pt2(last_x2, last_y2);

	    let length = p.history.len() as f32;
	    let o = pos as f32 / length;

	    draw.line()
		.start(s2)
		.end(e2)
		.stroke_weight(o)
		.color(RED);

	}
    }

    draw.to_frame(app, &frame).unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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
        let x = self.angle.sin() * self.length;
        let y = self.angle.cos() * self.length;
        XY { x, y }
    }
}
