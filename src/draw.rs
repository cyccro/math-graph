//check explanation files if any trouble to understand the code.
//im not gonna comment
use crate::{rgb::Rgb, vec2::Vec2};
use gtk4::cairo::Context;

static COLUMNS: f64 = 30.0;
static ROWS: f64 = 30.0;

pub struct Pencil<'a>(&'a Context, f64, f64);
struct Line(Vec2, Vec2, Rgb);
type ResultDraw<T = ()> = Result<T, gtk4::cairo::Error>;

impl Line {
    pub fn new(init: Vec2, end: Vec2, color: Rgb) -> Self {
        Self(init, end, color)
    }
    pub fn from_numbers(x: f64, y: f64, xx: f64, yy: f64, r: f64, g: f64, b: f64) -> Self {
        Self(Vec2(x, y), Vec2(xx, yy), Rgb(r, g, b))
    }
    fn draw(&self, pencil: &Pencil) {
        pencil.0.set_source_rgb(self.2.r(), self.2.g(), self.2.b());
        pencil.draw_line(&self.0, &self.1);
    }
}

impl<'a> Pencil<'a> {
    pub fn width(&self) -> f64 {
        self.1
    }
    pub fn height(&self) -> f64 {
        self.2
    }
    pub fn draw_line(&self, from: &Vec2, to: &Vec2) -> ResultDraw {
        self.0.move_to(from.x(), from.y());
        self.0.line_to(to.x(), to.y());
        self.0.stroke()
    }
    pub fn draw_points(&self, points: Vec<Line>) {
        let mut idx = 0;
        while let Some(line) = points.get(idx) {
            idx += 1;
            self.0.set_source_rgb(line.2.r(), line.2.g(), line.2.b());
            self.0.move_to(line.0.x(), line.0.y());
            self.0.line_to(line.1.x(), line.1.y());
            self.0.stroke();
        }
    }
    pub fn draw_graph(&self) -> ResultDraw {
        let ctx = &self.0;
        {
            self.0.set_line_width(2.5);
            let mids = self.mids();
            self.draw_line(&mids[0], &mids[1])?;
            self.draw_line(&mids[2], &mids[3])?;
        }
        ctx.set_line_width(1.0);
        (Rgb::white() / 2.0).set_cairos_color(ctx);
        self.draw_columns()?;
        self.draw_rows()?;
        Ok(())
    }
    fn draw_func(&self, w: f64, h: f64, steps: f64, f: impl Fn(f64) -> f64) -> ResultDraw {
        let step = steps.recip();
        let mut i = -steps;
        let width_step = w / steps;
        let height_step = h / steps;
        let half_height = h / 2.0;
        let half_width = w / 2.0;
        while i <= steps {
            let y = {
                let n = -f(i * height_step) + half_height;
                if n.is_nan() {
                    i += step;
                    continue;
                }
                n
            };
            let new_y = {
                let n = -f((i + step) * height_step) + half_height;
                if n.is_nan() {
                    i += step;
                    continue;
                }
                n
            };
            let init = Vec2(i * width_step + half_width, y);
            let end = Vec2((i + step) * width_step + half_width, new_y);
            Line::new(init, end, Rgb::RED).draw(self);
            i += step;
        }
        Ok(())
    }
    fn mids(&self) -> [Vec2; 4] {
        let wh = self.1 / 2.0; //width half
        let hh = self.2 / 2.0; //height half
        [
            Vec2(wh, 0.0),    //Center X top
            Vec2(wh, self.2), //Center X bottom
            Vec2(0.0, hh),    //Center X left
            Vec2(self.1, hh), //Center X right
        ]
    }
    fn draw_columns(&self) -> ResultDraw {
        let step = self.1 / COLUMNS;
        let mut from = Vec2(0.0, 0.0);
        let mut to = Vec2(0.0, self.2);
        for i in 0..ROWS as i64 {
            let x = step * i as f64;
            from.set_x(x);
            to.set_x(x);
            self.draw_line(&from, &to)?;
        }
        Ok(())
    }
    fn draw_rows(&self) -> ResultDraw {
        let step = self.1 / ROWS;
        let mut from = Vec2(0.0, 0.0);
        let mut to = Vec2(self.1, 0.0);
        for i in 0..ROWS as i64 {
            let y = step * i as f64;
            from.set_y(y);
            to.set_y(y);
            self.draw_line(&from, &to)?;
        }
        Ok(())
    }
}

pub fn start_draw(ctx: &Context, w: i32, h: i32) -> ResultDraw {
    let pencil = Pencil(ctx, w as f64, h as f64);
    pencil.draw_graph()?;
    pencil.draw_func(w as f64, h as f64, 200.0, |x| 30.0 * (x / 10.0).ln())?;
    Ok(())
}
