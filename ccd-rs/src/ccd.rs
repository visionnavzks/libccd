use crate::vec3::Vec3;

pub type SupportFn<T> = fn(obj: &T, dir: &Vec3) -> Vec3;
pub type CenterFn<T> = fn(obj: &T) -> Vec3;
pub type FirstDirFn<T> = fn(obj1: &T, obj2: &T) -> Vec3;

#[derive(Debug)]
pub struct CCD<T> {
    pub first_dir: FirstDirFn<T>,
    pub support1: SupportFn<T>,
    pub support2: SupportFn<T>,
    pub center1: CenterFn<T>,
    pub center2: CenterFn<T>,
    pub max_iterations: u64,
    pub epa_tolerance: f64,
    pub mpr_tolerance: f64,
    pub dist_tolerance: f64,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for CCD<T> {
    fn default() -> Self {
        CCD {
            first_dir: first_dir_default,
            support1: |_, _| Vec3::new(0.0, 0.0, 0.0),
            support2: |_, _| Vec3::new(0.0, 0.0, 0.0),
            center1: |_| Vec3::new(0.0, 0.0, 0.0),
            center2: |_| Vec3::new(0.0, 0.0, 0.0),
            max_iterations: u64::MAX,
            epa_tolerance: 0.0001,
            mpr_tolerance: 0.0001,
            dist_tolerance: 1e-6,
            _marker: std::marker::PhantomData,
        }
    }
}

pub fn first_dir_default<T>(_o1: &T, _o2: &T) -> Vec3 {
    Vec3::new(1.0, 0.0, 0.0)
}
