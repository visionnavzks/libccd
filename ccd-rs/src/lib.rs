pub use vec3::Vec3;
pub use ccd::{CCD, SupportFn, CenterFn, FirstDirFn};
pub use gjk::gjk_intersect;
pub use mpr::mpr_intersect;

mod vec3;
mod ccd;
mod gjk;
mod mpr;
mod support;

pub const EPS: f64 = std::f64::EPSILON;
pub const REAL_MAX: f64 = std::f64::MAX;
