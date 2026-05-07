unsafe extern "C" {
    fn fabsf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
}
pub type size_t = usize;
pub type ccd_real_t = ::core::ffi::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_vec3_t {
    pub v: [ccd_real_t; 3],
}
pub type ccd_vec3_t = _ccd_vec3_t;
pub const CCD_EPS: ::core::ffi::c_float = FLT_EPSILON;
pub const CCD_ONE: ::core::ffi::c_float = 1.0f32;
pub const CCD_ZERO: ::core::ffi::c_float = 0.0f32;
#[inline(always)]
pub unsafe extern "C" fn ccdIsZero(mut val: ccd_real_t) -> ::core::ffi::c_int {
    return (fabsf(val as ::core::ffi::c_float) < CCD_EPS) as ::core::ffi::c_int;
}
#[inline(always)]
pub unsafe extern "C" fn ccdEq(mut _a: ccd_real_t, mut _b: ccd_real_t) -> ::core::ffi::c_int {
    let mut ab: ccd_real_t = 0.;
    let mut a: ccd_real_t = 0.;
    let mut b: ccd_real_t = 0.;
    ab = fabsf(_a as ::core::ffi::c_float - _b as ::core::ffi::c_float) as ccd_real_t;
    if fabsf(ab as ::core::ffi::c_float) < CCD_EPS {
        return 1 as ::core::ffi::c_int;
    }
    a = fabsf(_a as ::core::ffi::c_float) as ccd_real_t;
    b = fabsf(_b as ::core::ffi::c_float) as ccd_real_t;
    if b > a {
        return (ab < CCD_EPS * b) as ::core::ffi::c_int;
    } else {
        return (ab < CCD_EPS * a) as ::core::ffi::c_int;
    };
}
#[inline(always)]
pub unsafe extern "C" fn ccdVec3Len2(mut v: *const ccd_vec3_t) -> ccd_real_t {
    return ccdVec3Dot(v, v);
}
#[inline(always)]
pub unsafe extern "C" fn ccdVec3Dist2(
    mut a: *const ccd_vec3_t,
    mut b: *const ccd_vec3_t,
) -> ccd_real_t {
    let mut ab: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    ccdVec3Sub2(&raw mut ab, a, b);
    return ccdVec3Len2(&raw mut ab);
}
#[inline(always)]
pub unsafe extern "C" fn ccdVec3Copy(mut v: *mut ccd_vec3_t, mut w: *const ccd_vec3_t) {
    *v = *w;
}
#[inline(always)]
pub unsafe extern "C" fn ccdVec3Sub2(
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
pub unsafe extern "C" fn ccdVec3Add(mut v: *mut ccd_vec3_t, mut w: *const ccd_vec3_t) {
    (*v).v[0 as ::core::ffi::c_int as usize] += (*w).v[0 as ::core::ffi::c_int as usize];
    (*v).v[1 as ::core::ffi::c_int as usize] += (*w).v[1 as ::core::ffi::c_int as usize];
    (*v).v[2 as ::core::ffi::c_int as usize] += (*w).v[2 as ::core::ffi::c_int as usize];
}
#[inline(always)]
pub unsafe extern "C" fn ccdVec3Scale(mut d: *mut ccd_vec3_t, mut k: ccd_real_t) {
    (*d).v[0 as ::core::ffi::c_int as usize] *= k;
    (*d).v[1 as ::core::ffi::c_int as usize] *= k;
    (*d).v[2 as ::core::ffi::c_int as usize] *= k;
}
#[inline(always)]
pub unsafe extern "C" fn ccdVec3Dot(mut a: *const ccd_vec3_t, mut b: *const ccd_vec3_t) -> ccd_real_t {
    let mut dot: ccd_real_t = 0.;
    dot = (*a).v[0 as ::core::ffi::c_int as usize] * (*b).v[0 as ::core::ffi::c_int as usize];
    dot += (*a).v[1 as ::core::ffi::c_int as usize] * (*b).v[1 as ::core::ffi::c_int as usize];
    dot += (*a).v[2 as ::core::ffi::c_int as usize] * (*b).v[2 as ::core::ffi::c_int as usize];
    return dot;
}
static mut __ccd_vec3_origin: ccd_vec3_t = _ccd_vec3_t {
    v: [0.0f32, 0.0f32, 0.0f32],
};
#[unsafe(no_mangle)]


pub static mut ccd_vec3_origin: *mut ccd_vec3_t =
    unsafe { &raw const __ccd_vec3_origin as *mut ccd_vec3_t };
static mut points_on_sphere: [ccd_vec3_t; 42] = [
    _ccd_vec3_t {
        v: [0.000000f32, -0.000000f32, -1.000000f32],
    },
    _ccd_vec3_t {
        v: [0.723608f32, -0.525725f32, -0.447219f32],
    },
    _ccd_vec3_t {
        v: [-0.276388f32, -0.850649f32, -0.447219f32],
    },
    _ccd_vec3_t {
        v: [-0.894426f32, -0.000000f32, -0.447216f32],
    },
    _ccd_vec3_t {
        v: [-0.276388f32, 0.850649f32, -0.447220f32],
    },
    _ccd_vec3_t {
        v: [0.723608f32, 0.525725f32, -0.447219f32],
    },
    _ccd_vec3_t {
        v: [0.276388f32, -0.850649f32, 0.447220f32],
    },
    _ccd_vec3_t {
        v: [-0.723608f32, -0.525725f32, 0.447219f32],
    },
    _ccd_vec3_t {
        v: [-0.723608f32, 0.525725f32, 0.447219f32],
    },
    _ccd_vec3_t {
        v: [0.276388f32, 0.850649f32, 0.447219f32],
    },
    _ccd_vec3_t {
        v: [0.894426f32, 0.000000f32, 0.447216f32],
    },
    _ccd_vec3_t {
        v: [-0.000000f32, 0.000000f32, 1.000000f32],
    },
    _ccd_vec3_t {
        v: [0.425323f32, -0.309011f32, -0.850654f32],
    },
    _ccd_vec3_t {
        v: [-0.162456f32, -0.499995f32, -0.850654f32],
    },
    _ccd_vec3_t {
        v: [0.262869f32, -0.809012f32, -0.525738f32],
    },
    _ccd_vec3_t {
        v: [0.425323f32, 0.309011f32, -0.850654f32],
    },
    _ccd_vec3_t {
        v: [0.850648f32, -0.000000f32, -0.525736f32],
    },
    _ccd_vec3_t {
        v: [-0.525730f32, -0.000000f32, -0.850652f32],
    },
    _ccd_vec3_t {
        v: [-0.688190f32, -0.499997f32, -0.525736f32],
    },
    _ccd_vec3_t {
        v: [-0.162456f32, 0.499995f32, -0.850654f32],
    },
    _ccd_vec3_t {
        v: [-0.688190f32, 0.499997f32, -0.525736f32],
    },
    _ccd_vec3_t {
        v: [0.262869f32, 0.809012f32, -0.525738f32],
    },
    _ccd_vec3_t {
        v: [0.951058f32, 0.309013f32, 0.000000f32],
    },
    _ccd_vec3_t {
        v: [0.951058f32, -0.309013f32, 0.000000f32],
    },
    _ccd_vec3_t {
        v: [0.587786f32, -0.809017f32, 0.000000f32],
    },
    _ccd_vec3_t {
        v: [0.000000f32, -1.000000f32, 0.000000f32],
    },
    _ccd_vec3_t {
        v: [-0.587786f32, -0.809017f32, 0.000000f32],
    },
    _ccd_vec3_t {
        v: [-0.951058f32, -0.309013f32, -0.000000f32],
    },
    _ccd_vec3_t {
        v: [-0.951058f32, 0.309013f32, -0.000000f32],
    },
    _ccd_vec3_t {
        v: [-0.587786f32, 0.809017f32, -0.000000f32],
    },
    _ccd_vec3_t {
        v: [-0.000000f32, 1.000000f32, -0.000000f32],
    },
    _ccd_vec3_t {
        v: [0.587786f32, 0.809017f32, -0.000000f32],
    },
    _ccd_vec3_t {
        v: [0.688190f32, -0.499997f32, 0.525736f32],
    },
    _ccd_vec3_t {
        v: [-0.262869f32, -0.809012f32, 0.525738f32],
    },
    _ccd_vec3_t {
        v: [-0.850648f32, 0.000000f32, 0.525736f32],
    },
    _ccd_vec3_t {
        v: [-0.262869f32, 0.809012f32, 0.525738f32],
    },
    _ccd_vec3_t {
        v: [0.688190f32, 0.499997f32, 0.525736f32],
    },
    _ccd_vec3_t {
        v: [0.525730f32, 0.000000f32, 0.850652f32],
    },
    _ccd_vec3_t {
        v: [0.162456f32, -0.499995f32, 0.850654f32],
    },
    _ccd_vec3_t {
        v: [-0.425323f32, -0.309011f32, 0.850654f32],
    },
    _ccd_vec3_t {
        v: [-0.425323f32, 0.309011f32, 0.850654f32],
    },
    _ccd_vec3_t {
        v: [0.162456f32, 0.499995f32, 0.850654f32],
    },
];
#[unsafe(no_mangle)]


pub static mut ccd_points_on_sphere: *mut ccd_vec3_t =
    unsafe { &raw const points_on_sphere as *mut ccd_vec3_t };
#[unsafe(no_mangle)]


pub static mut ccd_points_on_sphere_len: size_t = 0;
#[inline(always)]
pub unsafe extern "C" fn __ccdVec3PointSegmentDist2(
    mut P: *const ccd_vec3_t,
    mut x0: *const ccd_vec3_t,
    mut b: *const ccd_vec3_t,
    mut witness: *mut ccd_vec3_t,
) -> ccd_real_t {
    let mut dist: ccd_real_t = 0.;
    let mut t: ccd_real_t = 0.;
    let mut d: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut a: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    ccdVec3Sub2(&raw mut d, b, x0);
    ccdVec3Sub2(&raw mut a, x0, P);
    t = -1.0f32 * ccdVec3Dot(&raw mut a, &raw mut d);
    t /= ccdVec3Len2(&raw mut d);
    if t < CCD_ZERO || ccdIsZero(t) != 0 {
        dist = ccdVec3Dist2(x0, P);
        if !witness.is_null() {
            ccdVec3Copy(witness, x0);
        }
    } else if t > CCD_ONE || ccdEq(t, CCD_ONE) != 0 {
        dist = ccdVec3Dist2(b, P);
        if !witness.is_null() {
            ccdVec3Copy(witness, b);
        }
    } else if !witness.is_null() {
        ccdVec3Copy(witness, &raw mut d);
        ccdVec3Scale(witness, t);
        ccdVec3Add(witness, x0);
        dist = ccdVec3Dist2(witness, P);
    } else {
        ccdVec3Scale(&raw mut d, t);
        ccdVec3Add(&raw mut d, &raw mut a);
        dist = ccdVec3Len2(&raw mut d);
    }
    return dist;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdVec3PointSegmentDist2(
    mut P: *const ccd_vec3_t,
    mut x0: *const ccd_vec3_t,
    mut b: *const ccd_vec3_t,
    mut witness: *mut ccd_vec3_t,
) -> ccd_real_t {
    return __ccdVec3PointSegmentDist2(P, x0, b, witness);
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdVec3PointTriDist2(
    mut P: *const ccd_vec3_t,
    mut x0: *const ccd_vec3_t,
    mut B: *const ccd_vec3_t,
    mut C: *const ccd_vec3_t,
    mut witness: *mut ccd_vec3_t,
) -> ccd_real_t {
    let mut d1: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut d2: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut a: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut u: ccd_real_t = 0.;
    let mut v: ccd_real_t = 0.;
    let mut w: ccd_real_t = 0.;
    let mut p: ccd_real_t = 0.;
    let mut q: ccd_real_t = 0.;
    let mut r: ccd_real_t = 0.;
    let mut d: ccd_real_t = 0.;
    let mut s: ccd_real_t = 0.;
    let mut t: ccd_real_t = 0.;
    let mut dist: ccd_real_t = 0.;
    let mut dist2: ccd_real_t = 0.;
    let mut witness2: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    ccdVec3Sub2(&raw mut d1, B, x0);
    ccdVec3Sub2(&raw mut d2, C, x0);
    ccdVec3Sub2(&raw mut a, x0, P);
    u = ccdVec3Dot(&raw mut a, &raw mut a);
    v = ccdVec3Dot(&raw mut d1, &raw mut d1);
    w = ccdVec3Dot(&raw mut d2, &raw mut d2);
    p = ccdVec3Dot(&raw mut a, &raw mut d1);
    q = ccdVec3Dot(&raw mut a, &raw mut d2);
    r = ccdVec3Dot(&raw mut d1, &raw mut d2);
    d = w * v - r * r;
    if ccdIsZero(d) != 0 {
        t = -1.0f64 as ccd_real_t;
        s = t;
    } else {
        s = (q * r - w * p) / d;
        t = (-s * r - q) / w;
    }
    if (ccdIsZero(s) != 0 || s > CCD_ZERO)
        && (ccdEq(s, CCD_ONE) != 0 || s < CCD_ONE)
        && (ccdIsZero(t) != 0 || t > CCD_ZERO)
        && (ccdEq(t, CCD_ONE) != 0 || t < CCD_ONE)
        && (ccdEq(t + s, CCD_ONE) != 0 || t + s < CCD_ONE)
    {
        if !witness.is_null() {
            ccdVec3Scale(&raw mut d1, s);
            ccdVec3Scale(&raw mut d2, t);
            ccdVec3Copy(witness, x0);
            ccdVec3Add(witness, &raw mut d1);
            ccdVec3Add(witness, &raw mut d2);
            dist = ccdVec3Dist2(witness, P);
        } else {
            dist = s * s * v;
            dist += t * t * w;
            dist += (2.0f32 * s * t * r) as ::core::ffi::c_float;
            dist += (2.0f32 * s * p) as ::core::ffi::c_float;
            dist += (2.0f32 * t * q) as ::core::ffi::c_float;
            dist += u;
        }
    } else {
        dist = __ccdVec3PointSegmentDist2(P, x0, B, witness);
        dist2 = __ccdVec3PointSegmentDist2(P, x0, C, &raw mut witness2);
        if dist2 < dist {
            dist = dist2;
            if !witness.is_null() {
                ccdVec3Copy(witness, &raw mut witness2);
            }
        }
        dist2 = __ccdVec3PointSegmentDist2(P, B, C, &raw mut witness2);
        if dist2 < dist {
            dist = dist2;
            if !witness.is_null() {
                ccdVec3Copy(witness, &raw mut witness2);
            }
        }
    }
    return dist;
}
pub const FLT_EPSILON: ::core::ffi::c_float = __FLT_EPSILON__;
pub const __FLT_EPSILON__: ::core::ffi::c_float = 1.19209290e-7f32;
pub unsafe extern "C" fn run_static_initializers() {
    ccd_points_on_sphere_len = (::core::mem::size_of::<[ccd_vec3_t; 42]>() as size_t)
        .wrapping_div(::core::mem::size_of::<ccd_vec3_t>() as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
