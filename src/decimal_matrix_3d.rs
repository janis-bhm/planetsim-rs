use crate::decimal_vector_3d::DecimalVector3d;
use crate::sin_cos::{cos, f64_to_dbig, sin};
use dashu_float::ops::SquareRoot;
use dashu_float::DBig;
use std::sync::LazyLock;

static DBIGHALF: LazyLock<DBig> = LazyLock::new(|| f64_to_dbig(0.5));

#[derive(Debug, Clone)]
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
        let c = &cos(-angle.clone(), 32);
        let s = &sin(-angle.clone(), 32);
        let one_minus_c = &(DBig::ONE.clone() - c);
        DecimalMatrix3d {
            data: [
                [
                    one_minus_c * (&axis.x) * (&axis.x) + (c),
                    one_minus_c * (&axis.x) * (&axis.y) - (&axis.z * (s)),
                    one_minus_c * (&axis.z) * (&axis.x) + (&axis.y * (s)),
                ],
                [
                    one_minus_c * (&axis.x) * (&axis.y) + (&axis.z * (s)),
                    one_minus_c * (&axis.y) * (&axis.y) + (c),
                    one_minus_c * (&axis.y) * (&axis.z) - (&axis.x * (s)),
                ],
                [
                    one_minus_c * (&axis.z) * (&axis.x) - (&axis.y * (s)),
                    one_minus_c * (&axis.y) * (&axis.z) + (&axis.x * (s)),
                    one_minus_c * (&axis.z) * (&axis.z) + (c),
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

    pub fn as_quat(&self) -> [DBig; 4] {
        let f_trace = self.data[0][0].clone() + self.data[1][1].clone() + self.data[2][2].clone();
        let mut f_root = DBig::ZERO.clone();

        if (f_trace > DBig::ZERO) {
            f_root = (f_trace + DBig::ONE.clone()).sqrt();
            let w = DBIGHALF.clone() * f_root.clone();
            f_root = DBIGHALF.clone() / f_root.clone();
            let x = (self.data[1][2].clone() - self.data[2][1].clone()) * f_root.clone();
            let y = (self.data[2][0].clone() - self.data[0][2].clone()) * f_root.clone();
            let z = (self.data[0][1].clone() - self.data[1][0].clone()) * f_root.clone();
            [x, y, z, w]
        } else {
            let mut i = 0;
            if (self.data[1][1] > self.data[0][0]) {
                i = 1;
            }
            if (self.data[2][2] > self.data[i][i]) {
                i = 2;
            }
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;

            f_root = (self.data[i][i].clone() - self.data[j][j].clone() - self.data[k][k].clone()
                + DBig::ONE.clone())
            .sqrt();
            let mut out = [DBig::ZERO, DBig::ZERO, DBig::ZERO, DBig::ZERO];
            out[i] = DBIGHALF.clone() * f_root.clone();
            f_root = DBIGHALF.clone() / f_root.clone();
            out[3] = (self.data[j][k].clone() - self.data[k][j].clone()) * f_root.clone();
            out[j] = (self.data[j][i].clone() + self.data[i][j].clone()) * f_root.clone();
            out[k] = (self.data[k][i].clone() + self.data[i][k].clone()) * f_root.clone();
            out
        }
    }
}
