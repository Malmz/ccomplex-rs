use std::ops;
use std::fmt;
use super::zprimitive::Z;

pub struct I(pub f64);

impl ops::Add for I {
	type Output = I;
	fn add(self, rhs: I) -> I {
		I(self.0 + rhs.0)
	}
}

impl ops::Add<f64> for I {
	type Output = Z;
	fn add(self, rhs: f64) -> Z {
		Z(rhs, self.0)
	}
}

impl ops::Add<I> for f64 {
	type Output = Z;
	fn add(self, rhs: I) -> Z {
		Z(self, rhs.0)
	}
}

impl ops::Sub for I {
	type Output = I;
	fn sub(self, rhs: I) -> I {
		I(self.0 - rhs.0)
	}
}

impl ops::Sub<f64> for I{
	type Output = Z;
	fn sub(self, rhs: f64) -> Z {
		Z(-rhs, self.0)
	}
}

impl ops::Sub<I> for f64 {
	type Output = Z;
	fn sub(self, rhs: I) -> Z {
		Z(self, -rhs.0)
	}
}

impl ops::Mul for I {
	type Output = f64;
	fn mul(self, rhs: I) -> f64 {
		-(self.0 * rhs.0)
	}
}

impl ops::Mul<f64> for I {
	type Output = I;
	fn mul(self, rhs: f64) -> I {
		I(self.0 * rhs)
	}
}

impl ops::Mul<I> for f64 {
	type Output = I;
	fn mul(self, rhs: I) -> I {
		I(self * rhs.0)
	}
}

impl ops::Div for I {
	type Output = f64;
	fn div(self, rhs: I) -> f64 {
		self.0 / rhs.0
	}
}

impl ops::Div<f64> for I {
	type Output = I;
	fn div(self, rhs: f64) -> I {
		I(self.0 / rhs)
	}
}

impl ops::Div<I> for f64 {
	type Output = I;
	fn div(self, rhs: I) -> I {
		I(self / rhs.0)
	}
}

impl fmt::Display for I {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.0 != 0.0 {format!("{}i", self.0)} else {"".to_string()})
    }
}