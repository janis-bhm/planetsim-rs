use crate::sin_cos::f64_to_dbig;
use dashu_float::DBig;
use std::sync::LazyLock;

pub static AU_METERS: LazyLock<DBig> = LazyLock::new(|| f64_to_dbig(149597870691.0));

pub fn au_to_meters(au: DBig) -> DBig {
    au * AU_METERS.clone()
}

pub fn meters_to_au(meters: DBig) -> DBig {
    meters / AU_METERS.clone()
}
