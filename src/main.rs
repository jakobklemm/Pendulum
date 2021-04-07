use nannou::prelude::*;
use nannou::ui::prelude::*;

// Defines model & data structure
struct Point {
    ui: Ui,
    // Each UI Element needs their own ID in the state object.
    s1: widget::Id,
    s2: widget::Id,
    s3: widget::Id,
    s4: widget::Id,
    s5: widget::Id,
    s6: widget::Id,
    s7: widget::Id,

    p1: Polar,
    p2: Polar,

    m1: f64,
    a1: f64,
    v1: f64,

    m2: f64,
    a2: f64,
    v2: f64,

    gravity: f64,

    history: Vec<(Polar, Polar)>,
}

fn main() {
    nannou::app(value).update(update).simple_window(view).run();
}

// Initiates the state using default vaules & random ID values.
fn value(app: &App) -> Point {
    let mut ui = app.new_ui().build().unwrap();

    let s1 = ui.generate_widget_id();
    let s2 = ui.generate_widget_id();
    let s3 = ui.generate_widget_id();
    let s4 = ui.generate_widget_id();
    let s5 = ui.generate_widget_id();
    let s6 = ui.generate_widget_id();
    let s7 = ui.generate_widget_id();

    Point {
        p1: Polar::new(250.0, 1.0),

        m1: 30.0,
        a1: 0.0,
        v1: 0.0,

        p2: Polar::new(250.0, 3.0),
        m2: 30.0,
        a2: 0.0,
        v2: 0.0,

        gravity: 0.3,

        history: Vec::new(),

        ui: ui,
        s1: s1,
        s2: s2,
        s3: s3,
        s4: s4,
        s5: s5,
        s6: s6,
        s7: s7,
    }
}

// Runs the main calculations for the new positions and controls the sliders.
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

    //Instead of using some complex redrawing logic a simple array of last positions is stored. 
    
    let limit = 125;

    p.history.push((p.p1, p.p2));
    if p.history.len() > limit {
        p.history.reverse();
        p.history.truncate(limit);
        p.history.reverse();
    }

    // Using the "set" method sliders & texts are added to the current UI object, from where they are drawn.
    let ui = &mut p.ui.set_widgets();

    // Initialize and connect all sliders & text fields.

    widget::Text::new("Pendulum Simulation")
        .top_right_with_margins(20.0, 20.0)
        .set(p.s6, ui);

    widget::Text::new("github.com/jakobklemm")
        .top_right_with_margins(40.0, 20.0)
        .set(p.s7, ui);

    for value in widget::Slider::new(p.gravity, 0.01, 2.0)
        .top_left_with_margins(20.0, 20.0)
        .label("Gravity")
        .w_h(300.0, 25.0)
        .label_font_size(13)
        .rgb(1.0, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(p.s1, ui)
    {
        p.gravity = value;
    }

    for value in widget::Slider::new(p.m1, 1.0, 100.0)
        .top_left_with_margins(50.0, 20.0)
        .label("Point 1 - Mass")
        .w_h(300.0, 25.0)
        .label_font_size(13)
        .rgb(1.0, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(p.s2, ui)
    {
        p.m1 = value;
    }

    for value in widget::Slider::new(p.p1.length, 1.0, 350.0)
        .top_left_with_margins(80.0, 20.0)
        .label("Point 1 - Distance")
        .w_h(300.0, 25.0)
        .label_font_size(13)
        .rgb(1.0, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(p.s4, ui)
    {
        p.p1.length = value;
    }

    for value in widget::Slider::new(p.m2, 1.0, 100.0)
        .top_left_with_margins(110.0, 20.0)
        .label("Point 2 - Mass")
        .w_h(300.0, 25.0)
        .label_font_size(13)
        .rgb(1.0, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(p.s3, ui)
    {
        p.m2 = value;
    }

    for value in widget::Slider::new(p.p2.length, 1.0, 350.0)
        .top_left_with_margins(140.0, 20.0)
        .label("Point 2 - Distance")
        .w_h(300.0, 25.0)
        .label_font_size(13)
        .rgb(1.0, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(p.s5, ui)
    {
        p.p2.length = value;
    }
}

fn view(app: &App, p: &Point, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    // Since all positions are relative the absolute coordinates of each point have to be calculated.
    
    let base_x: f64 = 0.0;
    let base_y: f64 = 0.0;

    let mut p1 = p.p1;
    p1.angle += PI as f64;
    let x1: f64 = base_x + (p1.angle.sin() * p1.length);
    let y1: f64 = base_y + (p1.angle.cos() * p1.length);

    draw.ellipse().x_y(x1 as f32, y1 as f32).w(p.m1 as f32).h(p.m1 as f32).color(BLACK);

    let s = pt2(base_x as f32, base_y as f32);
    let e = pt2(x1 as f32, y1 as f32);

    draw.line().start(s).end(e).color(BLACK);

    let mut p2 = p.p2;
    p2.angle += PI as f64;
    let x2 = x1 + (p2.angle.sin() * p2.length);
    let y2 = y1 + (p2.angle.cos() * p2.length);

    draw.ellipse().x_y(x2 as f32, y2 as f32).w(p.m2 as f32).h(p.m2 as f32).color(BLACK);

    let e2 = pt2(x2 as f32, y2 as f32);

    draw.line().start(e).end(e2).color(BLACK);


    // For each point in the history the positions are calculated and connected using lines.
    // This is not the most efficient method since for each frame all calculated are redone.
    for (pos, _) in p.history.iter().enumerate() {
        if pos != 0 {
            let (mut curr1, mut curr2) = p.history[pos];
            let (mut last1, mut last2) = p.history[pos - 1];

            curr1.angle += PI as f64;
            curr2.angle += PI as f64;
            last1.angle += PI as f64;
            last2.angle += PI as f64;

            let curr_x1 = base_x + curr1.angle.sin() * curr1.length;
            let curr_y1 = base_y + curr1.angle.cos() * curr1.length;

            let curr_x2 = curr_x1 + (curr2.angle.sin() * curr2.length);
            let curr_y2 = curr_y1 + (curr2.angle.cos() * curr2.length);

            let last_x1 = base_x + last1.angle.sin() * last1.length;
            let last_y1 = base_y + last1.angle.cos() * last1.length;

            let last_y2 = last_y1 + (last2.angle.cos() * last2.length);
            let last_x2 = last_x1 + (last2.angle.sin() * last2.length);

            let s1 = pt2(curr_x1 as f32, curr_y1 as f32);
            let e1 = pt2(last_x1 as f32, last_y1 as f32);

            let length = p.history.len() as f32;
            let o = pos as f32 / (length * 3.0);

            draw.line().start(s1).end(e1).stroke_weight(o).color(BLUE);

            let s2 = pt2(curr_x2 as f32, curr_y2 as f32);
            let e2 = pt2(last_x2 as f32, last_y2 as f32);

            let length = p.history.len() as f32;
            let o = pos as f32 / length;

            draw.line().start(s2).end(e2).stroke_weight(o).color(RED);
        }
    }

    draw.to_frame(app, &frame).unwrap();

    // Draw the UI to the frame.
    p.ui.draw_to_frame(app, &frame).unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct XY {
    x: f64,
    y: f64,
}

impl XY {
    pub fn new(x: f64, y: f64) -> Self {
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
    pub length: f64,
    pub angle: f64,
}

impl Polar {
    pub fn new(length: f64, angle: f64) -> Self {
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
