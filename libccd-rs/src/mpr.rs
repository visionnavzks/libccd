unsafe extern "C" {
    fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn fabsf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn fminf(__x: ::core::ffi::c_float, __y: ::core::ffi::c_float) -> ::core::ffi::c_float;
    static mut ccd_vec3_origin: *mut ccd_vec3_t;
    fn ccdVec3PointTriDist2(
        P: *const ccd_vec3_t,
        a: *const ccd_vec3_t,
        b: *const ccd_vec3_t,
        c: *const ccd_vec3_t,
        witness: *mut ccd_vec3_t,
    ) -> ccd_real_t;
    fn __ccdSupport(
        obj1: *const ::core::ffi::c_void,
        obj2: *const ::core::ffi::c_void,
        dir: *const ccd_vec3_t,
        ccd: *const ccd_t,
        supp: *mut ccd_support_t,
    );
}
pub type size_t = usize;
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
pub type ccd_simplex_t = _ccd_simplex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_simplex_t {
    pub ps: [ccd_support_t; 4],
    pub last: ::core::ffi::c_int,
}
pub type ccd_support_t = _ccd_support_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_support_t {
    pub v: ccd_vec3_t,
    pub v1: ccd_vec3_t,
    pub v2: ccd_vec3_t,
}
pub const CCD_EPS: ::core::ffi::c_float = FLT_EPSILON;
pub const CCD_ONE: ::core::ffi::c_float = 1.0f32;
pub const CCD_ZERO: ::core::ffi::c_float = 0.0f32;
#[inline(always)]
unsafe extern "C" fn ccdIsZero(mut val: ccd_real_t) -> ::core::ffi::c_int {
    return (fabsf(val as ::core::ffi::c_float) < CCD_EPS) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdEq(mut _a: ccd_real_t, mut _b: ccd_real_t) -> ::core::ffi::c_int {
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
unsafe extern "C" fn ccdVec3X(mut v: *const ccd_vec3_t) -> ccd_real_t {
    return (*v).v[0 as ::core::ffi::c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Y(mut v: *const ccd_vec3_t) -> ccd_real_t {
    return (*v).v[1 as ::core::ffi::c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Z(mut v: *const ccd_vec3_t) -> ccd_real_t {
    return (*v).v[2 as ::core::ffi::c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Eq(
    mut a: *const ccd_vec3_t,
    mut b: *const ccd_vec3_t,
) -> ::core::ffi::c_int {
    return (ccdEq(ccdVec3X(a), ccdVec3X(b)) != 0
        && ccdEq(ccdVec3Y(a), ccdVec3Y(b)) != 0
        && ccdEq(ccdVec3Z(a), ccdVec3Z(b)) != 0) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Len2(mut v: *const ccd_vec3_t) -> ccd_real_t {
    return ccdVec3Dot(v, v);
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Set(
    mut v: *mut ccd_vec3_t,
    mut x: ccd_real_t,
    mut y: ccd_real_t,
    mut z: ccd_real_t,
) {
    (*v).v[0 as ::core::ffi::c_int as usize] = x;
    (*v).v[1 as ::core::ffi::c_int as usize] = y;
    (*v).v[2 as ::core::ffi::c_int as usize] = z;
}
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
unsafe extern "C" fn ccdVec3Add(mut v: *mut ccd_vec3_t, mut w: *const ccd_vec3_t) {
    (*v).v[0 as ::core::ffi::c_int as usize] += (*w).v[0 as ::core::ffi::c_int as usize];
    (*v).v[1 as ::core::ffi::c_int as usize] += (*w).v[1 as ::core::ffi::c_int as usize];
    (*v).v[2 as ::core::ffi::c_int as usize] += (*w).v[2 as ::core::ffi::c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Scale(mut d: *mut ccd_vec3_t, mut k: ccd_real_t) {
    (*d).v[0 as ::core::ffi::c_int as usize] *= k;
    (*d).v[1 as ::core::ffi::c_int as usize] *= k;
    (*d).v[2 as ::core::ffi::c_int as usize] *= k;
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Normalize(mut d: *mut ccd_vec3_t) {
    let mut k: ccd_real_t = CCD_ONE / sqrtf(ccdVec3Len2(d) as ::core::ffi::c_float) as ccd_real_t;
    ccdVec3Scale(d, k);
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Dot(mut a: *const ccd_vec3_t, mut b: *const ccd_vec3_t) -> ccd_real_t {
    let mut dot: ccd_real_t = 0.;
    dot = (*a).v[0 as ::core::ffi::c_int as usize] * (*b).v[0 as ::core::ffi::c_int as usize];
    dot += (*a).v[1 as ::core::ffi::c_int as usize] * (*b).v[1 as ::core::ffi::c_int as usize];
    dot += (*a).v[2 as ::core::ffi::c_int as usize] * (*b).v[2 as ::core::ffi::c_int as usize];
    return dot;
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Cross(
    mut d: *mut ccd_vec3_t,
    mut a: *const ccd_vec3_t,
    mut b: *const ccd_vec3_t,
) {
    (*d).v[0 as ::core::ffi::c_int as usize] = (*a).v[1 as ::core::ffi::c_int as usize]
        * (*b).v[2 as ::core::ffi::c_int as usize]
        - (*a).v[2 as ::core::ffi::c_int as usize] * (*b).v[1 as ::core::ffi::c_int as usize];
    (*d).v[1 as ::core::ffi::c_int as usize] = (*a).v[2 as ::core::ffi::c_int as usize]
        * (*b).v[0 as ::core::ffi::c_int as usize]
        - (*a).v[0 as ::core::ffi::c_int as usize] * (*b).v[2 as ::core::ffi::c_int as usize];
    (*d).v[2 as ::core::ffi::c_int as usize] = (*a).v[0 as ::core::ffi::c_int as usize]
        * (*b).v[1 as ::core::ffi::c_int as usize]
        - (*a).v[1 as ::core::ffi::c_int as usize] * (*b).v[0 as ::core::ffi::c_int as usize];
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdMPRIntersect(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
) -> ::core::ffi::c_int {
    let mut portal: ccd_simplex_t = _ccd_simplex_t {
        ps: [_ccd_support_t {
            v: _ccd_vec3_t { v: [0.; 3] },
            v1: _ccd_vec3_t { v: [0.; 3] },
            v2: _ccd_vec3_t { v: [0.; 3] },
        }; 4],
        last: 0,
    };
    let mut res: ::core::ffi::c_int = 0;
    res = discoverPortal(obj1, obj2, ccd, &raw mut portal);
    if res < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if res > 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    res = refinePortal(obj1, obj2, ccd, &raw mut portal);
    return if res == 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdMPRPenetration(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut depth: *mut ccd_real_t,
    mut dir: *mut ccd_vec3_t,
    mut pos: *mut ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut portal: ccd_simplex_t = _ccd_simplex_t {
        ps: [_ccd_support_t {
            v: _ccd_vec3_t { v: [0.; 3] },
            v1: _ccd_vec3_t { v: [0.; 3] },
            v2: _ccd_vec3_t { v: [0.; 3] },
        }; 4],
        last: 0,
    };
    let mut res: ::core::ffi::c_int = 0;
    res = discoverPortal(obj1, obj2, ccd, &raw mut portal);
    if res < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    } else if res == 1 as ::core::ffi::c_int {
        findPenetrTouch(obj1, obj2, ccd, &raw mut portal, depth, dir, pos);
    } else if res == 2 as ::core::ffi::c_int {
        findPenetrSegment(obj1, obj2, ccd, &raw mut portal, depth, dir, pos);
    } else if res == 0 as ::core::ffi::c_int {
        res = refinePortal(obj1, obj2, ccd, &raw mut portal);
        if res < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        findPenetr(obj1, obj2, ccd, &raw mut portal, depth, dir, pos);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn findOrigin(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut center: *mut ccd_support_t,
) {
    (*ccd).center1.expect("non-null function pointer")(obj1, &raw mut (*center).v1);
    (*ccd).center2.expect("non-null function pointer")(obj2, &raw mut (*center).v2);
    ccdVec3Sub2(
        &raw mut (*center).v,
        &raw mut (*center).v1,
        &raw mut (*center).v2,
    );
}
unsafe extern "C" fn discoverPortal(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut portal: *mut ccd_simplex_t,
) -> ::core::ffi::c_int {
    let mut dir: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut va: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut vb: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut dot: ccd_real_t = 0.;
    let mut cont: ::core::ffi::c_int = 0;
    findOrigin(
        obj1,
        obj2,
        ccd,
        ccdSimplexPointW(portal, 0 as ::core::ffi::c_int),
    );
    ccdSimplexSetSize(portal, 1 as ::core::ffi::c_int);
    if ccdVec3Eq(
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
        ccd_vec3_origin,
    ) != 0
    {
        ccdVec3Set(&raw mut va, CCD_EPS * 10.0f32, CCD_ZERO, CCD_ZERO);
        ccdVec3Add(
            &raw mut (*(ccdSimplexPointW
                as unsafe extern "C" fn(
                    *mut ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *mut ccd_support_t)(portal, 0 as ::core::ffi::c_int))
            .v,
            &raw mut va,
        );
    }
    ccdVec3Copy(
        &raw mut dir,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
    );
    ccdVec3Scale(&raw mut dir, -1.0f32);
    ccdVec3Normalize(&raw mut dir);
    __ccdSupport(
        obj1,
        obj2,
        &raw mut dir,
        ccd,
        ccdSimplexPointW(portal, 1 as ::core::ffi::c_int),
    );
    ccdSimplexSetSize(portal, 2 as ::core::ffi::c_int);
    dot = ccdVec3Dot(
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
        &raw mut dir,
    );
    if ccdIsZero(dot) != 0 || dot < CCD_ZERO {
        return -(1 as ::core::ffi::c_int);
    }
    ccdVec3Cross(
        &raw mut dir,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
    );
    if ccdIsZero(ccdVec3Len2(&raw mut dir)) != 0 {
        if ccdVec3Eq(
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 1 as ::core::ffi::c_int
            ))
            .v,
            ccd_vec3_origin,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        } else {
            return 2 as ::core::ffi::c_int;
        }
    }
    ccdVec3Normalize(&raw mut dir);
    __ccdSupport(
        obj1,
        obj2,
        &raw mut dir,
        ccd,
        ccdSimplexPointW(portal, 2 as ::core::ffi::c_int),
    );
    dot = ccdVec3Dot(
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 2 as ::core::ffi::c_int))
        .v,
        &raw mut dir,
    );
    if ccdIsZero(dot) != 0 || dot < CCD_ZERO {
        return -(1 as ::core::ffi::c_int);
    }
    ccdSimplexSetSize(portal, 3 as ::core::ffi::c_int);
    ccdVec3Sub2(
        &raw mut va,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
    );
    ccdVec3Sub2(
        &raw mut vb,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 2 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
    );
    ccdVec3Cross(&raw mut dir, &raw mut va, &raw mut vb);
    ccdVec3Normalize(&raw mut dir);
    dot = ccdVec3Dot(
        &raw mut dir,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
    );
    if dot > CCD_ZERO {
        ccdSimplexSwap(portal, 1 as size_t, 2 as size_t);
        ccdVec3Scale(&raw mut dir, -1.0f32);
    }
    while ccdSimplexSize(portal) < 4 as ::core::ffi::c_int {
        __ccdSupport(
            obj1,
            obj2,
            &raw mut dir,
            ccd,
            ccdSimplexPointW(portal, 3 as ::core::ffi::c_int),
        );
        dot = ccdVec3Dot(
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 3 as ::core::ffi::c_int
            ))
            .v,
            &raw mut dir,
        );
        if ccdIsZero(dot) != 0 || dot < CCD_ZERO {
            return -(1 as ::core::ffi::c_int);
        }
        cont = 0 as ::core::ffi::c_int;
        ccdVec3Cross(
            &raw mut va,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 1 as ::core::ffi::c_int
            ))
            .v,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 3 as ::core::ffi::c_int
            ))
            .v,
        );
        dot = ccdVec3Dot(
            &raw mut va,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 0 as ::core::ffi::c_int
            ))
            .v,
        );
        if dot < CCD_ZERO && ccdIsZero(dot) == 0 {
            ccdSimplexSet(
                portal,
                2 as size_t,
                ccdSimplexPoint(portal, 3 as ::core::ffi::c_int),
            );
            cont = 1 as ::core::ffi::c_int;
        }
        if cont == 0 {
            ccdVec3Cross(
                &raw mut va,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 3 as ::core::ffi::c_int
                ))
                .v,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 2 as ::core::ffi::c_int
                ))
                .v,
            );
            dot = ccdVec3Dot(
                &raw mut va,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 0 as ::core::ffi::c_int
                ))
                .v,
            );
            if dot < CCD_ZERO && ccdIsZero(dot) == 0 {
                ccdSimplexSet(
                    portal,
                    1 as size_t,
                    ccdSimplexPoint(portal, 3 as ::core::ffi::c_int),
                );
                cont = 1 as ::core::ffi::c_int;
            }
        }
        if cont != 0 {
            ccdVec3Sub2(
                &raw mut va,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 1 as ::core::ffi::c_int
                ))
                .v,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 0 as ::core::ffi::c_int
                ))
                .v,
            );
            ccdVec3Sub2(
                &raw mut vb,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 2 as ::core::ffi::c_int
                ))
                .v,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 0 as ::core::ffi::c_int
                ))
                .v,
            );
            ccdVec3Cross(&raw mut dir, &raw mut va, &raw mut vb);
            ccdVec3Normalize(&raw mut dir);
        } else {
            ccdSimplexSetSize(portal, 4 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn refinePortal(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut portal: *mut ccd_simplex_t,
) -> ::core::ffi::c_int {
    let mut dir: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut v4: ccd_support_t = _ccd_support_t {
        v: _ccd_vec3_t { v: [0.; 3] },
        v1: _ccd_vec3_t { v: [0.; 3] },
        v2: _ccd_vec3_t { v: [0.; 3] },
    };
    loop {
        portalDir(portal, &raw mut dir);
        if portalEncapsulesOrigin(portal, &raw mut dir) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        __ccdSupport(obj1, obj2, &raw mut dir, ccd, &raw mut v4);
        if portalCanEncapsuleOrigin(portal, &raw mut v4, &raw mut dir) == 0
            || portalReachTolerance(portal, &raw mut v4, &raw mut dir, ccd) != 0
        {
            return -(1 as ::core::ffi::c_int);
        }
        expandPortal(portal, &raw mut v4);
    }
}
unsafe extern "C" fn findPenetr(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut portal: *mut ccd_simplex_t,
    mut depth: *mut ccd_real_t,
    mut pdir: *mut ccd_vec3_t,
    mut pos: *mut ccd_vec3_t,
) {
    let mut dir: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut v4: ccd_support_t = _ccd_support_t {
        v: _ccd_vec3_t { v: [0.; 3] },
        v1: _ccd_vec3_t { v: [0.; 3] },
        v2: _ccd_vec3_t { v: [0.; 3] },
    };
    let mut iterations: ::core::ffi::c_ulong = 0;
    iterations = 0 as ::core::ffi::c_ulong;
    loop {
        portalDir(portal, &raw mut dir);
        __ccdSupport(obj1, obj2, &raw mut dir, ccd, &raw mut v4);
        if portalReachTolerance(portal, &raw mut v4, &raw mut dir, ccd) != 0
            || iterations > (*ccd).max_iterations
        {
            *depth = ccdVec3PointTriDist2(
                ccd_vec3_origin,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 1 as ::core::ffi::c_int
                ))
                .v,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 2 as ::core::ffi::c_int
                ))
                .v,
                &raw const (*(ccdSimplexPoint
                    as unsafe extern "C" fn(
                        *const ccd_simplex_t,
                        ::core::ffi::c_int,
                    ) -> *const ccd_support_t)(
                    portal, 3 as ::core::ffi::c_int
                ))
                .v,
                pdir,
            );
            *depth = sqrtf(*depth) as ccd_real_t;
            if ccdIsZero(*depth) != 0 {
                ccdVec3Copy(pdir, ccd_vec3_origin);
            } else {
                ccdVec3Normalize(pdir);
            }
            findPos(obj1, obj2, ccd, portal, pos);
            return;
        }
        expandPortal(portal, &raw mut v4);
        iterations = iterations.wrapping_add(1);
    }
}
unsafe extern "C" fn findPenetrTouch(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut portal: *mut ccd_simplex_t,
    mut depth: *mut ccd_real_t,
    mut dir: *mut ccd_vec3_t,
    mut pos: *mut ccd_vec3_t,
) {
    *depth = 0.0f32 as ccd_real_t;
    ccdVec3Copy(dir, ccd_vec3_origin);
    ccdVec3Copy(
        pos,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v1,
    );
    ccdVec3Add(
        pos,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v2,
    );
    ccdVec3Scale(pos, 0.5f32);
}
unsafe extern "C" fn findPenetrSegment(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut portal: *mut ccd_simplex_t,
    mut depth: *mut ccd_real_t,
    mut dir: *mut ccd_vec3_t,
    mut pos: *mut ccd_vec3_t,
) {
    ccdVec3Copy(
        pos,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v1,
    );
    ccdVec3Add(
        pos,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v2,
    );
    ccdVec3Scale(pos, 0.5f32);
    ccdVec3Copy(
        dir,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
    );
    *depth = sqrtf(ccdVec3Len2(dir) as ::core::ffi::c_float) as ccd_real_t;
    ccdVec3Normalize(dir);
}
unsafe extern "C" fn findPos(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut portal: *const ccd_simplex_t,
    mut pos: *mut ccd_vec3_t,
) {
    let mut dir: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut i: size_t = 0;
    let mut b: [ccd_real_t; 4] = [0.; 4];
    let mut sum: ccd_real_t = 0.;
    let mut inv: ccd_real_t = 0.;
    let mut vec: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut p1: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut p2: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    portalDir(portal, &raw mut dir);
    ccdVec3Cross(
        &raw mut vec,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 2 as ::core::ffi::c_int))
        .v,
    );
    b[0 as ::core::ffi::c_int as usize] = ccdVec3Dot(
        &raw mut vec,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 3 as ::core::ffi::c_int))
        .v,
    );
    ccdVec3Cross(
        &raw mut vec,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 3 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 2 as ::core::ffi::c_int))
        .v,
    );
    b[1 as ::core::ffi::c_int as usize] = ccdVec3Dot(
        &raw mut vec,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
    );
    ccdVec3Cross(
        &raw mut vec,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
    );
    b[2 as ::core::ffi::c_int as usize] = ccdVec3Dot(
        &raw mut vec,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 3 as ::core::ffi::c_int))
        .v,
    );
    ccdVec3Cross(
        &raw mut vec,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 2 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
    );
    b[3 as ::core::ffi::c_int as usize] = ccdVec3Dot(
        &raw mut vec,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
    );
    sum = b[0 as ::core::ffi::c_int as usize]
        + b[1 as ::core::ffi::c_int as usize]
        + b[2 as ::core::ffi::c_int as usize]
        + b[3 as ::core::ffi::c_int as usize];
    if ccdIsZero(sum) != 0 || sum < CCD_ZERO {
        b[0 as ::core::ffi::c_int as usize] = 0.0f32 as ccd_real_t;
        ccdVec3Cross(
            &raw mut vec,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 2 as ::core::ffi::c_int
            ))
            .v,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 3 as ::core::ffi::c_int
            ))
            .v,
        );
        b[1 as ::core::ffi::c_int as usize] = ccdVec3Dot(&raw mut vec, &raw mut dir);
        ccdVec3Cross(
            &raw mut vec,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 3 as ::core::ffi::c_int
            ))
            .v,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 1 as ::core::ffi::c_int
            ))
            .v,
        );
        b[2 as ::core::ffi::c_int as usize] = ccdVec3Dot(&raw mut vec, &raw mut dir);
        ccdVec3Cross(
            &raw mut vec,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 1 as ::core::ffi::c_int
            ))
            .v,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 2 as ::core::ffi::c_int
            ))
            .v,
        );
        b[3 as ::core::ffi::c_int as usize] = ccdVec3Dot(&raw mut vec, &raw mut dir);
        sum = b[1 as ::core::ffi::c_int as usize]
            + b[2 as ::core::ffi::c_int as usize]
            + b[3 as ::core::ffi::c_int as usize];
    }
    inv = 1.0f32 / sum;
    ccdVec3Copy(&raw mut p1, ccd_vec3_origin);
    ccdVec3Copy(&raw mut p2, ccd_vec3_origin);
    i = 0 as size_t;
    while i < 4 as size_t {
        ccdVec3Copy(
            &raw mut vec,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, i as ::core::ffi::c_int
            ))
            .v1,
        );
        ccdVec3Scale(&raw mut vec, b[i as usize]);
        ccdVec3Add(&raw mut p1, &raw mut vec);
        ccdVec3Copy(
            &raw mut vec,
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, i as ::core::ffi::c_int
            ))
            .v2,
        );
        ccdVec3Scale(&raw mut vec, b[i as usize]);
        ccdVec3Add(&raw mut p2, &raw mut vec);
        i = i.wrapping_add(1);
    }
    ccdVec3Scale(&raw mut p1, inv);
    ccdVec3Scale(&raw mut p2, inv);
    ccdVec3Copy(pos, &raw mut p1);
    ccdVec3Add(pos, &raw mut p2);
    ccdVec3Scale(pos, 0.5f32);
}
#[inline(always)]
unsafe extern "C" fn expandPortal(mut portal: *mut ccd_simplex_t, mut v4: *const ccd_support_t) {
    let mut dot: ccd_real_t = 0.;
    let mut v4v0: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    ccdVec3Cross(
        &raw mut v4v0,
        &raw const (*v4).v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 0 as ::core::ffi::c_int))
        .v,
    );
    dot = ccdVec3Dot(
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
        &raw mut v4v0,
    );
    if dot > CCD_ZERO {
        dot = ccdVec3Dot(
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 2 as ::core::ffi::c_int
            ))
            .v,
            &raw mut v4v0,
        );
        if dot > CCD_ZERO {
            ccdSimplexSet(portal, 1 as size_t, v4);
        } else {
            ccdSimplexSet(portal, 3 as size_t, v4);
        }
    } else {
        dot = ccdVec3Dot(
            &raw const (*(ccdSimplexPoint
                as unsafe extern "C" fn(
                    *const ccd_simplex_t,
                    ::core::ffi::c_int,
                ) -> *const ccd_support_t)(
                portal, 3 as ::core::ffi::c_int
            ))
            .v,
            &raw mut v4v0,
        );
        if dot > CCD_ZERO {
            ccdSimplexSet(portal, 2 as size_t, v4);
        } else {
            ccdSimplexSet(portal, 1 as size_t, v4);
        }
    };
}
#[inline(always)]
unsafe extern "C" fn portalDir(mut portal: *const ccd_simplex_t, mut dir: *mut ccd_vec3_t) {
    let mut v2v1: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut v3v1: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    ccdVec3Sub2(
        &raw mut v2v1,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 2 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
    );
    ccdVec3Sub2(
        &raw mut v3v1,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 3 as ::core::ffi::c_int))
        .v,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
    );
    ccdVec3Cross(dir, &raw mut v2v1, &raw mut v3v1);
    ccdVec3Normalize(dir);
}
#[inline(always)]
unsafe extern "C" fn portalEncapsulesOrigin(
    mut portal: *const ccd_simplex_t,
    mut dir: *const ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut dot: ccd_real_t = 0.;
    dot = ccdVec3Dot(
        dir,
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
    );
    return (ccdIsZero(dot) != 0 || dot > CCD_ZERO) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn portalReachTolerance(
    mut portal: *const ccd_simplex_t,
    mut v4: *const ccd_support_t,
    mut dir: *const ccd_vec3_t,
    mut ccd: *const ccd_t,
) -> ::core::ffi::c_int {
    let mut dv1: ccd_real_t = 0.;
    let mut dv2: ccd_real_t = 0.;
    let mut dv3: ccd_real_t = 0.;
    let mut dv4: ccd_real_t = 0.;
    let mut dot1: ccd_real_t = 0.;
    let mut dot2: ccd_real_t = 0.;
    let mut dot3: ccd_real_t = 0.;
    dv1 = ccdVec3Dot(
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 1 as ::core::ffi::c_int))
        .v,
        dir,
    );
    dv2 = ccdVec3Dot(
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 2 as ::core::ffi::c_int))
        .v,
        dir,
    );
    dv3 = ccdVec3Dot(
        &raw const (*(ccdSimplexPoint
            as unsafe extern "C" fn(
                *const ccd_simplex_t,
                ::core::ffi::c_int,
            ) -> *const ccd_support_t)(portal, 3 as ::core::ffi::c_int))
        .v,
        dir,
    );
    dv4 = ccdVec3Dot(&raw const (*v4).v, dir);
    dot1 = dv4 - dv1;
    dot2 = dv4 - dv2;
    dot3 = dv4 - dv3;
    dot1 = fminf(dot1 as ::core::ffi::c_float, dot2 as ::core::ffi::c_float) as ccd_real_t;
    dot1 = fminf(dot1 as ::core::ffi::c_float, dot3 as ::core::ffi::c_float) as ccd_real_t;
    return (ccdEq(dot1, (*ccd).mpr_tolerance) != 0 || dot1 < (*ccd).mpr_tolerance)
        as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn portalCanEncapsuleOrigin(
    mut portal: *const ccd_simplex_t,
    mut v4: *const ccd_support_t,
    mut dir: *const ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut dot: ccd_real_t = 0.;
    dot = ccdVec3Dot(&raw const (*v4).v, dir);
    return (ccdIsZero(dot) != 0 || dot > CCD_ZERO) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdSupportCopy(mut d: *mut ccd_support_t, mut s: *const ccd_support_t) {
    *d = *s;
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexSize(mut s: *const ccd_simplex_t) -> ::core::ffi::c_int {
    return (*s).last + 1 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexPoint(
    mut s: *const ccd_simplex_t,
    mut idx: ::core::ffi::c_int,
) -> *const ccd_support_t {
    return (&raw const (*s).ps as *const ccd_support_t).offset(idx as isize)
        as *const ccd_support_t;
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexPointW(
    mut s: *mut ccd_simplex_t,
    mut idx: ::core::ffi::c_int,
) -> *mut ccd_support_t {
    return (&raw mut (*s).ps as *mut ccd_support_t).offset(idx as isize) as *mut ccd_support_t;
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexSet(
    mut s: *mut ccd_simplex_t,
    mut pos: size_t,
    mut a: *const ccd_support_t,
) {
    ccdSupportCopy(
        (&raw mut (*s).ps as *mut ccd_support_t).offset(pos as isize),
        a,
    );
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexSetSize(mut s: *mut ccd_simplex_t, mut size: ::core::ffi::c_int) {
    (*s).last = size - 1 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexSwap(mut s: *mut ccd_simplex_t, mut pos1: size_t, mut pos2: size_t) {
    let mut supp: ccd_support_t = _ccd_support_t {
        v: _ccd_vec3_t { v: [0.; 3] },
        v1: _ccd_vec3_t { v: [0.; 3] },
        v2: _ccd_vec3_t { v: [0.; 3] },
    };
    ccdSupportCopy(
        &raw mut supp,
        (&raw mut (*s).ps as *mut ccd_support_t).offset(pos1 as isize) as *mut ccd_support_t,
    );
    ccdSupportCopy(
        (&raw mut (*s).ps as *mut ccd_support_t).offset(pos1 as isize) as *mut ccd_support_t,
        (&raw mut (*s).ps as *mut ccd_support_t).offset(pos2 as isize) as *mut ccd_support_t,
    );
    ccdSupportCopy(
        (&raw mut (*s).ps as *mut ccd_support_t).offset(pos2 as isize) as *mut ccd_support_t,
        &raw mut supp,
    );
}
pub const FLT_EPSILON: ::core::ffi::c_float = __FLT_EPSILON__;
pub const __FLT_EPSILON__: ::core::ffi::c_float = 1.19209290e-7f32;
