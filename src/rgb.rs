use std::ops::{Add, Div, Mul, Neg, Sub};

use gtk4::cairo::Context;
#[derive(Clone)]
pub struct Rgb(pub f64, pub f64, pub f64);

impl Rgb {
    pub const WHITE: Rgb = Rgb(1.0, 1.0, 1.0);
    pub const BLACK: Rgb = Rgb(0.0, 0.0, 0.0);
    pub const RED: Rgb = Rgb(1.0, 0.0, 0.0);
    pub fn black() -> Self {
        Self(0.0, 0.0, 0.0)
    }
    pub fn white() -> Self {
        Self(1.0, 1.0, 1.0)
    }
    pub fn from_color(color: i64) -> Self {
        let r = (color & 0xff0000) >> 0x8;
        let g = (color & 0x00ff00) >> 0x4;
        let b = (color & 0x0000ff) >> 0x0;
        Self(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }
    pub fn set_cairos_color(&self, ctx: &Context) {
        ctx.set_source_rgb(self.0, self.1, self.2);
    }
    pub fn r(&self) -> f64 {
        self.0
    }
    pub fn g(&self) -> f64 {
        self.1
    }
    pub fn b(&self) -> f64 {
        self.2
    }
}
impl Add<Rgb> for Rgb {
    type Output = Rgb;
    fn add(self, rhs: Rgb) -> Rgb {
        Rgb(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Sub<Rgb> for Rgb {
    type Output = Rgb;
    fn sub(self, rhs: Rgb) -> Rgb {
        Rgb(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Mul<Rgb> for Rgb {
    type Output = Rgb;
    fn mul(self, rhs: Rgb) -> Rgb {
        Rgb(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl Div<Rgb> for Rgb {
    type Output = Rgb;
    fn div(self, rhs: Rgb) -> Rgb {
        Rgb(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Mul<f64> for Rgb {
    type Output = Rgb;
    fn mul(self, rhs: f64) -> Rgb {
        Rgb(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl Div<f64> for Rgb {
    type Output = Rgb;
    fn div(self, rhs: f64) -> Rgb {
        Rgb(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl Neg for Rgb {
    type Output = Rgb;
    fn neg(self) -> Rgb {
        Rgb::WHITE - self
    }
}
