pub type ccd_real_t = ::core::ffi::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_vec3_t {
    pub v: [ccd_real_t; 3],
}
pub type ccd_vec3_t = _ccd_vec3_t;
pub type ccd_support_fn = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_void, *const ccd_vec3_t, *mut ccd_vec3_t) -> (),
>;
pub type ccd_first_dir_fn = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
        *mut ccd_vec3_t,
    ) -> (),
>;
pub type ccd_center_fn =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *mut ccd_vec3_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_t {
    pub first_dir: ccd_first_dir_fn,
    pub support1: ccd_support_fn,
    pub support2: ccd_support_fn,
    pub center1: ccd_center_fn,
    pub center2: ccd_center_fn,
    pub max_iterations: ::core::ffi::c_ulong,
    pub epa_tolerance: ccd_real_t,
    pub mpr_tolerance: ccd_real_t,
    pub dist_tolerance: ccd_real_t,
}
pub type ccd_t = _ccd_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_support_t {
    pub v: ccd_vec3_t,
    pub v1: ccd_vec3_t,
    pub v2: ccd_vec3_t,
}
pub type ccd_support_t = _ccd_support_t;
pub const CCD_ONE: ::core::ffi::c_float = 1.0f32;
#[inline(always)]
unsafe extern "C" fn ccdVec3Copy(mut v: *mut ccd_vec3_t, mut w: *const ccd_vec3_t) {
    *v = *w;
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Sub2(
    mut d: *mut ccd_vec3_t,
    mut v: *const ccd_vec3_t,
    mut w: *const ccd_vec3_t,
) {
    (*d).v[0 as ::core::ffi::c_int as usize] =
        (*v).v[0 as ::core::ffi::c_int as usize] - (*w).v[0 as ::core::ffi::c_int as usize];
    (*d).v[1 as ::core::ffi::c_int as usize] =
        (*v).v[1 as ::core::ffi::c_int as usize] - (*w).v[1 as ::core::ffi::c_int as usize];
    (*d).v[2 as ::core::ffi::c_int as usize] =
        (*v).v[2 as ::core::ffi::c_int as usize] - (*w).v[2 as ::core::ffi::c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Scale(mut d: *mut ccd_vec3_t, mut k: ccd_real_t) {
    (*d).v[0 as ::core::ffi::c_int as usize] *= k;
    (*d).v[1 as ::core::ffi::c_int as usize] *= k;
    (*d).v[2 as ::core::ffi::c_int as usize] *= k;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn __ccdSupport(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut _dir: *const ccd_vec3_t,
    mut ccd: *const ccd_t,
    mut supp: *mut ccd_support_t,
) {
    let mut dir: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    ccdVec3Copy(&raw mut dir, _dir);
    (*ccd).support1.expect("non-null function pointer")(obj1, &raw mut dir, &raw mut (*supp).v1);
    ccdVec3Scale(&raw mut dir, -CCD_ONE);
    (*ccd).support2.expect("non-null function pointer")(obj2, &raw mut dir, &raw mut (*supp).v2);
    ccdVec3Sub2(&raw mut (*supp).v, &raw mut (*supp).v1, &raw mut (*supp).v2);
}
