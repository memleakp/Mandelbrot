// implement the Complex struct and traits below

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex{re: re, im: im}
    }

    pub fn mag(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_re = self.re * rhs.re - self.im * rhs.im;
        let new_im = self.re * rhs.im + self.im * rhs.re;
        Complex::new(new_re, new_im)
    }
}
