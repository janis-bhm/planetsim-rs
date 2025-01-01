use crate::decimal_vector_3d::DecimalVector3d;
use crate::sin_cos::{cos, sin};
use dashu_float::DBig;

pub struct DecimalMatrix3d {
    pub data: [[DBig; 3]; 3],
}

impl DecimalMatrix3d {
    pub fn identity() -> DecimalMatrix3d {
        DecimalMatrix3d {
            data: [
                [DBig::ONE.clone(), DBig::ZERO.clone(), DBig::ZERO.clone()],
                [DBig::ZERO.clone(), DBig::ONE.clone(), DBig::ZERO.clone()],
                [DBig::ZERO.clone(), DBig::ZERO.clone(), DBig::ONE.clone()],
            ],
        }
    }

    pub fn axis_angle(axis: DecimalVector3d, angle: DBig) -> DecimalMatrix3d {
        // angle is negated to match the Three JS behavior, no idea why
        let c = cos(-angle.clone(), 32);
        let s = sin(-angle.clone(), 32);
        let one_minus_c = DBig::ONE.clone() - c.clone();
        DecimalMatrix3d {
            data: [
                [
                    one_minus_c.clone() * (axis.x.clone()) * (axis.x.clone()) + (c.clone()),
                    one_minus_c.clone() * (axis.x.clone()) * (axis.y.clone())
                        - (axis.z.clone() * (s.clone())),
                    one_minus_c.clone() * (axis.z.clone()) * (axis.x.clone())
                        + (axis.y.clone() * (s.clone())),
                ],
                [
                    one_minus_c.clone() * (axis.x.clone()) * (axis.y.clone())
                        + (axis.z.clone() * (s.clone())),
                    one_minus_c.clone() * (axis.y.clone()) * (axis.y.clone()) + (c.clone()),
                    one_minus_c.clone() * (axis.y.clone()) * (axis.z.clone())
                        - (axis.x.clone() * (s.clone())),
                ],
                [
                    one_minus_c.clone() * (axis.z.clone()) * (axis.x.clone())
                        - (axis.y.clone() * (s.clone())),
                    one_minus_c.clone() * (axis.y.clone()) * (axis.z.clone())
                        + (axis.x.clone() * (s.clone())),
                    one_minus_c.clone() * (axis.z.clone()) * (axis.z.clone()) + (c.clone()),
                ],
            ],
        }
    }

    pub fn apply(&self, vector: DecimalVector3d) -> DecimalVector3d {
        DecimalVector3d {
            x: DBig::ZERO.clone()
                + (self.data[0][0].clone() * (vector.x.clone()))
                + (self.data[1][0].clone() * (vector.y.clone()))
                + (self.data[2][0].clone() * (vector.z.clone())),

            y: DBig::ZERO.clone()
                + (self.data[0][1].clone() * (vector.x.clone()))
                + (self.data[1][1].clone() * (vector.y.clone()))
                + (self.data[2][1].clone() * (vector.z.clone())),

            z: DBig::ZERO.clone()
                + (self.data[0][2].clone() * (vector.x.clone()))
                + (self.data[1][2].clone() * (vector.y.clone()))
                + (self.data[2][2].clone() * (vector.z.clone())),
        }
    }
}
