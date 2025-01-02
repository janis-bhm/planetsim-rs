use dashu_float::ops::SquareRoot;
use dashu_float::DBig;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
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

// ADD
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty; $v:ty) => {
        impl $imp<$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                <$t>::new(
                    $imp::$method(&self.x, other.x),
                    $imp::$method(&self.y, other.y),
                    $imp::$method(&self.z, other.z),
                )
            }
        }
        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                <$t>::new(
                    $imp::$method(self.x, &other.x),
                    $imp::$method(self.y, &other.y),
                    $imp::$method(self.z, &other.z),
                )
            }
        }
        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                <$t>::new(
                    $imp::$method(&self.x, &other.x),
                    $imp::$method(&self.y, &other.y),
                    $imp::$method(&self.z, &other.z),
                )
            }
        }

        // Component
        impl $imp<$v> for &$t {
            type Output = <$t as $imp<$v>>::Output;

            fn $method(self, other: $v) -> <$t as $imp<$v>>::Output {
                <$t>::new(
                    $imp::$method(&self.x, &other),
                    $imp::$method(&self.y, &other),
                    $imp::$method(&self.z, &other),
                )
            }
        }
        impl $imp<&$v> for $t {
            type Output = <$t as $imp<$v>>::Output;

            fn $method(self, other: &$v) -> <$t as $imp<$v>>::Output {
                <$t>::new(
                    $imp::$method(self.x, other),
                    $imp::$method(self.y, other),
                    $imp::$method(self.z, other),
                )
            }
        }
        impl $imp<&$v> for &$t {
            type Output = <$t as $imp<$v>>::Output;

            fn $method(self, other: &$v) -> <$t as $imp<$v>>::Output {
                <$t>::new(
                    $imp::$method(&self.x, other),
                    $imp::$method(&self.y, other),
                    $imp::$method(&self.z, other),
                )
            }
        }
    };
}

macro_rules! impl_binop {
    (impl $imp:ident, $method:ident for $t:ty, $v:ty) => {
        impl $imp<$t> for $t {
            type Output = $t;

            fn $method(self, other: $t) -> Self::Output {
                Self::new(
                    $imp::$method(self.x, other.x),
                    $imp::$method(self.y, other.y),
                    $imp::$method(self.z, other.z),
                )
            }
        }
        impl $imp<$v> for $t {
            type Output = $t;

            fn $method(self, other: $v) -> Self::Output {
                Self::new(
                    $imp::$method(self.x, &other),
                    $imp::$method(self.y, &other),
                    $imp::$method(self.z, &other),
                )
            }
        }

        forward_ref_binop!(impl $imp, $method for $t, $t; $v);
    };
}

macro_rules! impl_binop_assign {
    (impl $imp:ident, $method:ident for $t:ty, $v:ty) => {
        // *Assign<Component>
        impl $imp<$v> for $t {
            fn $method(&mut self, other: $v) {
                $imp::$method(&mut self.x, &other);
                $imp::$method(&mut self.y, &other);
                $imp::$method(&mut self.z, &other);
            }
        }
        impl $imp<&$v> for $t {
            fn $method(&mut self, other: &$v) {
                $imp::$method(&mut self.x, other);
                $imp::$method(&mut self.y, other);
                $imp::$method(&mut self.z, other);
            }
        }
        // *Assign<Self>
        impl $imp<$t> for $t {
            fn $method(&mut self, other: $t) {
                $imp::$method(&mut self.x, &other.x);
                $imp::$method(&mut self.y, &other.y);
                $imp::$method(&mut self.z, &other.z);
            }
        }
        impl $imp<&$t> for $t {
            fn $method(&mut self, other: &$t) {
                $imp::$method(&mut self.x, &other.x);
                $imp::$method(&mut self.y, &other.y);
                $imp::$method(&mut self.z, &other.z);
            }
        }
    };
}

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
impl_binop!(impl Add, add for DecimalVector3d, DBig);
impl_binop_assign!(impl AddAssign, add_assign for DecimalVector3d, DBig);

impl_binop!(impl Sub, sub for DecimalVector3d, DBig);
impl_binop_assign!(impl SubAssign, sub_assign for DecimalVector3d, DBig);

impl_binop!(impl Mul, mul for DecimalVector3d, DBig);
impl_binop_assign!(impl MulAssign, mul_assign for DecimalVector3d, DBig);

impl_binop!(impl Div, div for DecimalVector3d, DBig);
impl_binop_assign!(impl DivAssign, div_assign for DecimalVector3d, DBig);
