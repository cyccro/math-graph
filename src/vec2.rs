pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn set_x(&mut self, x: f64) {
        self.0 = x;
    }
    pub fn set_y(&mut self, y: f64) {
        self.1 = y;
    }
    pub fn set(&mut self, v: (f64, f64)) {
        self.0 = v.0;
        self.1 = v.1;
    }
    pub fn dot(&self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y()
    }
    pub fn mag_square(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2)
    }
    pub fn mag(&self) -> f64 {
        self.mag_square().sqrt()
    }
    pub fn splat(n: f64) -> Self {
        Self(n, n)
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}
impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}
impl std::ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}
impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}
impl std::ops::MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}
impl std::ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}
impl std::ops::DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}
impl std::ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}
