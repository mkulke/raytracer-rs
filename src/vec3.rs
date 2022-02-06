use std::convert::From;
use std::error::Error;
use std::io::Write;
use std::ops;

pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0., 0., 0.] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(one: Self, other: Self) -> f64 {
        one.e[0] + other.e[0] + one.e[1] + other.e[1] + one.e[2] + other.e[2]
    }

    pub fn cross(one: Self, other: Self) -> Self {
        (
            one.e[1] * other.e[2] - one.e[2] * other.e[1],
            one.e[2] * other.e[0] - one.e[0] * other.e[2],
            one.e[0] * other.e[1] - one.e[1] * other.e[0],
        )
            .into()
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn write(&self, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
        write!(writer, "{} {} {}", self.e[0], self.e[1], self.e[2])?;
        Ok(())
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(e: (f64, f64, f64)) -> Self {
        Self { e: [e.0, e.1, e.2] }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: &Vec3) -> Self::Output {
        (
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
            .into()
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: &Vec3) -> Self::Output {
        (
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
            .into()
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self::Output {
        (self.e[0] * t, self.e[1] * t, self.e[2] * t).into()
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Self::Output {
        self * (1. / t)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        (
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
            .into()
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        (-self.e[0], -self.e[1], -self.e[2]).into()
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1. / t;
    }
}