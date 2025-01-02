use dashu_float::ops::SquareRoot;
use dashu_float::DBig;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct DecimalVector3d {
    pub x: DBig,
    pub y: DBig,
    pub z: DBig,
}

impl DecimalVector3d {
    pub fn zero() -> DecimalVector3d {
        DecimalVector3d {
            x: DBig::ZERO.clone(),
            y: DBig::ZERO.clone(),
            z: DBig::ZERO.clone(),
        }
    }

    pub fn new(x: DBig, y: DBig, z: DBig) -> DecimalVector3d {
        DecimalVector3d { x, y, z }
    }

    pub fn assign(&mut self, v: DecimalVector3d) {
        self.x = v.x.clone();
        self.y = v.y.clone();
        self.z = v.z.clone();
    }

    pub fn from_str(x: &str, y: &str, z: &str) -> DecimalVector3d {
        DecimalVector3d {
            x: DBig::from_str(x).unwrap(),
            y: DBig::from_str(y).unwrap(),
            z: DBig::from_str(z).unwrap(),
        }
    }

    pub fn from_f64(x: f64, y: f64, z: f64) -> DecimalVector3d {
        DecimalVector3d {
            x: DBig::from_str(x.to_string().as_str()).unwrap(),
            y: DBig::from_str(y.to_string().as_str()).unwrap(),
            z: DBig::from_str(z.to_string().as_str()).unwrap(),
        }
    }

    pub fn length_squared(&self) -> DBig {
        (self.x.clone() + self.y.clone() + self.z.clone())
    }

    pub fn length(&self) -> DBig {
        (self.x.clone() + self.y.clone() + self.z.clone()).sqrt()
    }

    pub fn distance_to(&self, rhs: Self) -> DBig {
        let difference = self - rhs;
        difference.length()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        *self /= len;
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        self / len
    }

    pub fn dot(&self, rhs: Self) -> DBig {
        self.x.clone() * rhs.x + self.y.clone() * rhs.y + self.z.clone() * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> DecimalVector3d {
        let ax = &self.x;
        let ay = &self.y;
        let az = &self.z;
        let bx = rhs.x;
        let by = rhs.y;
        let bz = rhs.z;

        let x = ay.clone() * bz.clone() - az.clone() * by.clone();
        let y = az.clone() * bx.clone() - ax.clone() * bz.clone();
        let z = ax.clone() * by.clone() - ay.clone() * bx.clone();

        DecimalVector3d { x, y, z }
    }
}

impl fmt::Display for DecimalVector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

impl Clone for DecimalVector3d {
    fn clone(&self) -> Self {
        DecimalVector3d {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

// ADD

impl std::ops::Add<DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<&DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x + rhs.x.clone(),
            y: self.y + rhs.y.clone(),
            z: self.z + rhs.z.clone(),
        }
    }
}

impl std::ops::Add<DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() + rhs.x,
            y: self.y.clone() + rhs.y,
            z: self.z.clone() + rhs.z,
        }
    }
}

impl std::ops::Add<&DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() + rhs.x.clone(),
            y: self.y.clone() + rhs.y.clone(),
            z: self.z.clone() + rhs.z.clone(),
        }
    }
}

impl std::ops::Add<DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x + rhs.clone(),
            y: self.y + rhs.clone(),
            z: self.z + rhs.clone(),
        }
    }
}

impl std::ops::Add<&DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl std::ops::Add<DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() + rhs.clone(),
            y: self.y.clone() + rhs.clone(),
            z: self.z.clone() + rhs.clone(),
        }
    }
}

impl std::ops::Add<&DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn add(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() + rhs.clone(),
            y: self.y.clone() + rhs.clone(),
            z: self.z.clone() + rhs.clone(),
        }
    }
}

impl std::ops::AddAssign<&DBig> for DecimalVector3d {
    fn add_assign(&mut self, rhs: &DBig) {
        self.x += rhs.clone();
        self.y += rhs.clone();
        self.z += rhs.clone();
    }
}

impl std::ops::AddAssign<DBig> for DecimalVector3d {
    fn add_assign(&mut self, rhs: DBig) {
        self.x += rhs.clone();
        self.y += rhs.clone();
        self.z += rhs.clone();
    }
}

// SUB

impl std::ops::Sub<DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<&DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x - rhs.x.clone(),
            y: self.y - rhs.y.clone(),
            z: self.z - rhs.z.clone(),
        }
    }
}

impl std::ops::Sub<DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() - rhs.x,
            y: self.y.clone() - rhs.y,
            z: self.z.clone() - rhs.z,
        }
    }
}

impl std::ops::Sub<&DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() - rhs.x.clone(),
            y: self.y.clone() - rhs.y.clone(),
            z: self.z.clone() - rhs.z.clone(),
        }
    }
}

impl std::ops::Sub<DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x - rhs.clone(),
            y: self.y - rhs.clone(),
            z: self.z - rhs.clone(),
        }
    }
}

impl std::ops::Sub<&DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl std::ops::Sub<DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() - rhs.clone(),
            y: self.y.clone() - rhs.clone(),
            z: self.z.clone() - rhs.clone(),
        }
    }
}

impl std::ops::Sub<&DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn sub(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() - rhs.clone(),
            y: self.y.clone() - rhs.clone(),
            z: self.z.clone() - rhs.clone(),
        }
    }
}

impl std::ops::SubAssign<&DBig> for DecimalVector3d {
    fn sub_assign(&mut self, rhs: &DBig) {
        self.x -= rhs.clone();
        self.y -= rhs.clone();
        self.z -= rhs.clone();
    }
}

impl std::ops::SubAssign<DBig> for DecimalVector3d {
    fn sub_assign(&mut self, rhs: DBig) {
        self.x -= rhs.clone();
        self.y -= rhs.clone();
        self.z -= rhs.clone();
    }
}

// MUL

impl std::ops::Mul<DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<&DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x * rhs.x.clone(),
            y: self.y * rhs.y.clone(),
            z: self.z * rhs.z.clone(),
        }
    }
}

impl std::ops::Mul<DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() * rhs.x,
            y: self.y.clone() * rhs.y,
            z: self.z.clone() * rhs.z,
        }
    }
}

impl std::ops::Mul<&DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() * rhs.x.clone(),
            y: self.y.clone() * rhs.y.clone(),
            z: self.z.clone() * rhs.z.clone(),
        }
    }
}

impl std::ops::Mul<DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs.clone(),
        }
    }
}

impl std::ops::Mul<&DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() * rhs.clone(),
            y: self.y.clone() * rhs.clone(),
            z: self.z.clone() * rhs.clone(),
        }
    }
}

impl std::ops::Mul<&DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn mul(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() * rhs.clone(),
            y: self.y.clone() * rhs.clone(),
            z: self.z.clone() * rhs.clone(),
        }
    }
}

impl std::ops::MulAssign<&DBig> for DecimalVector3d {
    fn mul_assign(&mut self, rhs: &DBig) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs.clone();
    }
}

impl std::ops::MulAssign<DBig> for DecimalVector3d {
    fn mul_assign(&mut self, rhs: DBig) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs.clone();
    }
}

// DIV

impl std::ops::Div<DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Div<&DecimalVector3d> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x / rhs.x.clone(),
            y: self.y / rhs.y.clone(),
            z: self.z / rhs.z.clone(),
        }
    }
}

impl std::ops::Div<DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() / rhs.x,
            y: self.y.clone() / rhs.y,
            z: self.z.clone() / rhs.z,
        }
    }
}

impl std::ops::Div<&DecimalVector3d> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: &DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() / rhs.x.clone(),
            y: self.y.clone() / rhs.y.clone(),
            z: self.z.clone() / rhs.z.clone(),
        }
    }
}

impl std::ops::Div<DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
            z: self.z / rhs.clone(),
        }
    }
}

impl std::ops::Div<&DBig> for DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Div<DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() / rhs.clone(),
            y: self.y.clone() / rhs.clone(),
            z: self.z.clone() / rhs.clone(),
        }
    }
}

impl std::ops::Div<&DBig> for &DecimalVector3d {
    type Output = DecimalVector3d;

    fn div(self, rhs: &DBig) -> DecimalVector3d {
        DecimalVector3d {
            x: self.x.clone() / rhs.clone(),
            y: self.y.clone() / rhs.clone(),
            z: self.z.clone() / rhs.clone(),
        }
    }
}

impl std::ops::DivAssign<&DBig> for DecimalVector3d {
    fn div_assign(&mut self, rhs: &DBig) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs.clone();
    }
}

impl std::ops::DivAssign<DBig> for DecimalVector3d {
    fn div_assign(&mut self, rhs: DBig) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs.clone();
    }
}
