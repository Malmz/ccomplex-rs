use std::ops;
use std::fmt;

use super::iprimitive::I;

trait Round5 {
    fn round5(&self) -> f64;
}

impl Round5 for f64 {
    fn round5(&self) -> f64 {
        (self * 100000.0).round() / 100000.0
    }
}
/// `Z(A, B)`   
///
/// `A + Bi`
///
/// Complex number.
#[derive(Copy, Clone)]
pub struct Z(pub f64, pub f64);

impl Z { 
    pub fn to_pol(&self) -> ZP {
        ZP(self.0.hypot(self.1), (self.1 / self.0).atan())
    }

	pub fn conjungate(&self) -> Z {
		Z(self.0, -self.1)
	}
}

//------------
// Addition
//------------

impl ops::Add for Z {
    type Output = Z;
    fn add(self, rhs: Z) -> Z {
        Z(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Add<f64> for Z {
    type Output = Z;
    fn add(self, rhs: f64) -> Z {
        Z(self.0 + rhs, self.1)
    }
}

impl ops::Add<Z> for f64 {
    type Output = Z;
    fn add(self, rhs: Z) -> Z {
        Z(self + rhs.0, rhs.1)
    }
}

impl ops::Add<I> for Z {
    type Output = Z;
    fn add(self, rhs: I) -> Z {
        Z(self.0, self.1 + rhs.0)
    }
}

impl ops::Add<Z> for I {
    type Output = Z;
    fn add(self, rhs: Z) -> Z {
        Z(rhs.0, self.0 + rhs.1)
    }
}

//---------------
// Subtraction
//---------------

impl ops::Sub for Z {
    type Output = Z;
    fn sub(self, rhs: Z) -> Z {
        Z(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Sub<f64> for Z {
    type Output = Z;
    fn sub(self, rhs: f64) -> Z {
        Z(self.0 - rhs, self.1)
    }
}

impl ops::Sub<Z> for f64 {
    type Output = Z;
    fn sub(self, rhs: Z) -> Z {
        Z(self - rhs.0, -rhs.1)
    }
}

impl ops::Sub<I> for Z {
    type Output = Z;
    fn sub(self, rhs: I) -> Z {
        Z(self.0, self.1 - rhs.0)
    }
}

impl ops::Sub<Z> for I {
    type Output = Z;
    fn sub(self, rhs: Z) -> Z {
        Z(-rhs.0, self.0 - rhs.1)
    }
}

//------------------
// Multiplication
//------------------

impl ops::Mul for Z {
    type Output = Z;
    fn mul(self, rhs: Z) -> Z {
        //(self.to_pol() * rhs.to_pol()).to_rect()
		self.0 * rhs.0 + I(self.1) * rhs.0 + self.0 * I(rhs.1) + I(self.1) * I(rhs.1)
    }
}

impl ops::Mul<f64> for Z {
    type Output = Z;
    fn mul(self, rhs: f64) -> Z {
		Z(self.0 * rhs, self.1 * rhs)
    }
}

impl ops::Mul<Z> for f64 {
    type Output = Z;
    fn mul(self, rhs: Z) -> Z {
		Z(self * rhs.0, self * rhs.1)
    }
}

impl ops::Mul<I> for Z {
    type Output = Z;
    fn mul(self, rhs: I) -> Z {
		Z(-(self.1 * rhs.0), self.0 * rhs.0)
    }
}

impl ops::Mul<Z> for I {
    type Output = Z;
    fn mul(self, rhs: Z) -> Z {
		Z(-(self.0 * rhs.1), self.0 * rhs.0)
    }
}

//------------
// Division
//------------

impl ops::Div for Z {
    type Output = Z;
    fn div(self, rhs: Z) -> Z {
        //(self.to_pol() / rhs.to_pol()).to_rect()
		self * rhs.conjungate() / (rhs * rhs.conjungate()).0
    }
}

impl ops::Div<f64> for Z {
    type Output = Z;
    fn div(self, rhs: f64) -> Z {
        Z(self.0 / rhs, self.1 / rhs)
    }
}

impl ops::Div<Z> for f64 {
    type Output = Z;
    fn div(self, rhs: Z) -> Z {
        Z(rhs.0 / self, rhs.1 / self)
    }
}

impl ops::Div<I> for Z {
    type Output = Z;
    fn div(self, rhs: I) -> Z {
        Z(-(self.1 / rhs.0), self.0 / rhs.0)
    }
}

impl ops::Div<Z> for I {
    type Output = Z;
    fn div(self, rhs: Z) -> Z {
        Z(0.0, rhs.0) / self
    }
}

impl fmt::Display for Z {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", 
		if self.0 != 0.0 {self.0.to_string()} else {"".to_string()}, 
		if self.1 > 0.0 && self.0 != 0.0 {"+"} else {""}, 
		I(self.1))
    }
}

/// `ZP(Length, Angle)`
///
/// `Length (cos Angle + i * sin Angle)`
///
/// Complex number stored as a vector. (Angle is stored in radians)
pub struct ZP(pub f64, pub f64);

impl ZP {
    pub fn to_rect(&self) -> Z {
        Z(self.0 * self.1.cos(), self.0 * self.1.sin())
    }
}

impl ops::Add for ZP {
    type Output = ZP;
    fn add(self, rhs: ZP) -> ZP {
        (self.to_rect() + rhs.to_rect()).to_pol()
    }
}

impl ops::Sub for ZP {
    type Output = ZP;
    fn sub(self, rhs: ZP) -> ZP {
        (self.to_rect() - rhs.to_rect()).to_pol()
    }
}

impl ops::Mul for ZP {
    type Output = ZP;
    fn mul(self, rhs: ZP) -> ZP {
        ZP(self.0 * rhs.0, self.1 + rhs.1)
    }
}

impl ops::Div for ZP {
    type Output = ZP;
    fn div(self, rhs: ZP) -> ZP {
        ZP(self.0 / rhs.0, self.1 - rhs.1)
    }
}

impl fmt::Display for ZP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.1.to_degrees();
        write!(f, "{} * (cos {} + i * sin {})", self.0, v, v)
    }
}