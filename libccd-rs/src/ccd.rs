unsafe extern "C" {
    fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn fabsf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut ccd_vec3_origin: *mut ccd_vec3_t;
    static mut ccd_points_on_sphere: *mut ccd_vec3_t;
    static mut ccd_points_on_sphere_len: size_t;
    fn ccdVec3PointSegmentDist2(
        P: *const ccd_vec3_t,
        a: *const ccd_vec3_t,
        b: *const ccd_vec3_t,
        witness: *mut ccd_vec3_t,
    ) -> ccd_real_t;
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
    fn ccdPtInit(pt: *mut ccd_pt_t);
    fn ccdPtDestroy(pt: *mut ccd_pt_t);
    fn ccdPtAddVertex(pt: *mut ccd_pt_t, v: *const ccd_support_t) -> *mut ccd_pt_vertex_t;
    fn ccdPtAddEdge(
        pt: *mut ccd_pt_t,
        v1: *mut ccd_pt_vertex_t,
        v2: *mut ccd_pt_vertex_t,
    ) -> *mut ccd_pt_edge_t;
    fn ccdPtAddFace(
        pt: *mut ccd_pt_t,
        e1: *mut ccd_pt_edge_t,
        e2: *mut ccd_pt_edge_t,
        e3: *mut ccd_pt_edge_t,
    ) -> *mut ccd_pt_face_t;
    fn ccdPtNearest(pt: *mut ccd_pt_t) -> *mut ccd_pt_el_t;
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type ccd_pt_t = _ccd_pt_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_pt_t {
    pub vertices: ccd_list_t,
    pub edges: ccd_list_t,
    pub faces: ccd_list_t,
    pub nearest: *mut ccd_pt_el_t,
    pub nearest_dist: ccd_real_t,
    pub nearest_type: ::core::ffi::c_int,
}
pub type ccd_pt_el_t = _ccd_pt_el_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_pt_el_t {
    pub type_0: ::core::ffi::c_int,
    pub dist: ccd_real_t,
    pub witness: ccd_vec3_t,
    pub list: ccd_list_t,
}
pub type ccd_list_t = _ccd_list_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_list_t {
    pub next: *mut _ccd_list_t,
    pub prev: *mut _ccd_list_t,
}
pub type ccd_pt_face_t = _ccd_pt_face_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_pt_face_t {
    pub type_0: ::core::ffi::c_int,
    pub dist: ccd_real_t,
    pub witness: ccd_vec3_t,
    pub list: ccd_list_t,
    pub edge: [*mut ccd_pt_edge_t; 3],
}
pub type ccd_pt_edge_t = _ccd_pt_edge_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_pt_edge_t {
    pub type_0: ::core::ffi::c_int,
    pub dist: ccd_real_t,
    pub witness: ccd_vec3_t,
    pub list: ccd_list_t,
    pub vertex: [*mut ccd_pt_vertex_t; 2],
    pub faces: [*mut _ccd_pt_face_t; 2],
    pub vertex_list: [ccd_list_t; 2],
}
pub type ccd_pt_vertex_t = _ccd_pt_vertex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_pt_vertex_t {
    pub type_0: ::core::ffi::c_int,
    pub dist: ccd_real_t,
    pub witness: ccd_vec3_t,
    pub list: ccd_list_t,
    pub id: ::core::ffi::c_int,
    pub v: ccd_support_t,
    pub edges: ccd_list_t,
}
pub const CCD_EPS: ::core::ffi::c_float = FLT_EPSILON;
pub const CCD_ONE: ::core::ffi::c_float = 1.0f32;
pub const CCD_ZERO: ::core::ffi::c_float = 0.0f32;
#[inline(always)]
unsafe extern "C" fn ccdSign(mut val: ccd_real_t) -> ::core::ffi::c_int {
    if ccdIsZero(val) != 0 {
        return 0 as ::core::ffi::c_int;
    } else if val < CCD_ZERO {
        return -(1 as ::core::ffi::c_int);
    }
    return 1 as ::core::ffi::c_int;
}
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


pub unsafe extern "C" fn ccdFirstDirDefault(
    mut o1: *const ::core::ffi::c_void,
    mut o2: *const ::core::ffi::c_void,
    mut dir: *mut ccd_vec3_t,
) {
    ccdVec3Set(dir, CCD_ONE, CCD_ZERO, CCD_ZERO);
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdGJKIntersect(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
) -> ::core::ffi::c_int {
    let mut simplex: ccd_simplex_t = _ccd_simplex_t {
        ps: [_ccd_support_t {
            v: _ccd_vec3_t { v: [0.; 3] },
            v1: _ccd_vec3_t { v: [0.; 3] },
            v2: _ccd_vec3_t { v: [0.; 3] },
        }; 4],
        last: 0,
    };
    return (__ccdGJK(obj1, obj2, ccd, &raw mut simplex) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdGJKSeparate(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut sep: *mut ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut polytope: ccd_pt_t = _ccd_pt_t {
        vertices: _ccd_list_t {
            next: ::core::ptr::null_mut::<_ccd_list_t>(),
            prev: ::core::ptr::null_mut::<_ccd_list_t>(),
        },
        edges: _ccd_list_t {
            next: ::core::ptr::null_mut::<_ccd_list_t>(),
            prev: ::core::ptr::null_mut::<_ccd_list_t>(),
        },
        faces: _ccd_list_t {
            next: ::core::ptr::null_mut::<_ccd_list_t>(),
            prev: ::core::ptr::null_mut::<_ccd_list_t>(),
        },
        nearest: ::core::ptr::null_mut::<ccd_pt_el_t>(),
        nearest_dist: 0.,
        nearest_type: 0,
    };
    let mut nearest: *mut ccd_pt_el_t = ::core::ptr::null_mut::<ccd_pt_el_t>();
    let mut ret: ::core::ffi::c_int = 0;
    ccdPtInit(&raw mut polytope);
    ret = __ccdGJKEPA(obj1, obj2, ccd, &raw mut polytope, &raw mut nearest);
    if !nearest.is_null() {
        ccdVec3Copy(sep, &raw mut (*nearest).witness);
    }
    ccdPtDestroy(&raw mut polytope);
    return ret;
}
unsafe extern "C" fn penEPAPosCmp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut v1: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut v2: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    v1 = *(a as *mut *mut ccd_pt_vertex_t);
    v2 = *(b as *mut *mut ccd_pt_vertex_t);
    if ccdEq((*v1).dist, (*v2).dist) != 0 {
        return 0 as ::core::ffi::c_int;
    } else if (*v1).dist < (*v2).dist {
        return -(1 as ::core::ffi::c_int);
    } else {
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn penEPAPos(
    mut pt: *const ccd_pt_t,
    mut nearest: *const ccd_pt_el_t,
    mut pos: *mut ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut v: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut vs: *mut *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<*mut ccd_pt_vertex_t>();
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut scale: ccd_real_t = 0.;
    len = 0 as size_t;
    v = ((*pt).vertices.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    while &raw mut (*v).list != &raw const (*pt).vertices as *mut ccd_list_t {
        len = len.wrapping_add(1);
        v = ((*v).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    }
    vs = realloc(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        (::core::mem::size_of::<*mut ccd_pt_vertex_t>() as size_t).wrapping_mul(len),
    ) as *mut *mut ccd_pt_vertex_t;
    if vs.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as size_t;
    v = ((*pt).vertices.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    while &raw mut (*v).list != &raw const (*pt).vertices as *mut ccd_list_t {
        let fresh0 = i;
        i = i.wrapping_add(1);
        let ref mut fresh1 = *vs.offset(fresh0 as isize);
        *fresh1 = v;
        v = ((*v).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    }
    qsort(
        vs as *mut ::core::ffi::c_void,
        len,
        ::core::mem::size_of::<*mut ccd_pt_vertex_t>() as size_t,
        Some(
            penEPAPosCmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    ccdVec3Set(pos, CCD_ZERO, CCD_ZERO, CCD_ZERO);
    scale = CCD_ZERO as ccd_real_t;
    if len.wrapping_rem(2 as size_t) == 1 as size_t {
        len = len.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < len.wrapping_div(2 as size_t) {
        ccdVec3Add(pos, &raw mut (**vs.offset(i as isize)).v.v1);
        ccdVec3Add(pos, &raw mut (**vs.offset(i as isize)).v.v2);
        scale += 2.0f32;
        i = i.wrapping_add(1);
    }
    ccdVec3Scale(pos, CCD_ONE / scale);
    free(vs as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdGJKPenetration(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut depth: *mut ccd_real_t,
    mut dir: *mut ccd_vec3_t,
    mut pos: *mut ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut polytope: ccd_pt_t = _ccd_pt_t {
        vertices: _ccd_list_t {
            next: ::core::ptr::null_mut::<_ccd_list_t>(),
            prev: ::core::ptr::null_mut::<_ccd_list_t>(),
        },
        edges: _ccd_list_t {
            next: ::core::ptr::null_mut::<_ccd_list_t>(),
            prev: ::core::ptr::null_mut::<_ccd_list_t>(),
        },
        faces: _ccd_list_t {
            next: ::core::ptr::null_mut::<_ccd_list_t>(),
            prev: ::core::ptr::null_mut::<_ccd_list_t>(),
        },
        nearest: ::core::ptr::null_mut::<ccd_pt_el_t>(),
        nearest_dist: 0.,
        nearest_type: 0,
    };
    let mut nearest: *mut ccd_pt_el_t = ::core::ptr::null_mut::<ccd_pt_el_t>();
    let mut ret: ::core::ffi::c_int = 0;
    ccdPtInit(&raw mut polytope);
    ret = __ccdGJKEPA(obj1, obj2, ccd, &raw mut polytope, &raw mut nearest);
    if ret == 0 as ::core::ffi::c_int && !nearest.is_null() {
        *depth = sqrtf((*nearest).dist as ::core::ffi::c_float) as ccd_real_t;
        ccdVec3Copy(dir, &raw mut (*nearest).witness);
        ccdVec3Normalize(dir);
        if penEPAPos(&raw mut polytope, nearest, pos) != 0 as ::core::ffi::c_int {
            ccdPtDestroy(&raw mut polytope);
            return -(2 as ::core::ffi::c_int);
        }
    }
    ccdPtDestroy(&raw mut polytope);
    return ret;
}
unsafe extern "C" fn __ccdGJK(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut simplex: *mut ccd_simplex_t,
) -> ::core::ffi::c_int {
    let mut iterations: ::core::ffi::c_ulong = 0;
    let mut dir: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut last: ccd_support_t = _ccd_support_t {
        v: _ccd_vec3_t { v: [0.; 3] },
        v1: _ccd_vec3_t { v: [0.; 3] },
        v2: _ccd_vec3_t { v: [0.; 3] },
    };
    let mut do_simplex_res: ::core::ffi::c_int = 0;
    ccdSimplexInit(simplex);
    (*ccd).first_dir.expect("non-null function pointer")(obj1, obj2, &raw mut dir);
    __ccdSupport(obj1, obj2, &raw mut dir, ccd, &raw mut last);
    ccdSimplexAdd(simplex, &raw mut last);
    ccdVec3Copy(&raw mut dir, &raw mut last.v);
    ccdVec3Scale(&raw mut dir, -CCD_ONE);
    iterations = 0 as ::core::ffi::c_ulong;
    while iterations < (*ccd).max_iterations {
        __ccdSupport(obj1, obj2, &raw mut dir, ccd, &raw mut last);
        if ccdVec3Dot(&raw mut last.v, &raw mut dir) < CCD_ZERO {
            return -(1 as ::core::ffi::c_int);
        }
        ccdSimplexAdd(simplex, &raw mut last);
        do_simplex_res = doSimplex(simplex, &raw mut dir);
        if do_simplex_res == 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        } else if do_simplex_res == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
        if ccdIsZero(ccdVec3Len2(&raw mut dir)) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        iterations = iterations.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn __ccdGJKEPA(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut polytope: *mut ccd_pt_t,
    mut nearest: *mut *mut ccd_pt_el_t,
) -> ::core::ffi::c_int {
    let mut simplex: ccd_simplex_t = _ccd_simplex_t {
        ps: [_ccd_support_t {
            v: _ccd_vec3_t { v: [0.; 3] },
            v1: _ccd_vec3_t { v: [0.; 3] },
            v2: _ccd_vec3_t { v: [0.; 3] },
        }; 4],
        last: 0,
    };
    let mut supp: ccd_support_t = _ccd_support_t {
        v: _ccd_vec3_t { v: [0.; 3] },
        v1: _ccd_vec3_t { v: [0.; 3] },
        v2: _ccd_vec3_t { v: [0.; 3] },
    };
    let mut ret: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    *nearest = ::core::ptr::null_mut::<ccd_pt_el_t>();
    ret = __ccdGJK(obj1, obj2, ccd, &raw mut simplex);
    if ret != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    size = ccdSimplexSize(&raw mut simplex);
    if size == 4 as ::core::ffi::c_int {
        ret = simplexToPolytope4(obj1, obj2, ccd, &raw mut simplex, polytope, nearest);
    } else if size == 3 as ::core::ffi::c_int {
        ret = simplexToPolytope3(obj1, obj2, ccd, &raw mut simplex, polytope, nearest);
    } else {
        ret = simplexToPolytope2(obj1, obj2, ccd, &raw mut simplex, polytope, nearest);
    }
    if ret == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    } else if ret == -(2 as ::core::ffi::c_int) {
        return -(2 as ::core::ffi::c_int);
    }
    loop {
        *nearest = ccdPtNearest(polytope);
        if nextSupport(obj1, obj2, ccd, *nearest, &raw mut supp) != 0 as ::core::ffi::c_int {
            break;
        }
        if expandPolytope(polytope, *nearest, &raw mut supp) != 0 as ::core::ffi::c_int {
            return -(2 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn doSimplex2(
    mut simplex: *mut ccd_simplex_t,
    mut dir: *mut ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut A: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut B: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut AB: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut AO: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut tmp: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut dot: ccd_real_t = 0.;
    A = ccdSimplexLast(simplex);
    B = ccdSimplexPoint(simplex, 0 as ::core::ffi::c_int);
    ccdVec3Sub2(&raw mut AB, &raw const (*B).v, &raw const (*A).v);
    ccdVec3Copy(&raw mut AO, &raw const (*A).v);
    ccdVec3Scale(&raw mut AO, -CCD_ONE);
    dot = ccdVec3Dot(&raw mut AB, &raw mut AO);
    ccdVec3Cross(&raw mut tmp, &raw mut AB, &raw mut AO);
    if ccdIsZero(ccdVec3Len2(&raw mut tmp)) != 0 && dot > CCD_ZERO {
        return 1 as ::core::ffi::c_int;
    }
    if ccdIsZero(dot) != 0 || dot < CCD_ZERO {
        ccdSimplexSet(simplex, 0 as size_t, A);
        ccdSimplexSetSize(simplex, 1 as ::core::ffi::c_int);
        ccdVec3Copy(dir, &raw mut AO);
    } else {
        tripleCross(&raw mut AB, &raw mut AO, &raw mut AB, dir);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn doSimplex3(
    mut simplex: *mut ccd_simplex_t,
    mut dir: *mut ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut A: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut B: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut C: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut AO: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut AB: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut AC: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut ABC: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut tmp: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut dot: ccd_real_t = 0.;
    let mut dist: ccd_real_t = 0.;
    A = ccdSimplexLast(simplex);
    B = ccdSimplexPoint(simplex, 1 as ::core::ffi::c_int);
    C = ccdSimplexPoint(simplex, 0 as ::core::ffi::c_int);
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*A).v,
        &raw const (*B).v,
        &raw const (*C).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if ccdVec3Eq(&raw const (*A).v, &raw const (*B).v) != 0
        || ccdVec3Eq(&raw const (*A).v, &raw const (*C).v) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    ccdVec3Copy(&raw mut AO, &raw const (*A).v);
    ccdVec3Scale(&raw mut AO, -CCD_ONE);
    ccdVec3Sub2(&raw mut AB, &raw const (*B).v, &raw const (*A).v);
    ccdVec3Sub2(&raw mut AC, &raw const (*C).v, &raw const (*A).v);
    ccdVec3Cross(&raw mut ABC, &raw mut AB, &raw mut AC);
    ccdVec3Cross(&raw mut tmp, &raw mut ABC, &raw mut AC);
    dot = ccdVec3Dot(&raw mut tmp, &raw mut AO);
    let mut current_block_45: u64;
    if ccdIsZero(dot) != 0 || dot > CCD_ZERO {
        dot = ccdVec3Dot(&raw mut AC, &raw mut AO);
        if ccdIsZero(dot) != 0 || dot > CCD_ZERO {
            ccdSimplexSet(simplex, 1 as size_t, A);
            ccdSimplexSetSize(simplex, 2 as ::core::ffi::c_int);
            tripleCross(&raw mut AC, &raw mut AO, &raw mut AC, dir);
            current_block_45 = 8845338526596852646;
        } else {
            current_block_45 = 10806444022487928479;
        }
    } else {
        ccdVec3Cross(&raw mut tmp, &raw mut AB, &raw mut ABC);
        dot = ccdVec3Dot(&raw mut tmp, &raw mut AO);
        if ccdIsZero(dot) != 0 || dot > CCD_ZERO {
            current_block_45 = 10806444022487928479;
        } else {
            dot = ccdVec3Dot(&raw mut ABC, &raw mut AO);
            if ccdIsZero(dot) != 0 || dot > CCD_ZERO {
                ccdVec3Copy(dir, &raw mut ABC);
            } else {
                let mut Ctmp: ccd_support_t = _ccd_support_t {
                    v: _ccd_vec3_t { v: [0.; 3] },
                    v1: _ccd_vec3_t { v: [0.; 3] },
                    v2: _ccd_vec3_t { v: [0.; 3] },
                };
                ccdSupportCopy(&raw mut Ctmp, C);
                ccdSimplexSet(simplex, 0 as size_t, B);
                ccdSimplexSet(simplex, 1 as size_t, &raw mut Ctmp);
                ccdVec3Copy(dir, &raw mut ABC);
                ccdVec3Scale(dir, -CCD_ONE);
            }
            current_block_45 = 8845338526596852646;
        }
    }
    match current_block_45 {
        10806444022487928479 => {
            dot = ccdVec3Dot(&raw mut AB, &raw mut AO);
            if ccdIsZero(dot) != 0 || dot > CCD_ZERO {
                ccdSimplexSet(simplex, 0 as size_t, B);
                ccdSimplexSet(simplex, 1 as size_t, A);
                ccdSimplexSetSize(simplex, 2 as ::core::ffi::c_int);
                tripleCross(&raw mut AB, &raw mut AO, &raw mut AB, dir);
            } else {
                ccdSimplexSet(simplex, 0 as size_t, A);
                ccdSimplexSetSize(simplex, 1 as ::core::ffi::c_int);
                ccdVec3Copy(dir, &raw mut AO);
            }
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn doSimplex4(
    mut simplex: *mut ccd_simplex_t,
    mut dir: *mut ccd_vec3_t,
) -> ::core::ffi::c_int {
    let mut A: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut B: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut C: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut D: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut AO: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut AB: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut AC: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut AD: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut ABC: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut ACD: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut ADB: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut B_on_ACD: ::core::ffi::c_int = 0;
    let mut C_on_ADB: ::core::ffi::c_int = 0;
    let mut D_on_ABC: ::core::ffi::c_int = 0;
    let mut AB_O: ::core::ffi::c_int = 0;
    let mut AC_O: ::core::ffi::c_int = 0;
    let mut AD_O: ::core::ffi::c_int = 0;
    let mut dist: ccd_real_t = 0.;
    A = ccdSimplexLast(simplex);
    B = ccdSimplexPoint(simplex, 2 as ::core::ffi::c_int);
    C = ccdSimplexPoint(simplex, 1 as ::core::ffi::c_int);
    D = ccdSimplexPoint(simplex, 0 as ::core::ffi::c_int);
    dist = ccdVec3PointTriDist2(
        &raw const (*A).v,
        &raw const (*B).v,
        &raw const (*C).v,
        &raw const (*D).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*A).v,
        &raw const (*B).v,
        &raw const (*C).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*A).v,
        &raw const (*C).v,
        &raw const (*D).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*A).v,
        &raw const (*B).v,
        &raw const (*D).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*B).v,
        &raw const (*C).v,
        &raw const (*D).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    ccdVec3Copy(&raw mut AO, &raw const (*A).v);
    ccdVec3Scale(&raw mut AO, -CCD_ONE);
    ccdVec3Sub2(&raw mut AB, &raw const (*B).v, &raw const (*A).v);
    ccdVec3Sub2(&raw mut AC, &raw const (*C).v, &raw const (*A).v);
    ccdVec3Sub2(&raw mut AD, &raw const (*D).v, &raw const (*A).v);
    ccdVec3Cross(&raw mut ABC, &raw mut AB, &raw mut AC);
    ccdVec3Cross(&raw mut ACD, &raw mut AC, &raw mut AD);
    ccdVec3Cross(&raw mut ADB, &raw mut AD, &raw mut AB);
    B_on_ACD = ccdSign(ccdVec3Dot(&raw mut ACD, &raw mut AB));
    C_on_ADB = ccdSign(ccdVec3Dot(&raw mut ADB, &raw mut AC));
    D_on_ABC = ccdSign(ccdVec3Dot(&raw mut ABC, &raw mut AD));
    AB_O = (ccdSign(ccdVec3Dot(&raw mut ACD, &raw mut AO)) == B_on_ACD) as ::core::ffi::c_int;
    AC_O = (ccdSign(ccdVec3Dot(&raw mut ADB, &raw mut AO)) == C_on_ADB) as ::core::ffi::c_int;
    AD_O = (ccdSign(ccdVec3Dot(&raw mut ABC, &raw mut AO)) == D_on_ABC) as ::core::ffi::c_int;
    if AB_O != 0 && AC_O != 0 && AD_O != 0 {
        return 1 as ::core::ffi::c_int;
    } else if AB_O == 0 {
        ccdSimplexSet(simplex, 2 as size_t, A);
        ccdSimplexSetSize(simplex, 3 as ::core::ffi::c_int);
    } else if AC_O == 0 {
        ccdSimplexSet(simplex, 1 as size_t, D);
        ccdSimplexSet(simplex, 0 as size_t, B);
        ccdSimplexSet(simplex, 2 as size_t, A);
        ccdSimplexSetSize(simplex, 3 as ::core::ffi::c_int);
    } else {
        ccdSimplexSet(simplex, 0 as size_t, C);
        ccdSimplexSet(simplex, 1 as size_t, B);
        ccdSimplexSet(simplex, 2 as size_t, A);
        ccdSimplexSetSize(simplex, 3 as ::core::ffi::c_int);
    }
    return doSimplex3(simplex, dir);
}
unsafe extern "C" fn doSimplex(
    mut simplex: *mut ccd_simplex_t,
    mut dir: *mut ccd_vec3_t,
) -> ::core::ffi::c_int {
    if ccdSimplexSize(simplex) == 2 as ::core::ffi::c_int {
        return doSimplex2(simplex, dir);
    } else if ccdSimplexSize(simplex) == 3 as ::core::ffi::c_int {
        return doSimplex3(simplex, dir);
    } else {
        return doSimplex4(simplex, dir);
    };
}
#[inline(always)]
unsafe extern "C" fn tripleCross(
    mut a: *const ccd_vec3_t,
    mut b: *const ccd_vec3_t,
    mut c: *const ccd_vec3_t,
    mut d: *mut ccd_vec3_t,
) {
    let mut e: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    ccdVec3Cross(&raw mut e, a, b);
    ccdVec3Cross(d, &raw mut e, c);
}
unsafe extern "C" fn simplexToPolytope4(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut simplex: *mut ccd_simplex_t,
    mut pt: *mut ccd_pt_t,
    mut nearest: *mut *mut ccd_pt_el_t,
) -> ::core::ffi::c_int {
    let mut a: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut b: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut c: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut d: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut use_polytope3: ::core::ffi::c_int = 0;
    let mut dist: ccd_real_t = 0.;
    let mut v: [*mut ccd_pt_vertex_t; 4] = [::core::ptr::null_mut::<ccd_pt_vertex_t>(); 4];
    let mut e: [*mut ccd_pt_edge_t; 6] = [::core::ptr::null_mut::<ccd_pt_edge_t>(); 6];
    let mut i: size_t = 0;
    a = ccdSimplexPoint(simplex, 0 as ::core::ffi::c_int);
    b = ccdSimplexPoint(simplex, 1 as ::core::ffi::c_int);
    c = ccdSimplexPoint(simplex, 2 as ::core::ffi::c_int);
    d = ccdSimplexPoint(simplex, 3 as ::core::ffi::c_int);
    use_polytope3 = 0 as ::core::ffi::c_int;
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*a).v,
        &raw const (*b).v,
        &raw const (*c).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        use_polytope3 = 1 as ::core::ffi::c_int;
    }
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*a).v,
        &raw const (*c).v,
        &raw const (*d).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        use_polytope3 = 1 as ::core::ffi::c_int;
        ccdSimplexSet(simplex, 1 as size_t, c);
        ccdSimplexSet(simplex, 2 as size_t, d);
    }
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*a).v,
        &raw const (*b).v,
        &raw const (*d).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        use_polytope3 = 1 as ::core::ffi::c_int;
        ccdSimplexSet(simplex, 2 as size_t, d);
    }
    dist = ccdVec3PointTriDist2(
        ccd_vec3_origin,
        &raw const (*b).v,
        &raw const (*c).v,
        &raw const (*d).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 {
        use_polytope3 = 1 as ::core::ffi::c_int;
        ccdSimplexSet(simplex, 0 as size_t, b);
        ccdSimplexSet(simplex, 1 as size_t, c);
        ccdSimplexSet(simplex, 2 as size_t, d);
    }
    if use_polytope3 != 0 {
        ccdSimplexSetSize(simplex, 3 as ::core::ffi::c_int);
        return simplexToPolytope3(obj1, obj2, ccd, simplex, pt, nearest);
    }
    i = 0 as size_t;
    while i < 4 as size_t {
        v[i as usize] = ccdPtAddVertex(pt, ccdSimplexPoint(simplex, i as ::core::ffi::c_int));
        i = i.wrapping_add(1);
    }
    e[0 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[0 as ::core::ffi::c_int as usize],
        v[1 as ::core::ffi::c_int as usize],
    );
    e[1 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[1 as ::core::ffi::c_int as usize],
        v[2 as ::core::ffi::c_int as usize],
    );
    e[2 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[2 as ::core::ffi::c_int as usize],
        v[0 as ::core::ffi::c_int as usize],
    );
    e[3 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[3 as ::core::ffi::c_int as usize],
        v[0 as ::core::ffi::c_int as usize],
    );
    e[4 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[3 as ::core::ffi::c_int as usize],
        v[1 as ::core::ffi::c_int as usize],
    );
    e[5 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[3 as ::core::ffi::c_int as usize],
        v[2 as ::core::ffi::c_int as usize],
    );
    if ccdPtAddFace(
        pt,
        e[0 as ::core::ffi::c_int as usize],
        e[1 as ::core::ffi::c_int as usize],
        e[2 as ::core::ffi::c_int as usize],
    )
    .is_null()
        || ccdPtAddFace(
            pt,
            e[3 as ::core::ffi::c_int as usize],
            e[4 as ::core::ffi::c_int as usize],
            e[0 as ::core::ffi::c_int as usize],
        )
        .is_null()
        || ccdPtAddFace(
            pt,
            e[4 as ::core::ffi::c_int as usize],
            e[5 as ::core::ffi::c_int as usize],
            e[1 as ::core::ffi::c_int as usize],
        )
        .is_null()
        || ccdPtAddFace(
            pt,
            e[5 as ::core::ffi::c_int as usize],
            e[3 as ::core::ffi::c_int as usize],
            e[2 as ::core::ffi::c_int as usize],
        )
        .is_null()
    {
        return -(2 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn simplexToPolytope3(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut simplex: *const ccd_simplex_t,
    mut pt: *mut ccd_pt_t,
    mut nearest: *mut *mut ccd_pt_el_t,
) -> ::core::ffi::c_int {
    let mut a: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut b: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut c: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut d: ccd_support_t = _ccd_support_t {
        v: _ccd_vec3_t { v: [0.; 3] },
        v1: _ccd_vec3_t { v: [0.; 3] },
        v2: _ccd_vec3_t { v: [0.; 3] },
    };
    let mut d2: ccd_support_t = _ccd_support_t {
        v: _ccd_vec3_t { v: [0.; 3] },
        v1: _ccd_vec3_t { v: [0.; 3] },
        v2: _ccd_vec3_t { v: [0.; 3] },
    };
    let mut ab: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut ac: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut dir: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut v: [*mut ccd_pt_vertex_t; 5] = [::core::ptr::null_mut::<ccd_pt_vertex_t>(); 5];
    let mut e: [*mut ccd_pt_edge_t; 9] = [::core::ptr::null_mut::<ccd_pt_edge_t>(); 9];
    let mut dist: ccd_real_t = 0.;
    let mut dist2: ccd_real_t = 0.;
    *nearest = ::core::ptr::null_mut::<ccd_pt_el_t>();
    a = ccdSimplexPoint(simplex, 0 as ::core::ffi::c_int);
    b = ccdSimplexPoint(simplex, 1 as ::core::ffi::c_int);
    c = ccdSimplexPoint(simplex, 2 as ::core::ffi::c_int);
    ccdVec3Sub2(&raw mut ab, &raw const (*b).v, &raw const (*a).v);
    ccdVec3Sub2(&raw mut ac, &raw const (*c).v, &raw const (*a).v);
    ccdVec3Cross(&raw mut dir, &raw mut ab, &raw mut ac);
    __ccdSupport(obj1, obj2, &raw mut dir, ccd, &raw mut d);
    dist = ccdVec3PointTriDist2(
        &raw mut d.v,
        &raw const (*a).v,
        &raw const (*b).v,
        &raw const (*c).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    ccdVec3Scale(&raw mut dir, -CCD_ONE);
    __ccdSupport(obj1, obj2, &raw mut dir, ccd, &raw mut d2);
    dist2 = ccdVec3PointTriDist2(
        &raw mut d2.v,
        &raw const (*a).v,
        &raw const (*b).v,
        &raw const (*c).v,
        ::core::ptr::null_mut::<ccd_vec3_t>(),
    );
    if ccdIsZero(dist) != 0 || ccdIsZero(dist2) != 0 {
        v[0 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, a);
        v[1 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, b);
        v[2 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, c);
        e[0 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
            pt,
            v[0 as ::core::ffi::c_int as usize],
            v[1 as ::core::ffi::c_int as usize],
        );
        e[1 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
            pt,
            v[1 as ::core::ffi::c_int as usize],
            v[2 as ::core::ffi::c_int as usize],
        );
        e[2 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
            pt,
            v[2 as ::core::ffi::c_int as usize],
            v[0 as ::core::ffi::c_int as usize],
        );
        *nearest = ccdPtAddFace(
            pt,
            e[0 as ::core::ffi::c_int as usize],
            e[1 as ::core::ffi::c_int as usize],
            e[2 as ::core::ffi::c_int as usize],
        ) as *mut ccd_pt_el_t;
        if (*nearest).is_null() {
            return -(2 as ::core::ffi::c_int);
        }
        return -(1 as ::core::ffi::c_int);
    }
    v[0 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, a);
    v[1 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, b);
    v[2 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, c);
    v[3 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, &raw mut d);
    v[4 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, &raw mut d2);
    e[0 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[0 as ::core::ffi::c_int as usize],
        v[1 as ::core::ffi::c_int as usize],
    );
    e[1 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[1 as ::core::ffi::c_int as usize],
        v[2 as ::core::ffi::c_int as usize],
    );
    e[2 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[2 as ::core::ffi::c_int as usize],
        v[0 as ::core::ffi::c_int as usize],
    );
    e[3 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[3 as ::core::ffi::c_int as usize],
        v[0 as ::core::ffi::c_int as usize],
    );
    e[4 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[3 as ::core::ffi::c_int as usize],
        v[1 as ::core::ffi::c_int as usize],
    );
    e[5 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[3 as ::core::ffi::c_int as usize],
        v[2 as ::core::ffi::c_int as usize],
    );
    e[6 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[4 as ::core::ffi::c_int as usize],
        v[0 as ::core::ffi::c_int as usize],
    );
    e[7 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[4 as ::core::ffi::c_int as usize],
        v[1 as ::core::ffi::c_int as usize],
    );
    e[8 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
        pt,
        v[4 as ::core::ffi::c_int as usize],
        v[2 as ::core::ffi::c_int as usize],
    );
    if ccdPtAddFace(
        pt,
        e[3 as ::core::ffi::c_int as usize],
        e[4 as ::core::ffi::c_int as usize],
        e[0 as ::core::ffi::c_int as usize],
    )
    .is_null()
        || ccdPtAddFace(
            pt,
            e[4 as ::core::ffi::c_int as usize],
            e[5 as ::core::ffi::c_int as usize],
            e[1 as ::core::ffi::c_int as usize],
        )
        .is_null()
        || ccdPtAddFace(
            pt,
            e[5 as ::core::ffi::c_int as usize],
            e[3 as ::core::ffi::c_int as usize],
            e[2 as ::core::ffi::c_int as usize],
        )
        .is_null()
        || ccdPtAddFace(
            pt,
            e[6 as ::core::ffi::c_int as usize],
            e[7 as ::core::ffi::c_int as usize],
            e[0 as ::core::ffi::c_int as usize],
        )
        .is_null()
        || ccdPtAddFace(
            pt,
            e[7 as ::core::ffi::c_int as usize],
            e[8 as ::core::ffi::c_int as usize],
            e[1 as ::core::ffi::c_int as usize],
        )
        .is_null()
        || ccdPtAddFace(
            pt,
            e[8 as ::core::ffi::c_int as usize],
            e[6 as ::core::ffi::c_int as usize],
            e[2 as ::core::ffi::c_int as usize],
        )
        .is_null()
    {
        return -(2 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn simplexToPolytope2(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut simplex: *const ccd_simplex_t,
    mut pt: *mut ccd_pt_t,
    mut nearest: *mut *mut ccd_pt_el_t,
) -> ::core::ffi::c_int {
    let mut a: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut b: *const ccd_support_t = ::core::ptr::null::<ccd_support_t>();
    let mut ab: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut ac: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut dir: ccd_vec3_t = _ccd_vec3_t { v: [0.; 3] };
    let mut supp: [ccd_support_t; 4] = [_ccd_support_t {
        v: _ccd_vec3_t { v: [0.; 3] },
        v1: _ccd_vec3_t { v: [0.; 3] },
        v2: _ccd_vec3_t { v: [0.; 3] },
    }; 4];
    let mut v: [*mut ccd_pt_vertex_t; 6] = [::core::ptr::null_mut::<ccd_pt_vertex_t>(); 6];
    let mut e: [*mut ccd_pt_edge_t; 12] = [::core::ptr::null_mut::<ccd_pt_edge_t>(); 12];
    let mut i: size_t = 0;
    let mut found: ::core::ffi::c_int = 0;
    a = ccdSimplexPoint(simplex, 0 as ::core::ffi::c_int);
    b = ccdSimplexPoint(simplex, 1 as ::core::ffi::c_int);
    found = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < ccd_points_on_sphere_len {
        __ccdSupport(
            obj1,
            obj2,
            ccd_points_on_sphere.offset(i as isize) as *mut ccd_vec3_t,
            ccd,
            (&raw mut supp as *mut ccd_support_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut ccd_support_t,
        );
        if ccdVec3Eq(
            &raw const (*a).v,
            &raw mut (*(&raw mut supp as *mut ccd_support_t)
                .offset(0 as ::core::ffi::c_int as isize))
            .v,
        ) == 0
            && ccdVec3Eq(
                &raw const (*b).v,
                &raw mut (*(&raw mut supp as *mut ccd_support_t)
                    .offset(0 as ::core::ffi::c_int as isize))
                .v,
            ) == 0
        {
            found = 1 as ::core::ffi::c_int;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if !(found == 0) {
        ccdVec3Copy(
            &raw mut dir,
            &raw mut (*(&raw mut supp as *mut ccd_support_t)
                .offset(0 as ::core::ffi::c_int as isize))
            .v,
        );
        ccdVec3Scale(&raw mut dir, -CCD_ONE);
        __ccdSupport(
            obj1,
            obj2,
            &raw mut dir,
            ccd,
            (&raw mut supp as *mut ccd_support_t).offset(1 as ::core::ffi::c_int as isize)
                as *mut ccd_support_t,
        );
        if !(ccdVec3Eq(
            &raw const (*a).v,
            &raw mut (*(&raw mut supp as *mut ccd_support_t)
                .offset(1 as ::core::ffi::c_int as isize))
            .v,
        ) != 0
            || ccdVec3Eq(
                &raw const (*b).v,
                &raw mut (*(&raw mut supp as *mut ccd_support_t)
                    .offset(1 as ::core::ffi::c_int as isize))
                .v,
            ) != 0)
        {
            ccdVec3Sub2(
                &raw mut ab,
                &raw mut (*(&raw mut supp as *mut ccd_support_t)
                    .offset(0 as ::core::ffi::c_int as isize))
                .v,
                &raw const (*a).v,
            );
            ccdVec3Sub2(
                &raw mut ac,
                &raw mut (*(&raw mut supp as *mut ccd_support_t)
                    .offset(1 as ::core::ffi::c_int as isize))
                .v,
                &raw const (*a).v,
            );
            ccdVec3Cross(&raw mut dir, &raw mut ab, &raw mut ac);
            __ccdSupport(
                obj1,
                obj2,
                &raw mut dir,
                ccd,
                (&raw mut supp as *mut ccd_support_t).offset(2 as ::core::ffi::c_int as isize)
                    as *mut ccd_support_t,
            );
            if !(ccdVec3Eq(
                &raw const (*a).v,
                &raw mut (*(&raw mut supp as *mut ccd_support_t)
                    .offset(2 as ::core::ffi::c_int as isize))
                .v,
            ) != 0
                || ccdVec3Eq(
                    &raw const (*b).v,
                    &raw mut (*(&raw mut supp as *mut ccd_support_t)
                        .offset(2 as ::core::ffi::c_int as isize))
                    .v,
                ) != 0)
            {
                ccdVec3Scale(&raw mut dir, -CCD_ONE);
                __ccdSupport(
                    obj1,
                    obj2,
                    &raw mut dir,
                    ccd,
                    (&raw mut supp as *mut ccd_support_t).offset(3 as ::core::ffi::c_int as isize)
                        as *mut ccd_support_t,
                );
                if !(ccdVec3Eq(
                    &raw const (*a).v,
                    &raw mut (*(&raw mut supp as *mut ccd_support_t)
                        .offset(3 as ::core::ffi::c_int as isize))
                    .v,
                ) != 0
                    || ccdVec3Eq(
                        &raw const (*b).v,
                        &raw mut (*(&raw mut supp as *mut ccd_support_t)
                            .offset(3 as ::core::ffi::c_int as isize))
                        .v,
                    ) != 0)
                {
                    v[0 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, a);
                    v[1 as ::core::ffi::c_int as usize] = ccdPtAddVertex(
                        pt,
                        (&raw mut supp as *mut ccd_support_t)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut ccd_support_t,
                    );
                    v[2 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, b);
                    v[3 as ::core::ffi::c_int as usize] = ccdPtAddVertex(
                        pt,
                        (&raw mut supp as *mut ccd_support_t)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut ccd_support_t,
                    );
                    v[4 as ::core::ffi::c_int as usize] = ccdPtAddVertex(
                        pt,
                        (&raw mut supp as *mut ccd_support_t)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut ccd_support_t,
                    );
                    v[5 as ::core::ffi::c_int as usize] = ccdPtAddVertex(
                        pt,
                        (&raw mut supp as *mut ccd_support_t)
                            .offset(3 as ::core::ffi::c_int as isize)
                            as *mut ccd_support_t,
                    );
                    e[0 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[0 as ::core::ffi::c_int as usize],
                        v[1 as ::core::ffi::c_int as usize],
                    );
                    e[1 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[1 as ::core::ffi::c_int as usize],
                        v[2 as ::core::ffi::c_int as usize],
                    );
                    e[2 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[2 as ::core::ffi::c_int as usize],
                        v[3 as ::core::ffi::c_int as usize],
                    );
                    e[3 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[3 as ::core::ffi::c_int as usize],
                        v[0 as ::core::ffi::c_int as usize],
                    );
                    e[4 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[4 as ::core::ffi::c_int as usize],
                        v[0 as ::core::ffi::c_int as usize],
                    );
                    e[5 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[4 as ::core::ffi::c_int as usize],
                        v[1 as ::core::ffi::c_int as usize],
                    );
                    e[6 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[4 as ::core::ffi::c_int as usize],
                        v[2 as ::core::ffi::c_int as usize],
                    );
                    e[7 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[4 as ::core::ffi::c_int as usize],
                        v[3 as ::core::ffi::c_int as usize],
                    );
                    e[8 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[5 as ::core::ffi::c_int as usize],
                        v[0 as ::core::ffi::c_int as usize],
                    );
                    e[9 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[5 as ::core::ffi::c_int as usize],
                        v[1 as ::core::ffi::c_int as usize],
                    );
                    e[10 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[5 as ::core::ffi::c_int as usize],
                        v[2 as ::core::ffi::c_int as usize],
                    );
                    e[11 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                        pt,
                        v[5 as ::core::ffi::c_int as usize],
                        v[3 as ::core::ffi::c_int as usize],
                    );
                    if ccdPtAddFace(
                        pt,
                        e[4 as ::core::ffi::c_int as usize],
                        e[5 as ::core::ffi::c_int as usize],
                        e[0 as ::core::ffi::c_int as usize],
                    )
                    .is_null()
                        || ccdPtAddFace(
                            pt,
                            e[5 as ::core::ffi::c_int as usize],
                            e[6 as ::core::ffi::c_int as usize],
                            e[1 as ::core::ffi::c_int as usize],
                        )
                        .is_null()
                        || ccdPtAddFace(
                            pt,
                            e[6 as ::core::ffi::c_int as usize],
                            e[7 as ::core::ffi::c_int as usize],
                            e[2 as ::core::ffi::c_int as usize],
                        )
                        .is_null()
                        || ccdPtAddFace(
                            pt,
                            e[7 as ::core::ffi::c_int as usize],
                            e[4 as ::core::ffi::c_int as usize],
                            e[3 as ::core::ffi::c_int as usize],
                        )
                        .is_null()
                        || ccdPtAddFace(
                            pt,
                            e[8 as ::core::ffi::c_int as usize],
                            e[9 as ::core::ffi::c_int as usize],
                            e[0 as ::core::ffi::c_int as usize],
                        )
                        .is_null()
                        || ccdPtAddFace(
                            pt,
                            e[9 as ::core::ffi::c_int as usize],
                            e[10 as ::core::ffi::c_int as usize],
                            e[1 as ::core::ffi::c_int as usize],
                        )
                        .is_null()
                        || ccdPtAddFace(
                            pt,
                            e[10 as ::core::ffi::c_int as usize],
                            e[11 as ::core::ffi::c_int as usize],
                            e[2 as ::core::ffi::c_int as usize],
                        )
                        .is_null()
                        || ccdPtAddFace(
                            pt,
                            e[11 as ::core::ffi::c_int as usize],
                            e[8 as ::core::ffi::c_int as usize],
                            e[3 as ::core::ffi::c_int as usize],
                        )
                        .is_null()
                    {
                        return -(2 as ::core::ffi::c_int);
                    }
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    v[0 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, a);
    v[1 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, b);
    *nearest = ccdPtAddEdge(
        pt,
        v[0 as ::core::ffi::c_int as usize],
        v[1 as ::core::ffi::c_int as usize],
    ) as *mut ccd_pt_el_t;
    if (*nearest).is_null() {
        return -(2 as ::core::ffi::c_int);
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn expandPolytope(
    mut pt: *mut ccd_pt_t,
    mut el: *mut ccd_pt_el_t,
    mut newv: *const ccd_support_t,
) -> ::core::ffi::c_int {
    let mut v: [*mut ccd_pt_vertex_t; 5] = [::core::ptr::null_mut::<ccd_pt_vertex_t>(); 5];
    let mut e: [*mut ccd_pt_edge_t; 8] = [::core::ptr::null_mut::<ccd_pt_edge_t>(); 8];
    let mut f: [*mut ccd_pt_face_t; 2] = [::core::ptr::null_mut::<ccd_pt_face_t>(); 2];
    if (*el).type_0 == CCD_PT_EDGE {
        ccdPtEdgeVertices(
            el as *const ccd_pt_edge_t,
            (&raw mut v as *mut *mut ccd_pt_vertex_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_vertex_t,
            (&raw mut v as *mut *mut ccd_pt_vertex_t).offset(2 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_vertex_t,
        );
        ccdPtEdgeFaces(
            el as *mut ccd_pt_edge_t,
            (&raw mut f as *mut *mut ccd_pt_face_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_face_t,
            (&raw mut f as *mut *mut ccd_pt_face_t).offset(1 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_face_t,
        );
        if !f[0 as ::core::ffi::c_int as usize].is_null() {
            ccdPtFaceEdges(
                f[0 as ::core::ffi::c_int as usize],
                (&raw mut e as *mut *mut ccd_pt_edge_t).offset(0 as ::core::ffi::c_int as isize)
                    as *mut *mut ccd_pt_edge_t,
                (&raw mut e as *mut *mut ccd_pt_edge_t).offset(1 as ::core::ffi::c_int as isize)
                    as *mut *mut ccd_pt_edge_t,
                (&raw mut e as *mut *mut ccd_pt_edge_t).offset(2 as ::core::ffi::c_int as isize)
                    as *mut *mut ccd_pt_edge_t,
            );
            if e[0 as ::core::ffi::c_int as usize] == el as *mut ccd_pt_edge_t {
                e[0 as ::core::ffi::c_int as usize] = e[2 as ::core::ffi::c_int as usize];
            } else if e[1 as ::core::ffi::c_int as usize] == el as *mut ccd_pt_edge_t {
                e[1 as ::core::ffi::c_int as usize] = e[2 as ::core::ffi::c_int as usize];
            }
            ccdPtEdgeVertices(
                e[0 as ::core::ffi::c_int as usize],
                (&raw mut v as *mut *mut ccd_pt_vertex_t).offset(1 as ::core::ffi::c_int as isize)
                    as *mut *mut ccd_pt_vertex_t,
                (&raw mut v as *mut *mut ccd_pt_vertex_t).offset(3 as ::core::ffi::c_int as isize)
                    as *mut *mut ccd_pt_vertex_t,
            );
            if v[1 as ::core::ffi::c_int as usize] != v[0 as ::core::ffi::c_int as usize]
                && v[3 as ::core::ffi::c_int as usize] != v[0 as ::core::ffi::c_int as usize]
            {
                e[2 as ::core::ffi::c_int as usize] = e[0 as ::core::ffi::c_int as usize];
                e[0 as ::core::ffi::c_int as usize] = e[1 as ::core::ffi::c_int as usize];
                e[1 as ::core::ffi::c_int as usize] = e[2 as ::core::ffi::c_int as usize];
                if v[1 as ::core::ffi::c_int as usize] == v[2 as ::core::ffi::c_int as usize] {
                    v[1 as ::core::ffi::c_int as usize] = v[3 as ::core::ffi::c_int as usize];
                }
            } else if v[1 as ::core::ffi::c_int as usize] == v[0 as ::core::ffi::c_int as usize] {
                v[1 as ::core::ffi::c_int as usize] = v[3 as ::core::ffi::c_int as usize];
            }
            if !f[1 as ::core::ffi::c_int as usize].is_null() {
                ccdPtFaceEdges(
                    f[1 as ::core::ffi::c_int as usize],
                    (&raw mut e as *mut *mut ccd_pt_edge_t).offset(2 as ::core::ffi::c_int as isize)
                        as *mut *mut ccd_pt_edge_t,
                    (&raw mut e as *mut *mut ccd_pt_edge_t).offset(3 as ::core::ffi::c_int as isize)
                        as *mut *mut ccd_pt_edge_t,
                    (&raw mut e as *mut *mut ccd_pt_edge_t).offset(4 as ::core::ffi::c_int as isize)
                        as *mut *mut ccd_pt_edge_t,
                );
                if e[2 as ::core::ffi::c_int as usize] == el as *mut ccd_pt_edge_t {
                    e[2 as ::core::ffi::c_int as usize] = e[4 as ::core::ffi::c_int as usize];
                } else if e[3 as ::core::ffi::c_int as usize] == el as *mut ccd_pt_edge_t {
                    e[3 as ::core::ffi::c_int as usize] = e[4 as ::core::ffi::c_int as usize];
                }
                ccdPtEdgeVertices(
                    e[2 as ::core::ffi::c_int as usize],
                    (&raw mut v as *mut *mut ccd_pt_vertex_t)
                        .offset(3 as ::core::ffi::c_int as isize)
                        as *mut *mut ccd_pt_vertex_t,
                    (&raw mut v as *mut *mut ccd_pt_vertex_t)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut *mut ccd_pt_vertex_t,
                );
                if v[3 as ::core::ffi::c_int as usize] != v[2 as ::core::ffi::c_int as usize]
                    && v[4 as ::core::ffi::c_int as usize] != v[2 as ::core::ffi::c_int as usize]
                {
                    e[4 as ::core::ffi::c_int as usize] = e[2 as ::core::ffi::c_int as usize];
                    e[2 as ::core::ffi::c_int as usize] = e[3 as ::core::ffi::c_int as usize];
                    e[3 as ::core::ffi::c_int as usize] = e[4 as ::core::ffi::c_int as usize];
                    if v[3 as ::core::ffi::c_int as usize] == v[0 as ::core::ffi::c_int as usize] {
                        v[3 as ::core::ffi::c_int as usize] = v[4 as ::core::ffi::c_int as usize];
                    }
                } else if v[3 as ::core::ffi::c_int as usize] == v[2 as ::core::ffi::c_int as usize]
                {
                    v[3 as ::core::ffi::c_int as usize] = v[4 as ::core::ffi::c_int as usize];
                }
            }
            v[4 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, newv);
            ccdPtDelFace(pt, f[0 as ::core::ffi::c_int as usize]);
            if !f[1 as ::core::ffi::c_int as usize].is_null() {
                ccdPtDelFace(pt, f[1 as ::core::ffi::c_int as usize]);
                ccdPtDelEdge(pt, el as *mut ccd_pt_edge_t);
            }
            e[4 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                pt,
                v[4 as ::core::ffi::c_int as usize],
                v[2 as ::core::ffi::c_int as usize],
            );
            e[5 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                pt,
                v[4 as ::core::ffi::c_int as usize],
                v[0 as ::core::ffi::c_int as usize],
            );
            e[6 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                pt,
                v[4 as ::core::ffi::c_int as usize],
                v[1 as ::core::ffi::c_int as usize],
            );
            if !f[1 as ::core::ffi::c_int as usize].is_null() {
                e[7 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
                    pt,
                    v[4 as ::core::ffi::c_int as usize],
                    v[3 as ::core::ffi::c_int as usize],
                );
            }
            if ccdPtAddFace(
                pt,
                e[1 as ::core::ffi::c_int as usize],
                e[4 as ::core::ffi::c_int as usize],
                e[6 as ::core::ffi::c_int as usize],
            )
            .is_null()
                || ccdPtAddFace(
                    pt,
                    e[0 as ::core::ffi::c_int as usize],
                    e[6 as ::core::ffi::c_int as usize],
                    e[5 as ::core::ffi::c_int as usize],
                )
                .is_null()
            {
                return -(2 as ::core::ffi::c_int);
            }
            if !f[1 as ::core::ffi::c_int as usize].is_null() {
                if ccdPtAddFace(
                    pt,
                    e[3 as ::core::ffi::c_int as usize],
                    e[5 as ::core::ffi::c_int as usize],
                    e[7 as ::core::ffi::c_int as usize],
                )
                .is_null()
                    || ccdPtAddFace(
                        pt,
                        e[4 as ::core::ffi::c_int as usize],
                        e[7 as ::core::ffi::c_int as usize],
                        e[2 as ::core::ffi::c_int as usize],
                    )
                    .is_null()
                {
                    return -(2 as ::core::ffi::c_int);
                }
            } else if ccdPtAddFace(
                pt,
                e[4 as ::core::ffi::c_int as usize],
                e[5 as ::core::ffi::c_int as usize],
                el as *mut ccd_pt_edge_t,
            )
            .is_null()
            {
                return -(2 as ::core::ffi::c_int);
            }
        }
    } else {
        ccdPtFaceEdges(
            el as *const ccd_pt_face_t,
            (&raw mut e as *mut *mut ccd_pt_edge_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_edge_t,
            (&raw mut e as *mut *mut ccd_pt_edge_t).offset(1 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_edge_t,
            (&raw mut e as *mut *mut ccd_pt_edge_t).offset(2 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_edge_t,
        );
        ccdPtEdgeVertices(
            e[0 as ::core::ffi::c_int as usize],
            (&raw mut v as *mut *mut ccd_pt_vertex_t).offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_vertex_t,
            (&raw mut v as *mut *mut ccd_pt_vertex_t).offset(1 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_vertex_t,
        );
        ccdPtEdgeVertices(
            e[1 as ::core::ffi::c_int as usize],
            (&raw mut v as *mut *mut ccd_pt_vertex_t).offset(2 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_vertex_t,
            (&raw mut v as *mut *mut ccd_pt_vertex_t).offset(3 as ::core::ffi::c_int as isize)
                as *mut *mut ccd_pt_vertex_t,
        );
        if v[2 as ::core::ffi::c_int as usize] != v[1 as ::core::ffi::c_int as usize]
            && v[3 as ::core::ffi::c_int as usize] != v[1 as ::core::ffi::c_int as usize]
        {
            e[3 as ::core::ffi::c_int as usize] = e[1 as ::core::ffi::c_int as usize];
            e[1 as ::core::ffi::c_int as usize] = e[2 as ::core::ffi::c_int as usize];
            e[2 as ::core::ffi::c_int as usize] = e[3 as ::core::ffi::c_int as usize];
        }
        if v[3 as ::core::ffi::c_int as usize] != v[0 as ::core::ffi::c_int as usize]
            && v[3 as ::core::ffi::c_int as usize] != v[1 as ::core::ffi::c_int as usize]
        {
            v[2 as ::core::ffi::c_int as usize] = v[3 as ::core::ffi::c_int as usize];
        }
        ccdPtDelFace(pt, el as *mut ccd_pt_face_t);
        v[3 as ::core::ffi::c_int as usize] = ccdPtAddVertex(pt, newv);
        e[3 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
            pt,
            v[3 as ::core::ffi::c_int as usize],
            v[0 as ::core::ffi::c_int as usize],
        );
        e[4 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
            pt,
            v[3 as ::core::ffi::c_int as usize],
            v[1 as ::core::ffi::c_int as usize],
        );
        e[5 as ::core::ffi::c_int as usize] = ccdPtAddEdge(
            pt,
            v[3 as ::core::ffi::c_int as usize],
            v[2 as ::core::ffi::c_int as usize],
        );
        if ccdPtAddFace(
            pt,
            e[3 as ::core::ffi::c_int as usize],
            e[4 as ::core::ffi::c_int as usize],
            e[0 as ::core::ffi::c_int as usize],
        )
        .is_null()
            || ccdPtAddFace(
                pt,
                e[4 as ::core::ffi::c_int as usize],
                e[5 as ::core::ffi::c_int as usize],
                e[1 as ::core::ffi::c_int as usize],
            )
            .is_null()
            || ccdPtAddFace(
                pt,
                e[5 as ::core::ffi::c_int as usize],
                e[3 as ::core::ffi::c_int as usize],
                e[2 as ::core::ffi::c_int as usize],
            )
            .is_null()
        {
            return -(2 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn nextSupport(
    mut obj1: *const ::core::ffi::c_void,
    mut obj2: *const ::core::ffi::c_void,
    mut ccd: *const ccd_t,
    mut el: *const ccd_pt_el_t,
    mut out: *mut ccd_support_t,
) -> ::core::ffi::c_int {
    let mut a: *mut ccd_vec3_t = ::core::ptr::null_mut::<ccd_vec3_t>();
    let mut b: *mut ccd_vec3_t = ::core::ptr::null_mut::<ccd_vec3_t>();
    let mut c: *mut ccd_vec3_t = ::core::ptr::null_mut::<ccd_vec3_t>();
    let mut dist: ccd_real_t = 0.;
    if (*el).type_0 == CCD_PT_VERTEX {
        return -(1 as ::core::ffi::c_int);
    }
    if ccdIsZero((*el).dist) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    __ccdSupport(obj1, obj2, &raw const (*el).witness, ccd, out);
    dist = ccdVec3Dot(&raw mut (*out).v, &raw const (*el).witness);
    if dist - (*el).dist < (*ccd).epa_tolerance {
        return -(1 as ::core::ffi::c_int);
    }
    if (*el).type_0 == CCD_PT_EDGE {
        ccdPtEdgeVec3(el as *mut ccd_pt_edge_t, &raw mut a, &raw mut b);
        dist = ccdVec3PointSegmentDist2(
            &raw mut (*out).v,
            a,
            b,
            ::core::ptr::null_mut::<ccd_vec3_t>(),
        );
    } else {
        ccdPtFaceVec3(el as *mut ccd_pt_face_t, &raw mut a, &raw mut b, &raw mut c);
        dist = ccdVec3PointTriDist2(
            &raw mut (*out).v,
            a,
            b,
            c,
            ::core::ptr::null_mut::<ccd_vec3_t>(),
        );
    }
    if dist < (*ccd).epa_tolerance {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdSupportCopy(mut d: *mut ccd_support_t, mut s: *const ccd_support_t) {
    *d = *s;
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexInit(mut s: *mut ccd_simplex_t) {
    (*s).last = -(1 as ::core::ffi::c_int);
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexSize(mut s: *const ccd_simplex_t) -> ::core::ffi::c_int {
    return (*s).last + 1 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdSimplexLast(mut s: *const ccd_simplex_t) -> *const ccd_support_t {
    return ccdSimplexPoint(s, (*s).last);
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
unsafe extern "C" fn ccdSimplexAdd(mut s: *mut ccd_simplex_t, mut v: *const ccd_support_t) {
    (*s).last += 1;
    ccdSupportCopy(
        (&raw mut (*s).ps as *mut ccd_support_t).offset((*s).last as isize),
        v,
    );
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
pub const CCD_PT_VERTEX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CCD_PT_EDGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn ccdPtDelEdge(
    mut pt: *mut ccd_pt_t,
    mut e: *mut ccd_pt_edge_t,
) -> ::core::ffi::c_int {
    if !(*e).faces[0 as ::core::ffi::c_int as usize].is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ccdListDel(
        (&raw mut (*e).vertex_list as *mut ccd_list_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut ccd_list_t,
    );
    ccdListDel(
        (&raw mut (*e).vertex_list as *mut ccd_list_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut ccd_list_t,
    );
    ccdListDel(&raw mut (*e).list);
    if (*pt).nearest as *mut ::core::ffi::c_void == e as *mut ::core::ffi::c_void {
        (*pt).nearest = ::core::ptr::null_mut::<ccd_pt_el_t>();
    }
    free(e as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdPtDelFace(
    mut pt: *mut ccd_pt_t,
    mut f: *mut ccd_pt_face_t,
) -> ::core::ffi::c_int {
    let mut e: *mut ccd_pt_edge_t = ::core::ptr::null_mut::<ccd_pt_edge_t>();
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 3 as size_t {
        e = (*f).edge[i as usize];
        if (*e).faces[0 as ::core::ffi::c_int as usize] == f {
            (*e).faces[0 as ::core::ffi::c_int as usize] =
                (*e).faces[1 as ::core::ffi::c_int as usize];
        }
        (*e).faces[1 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<_ccd_pt_face_t>();
        i = i.wrapping_add(1);
    }
    ccdListDel(&raw mut (*f).list);
    if (*pt).nearest as *mut ::core::ffi::c_void == f as *mut ::core::ffi::c_void {
        (*pt).nearest = ::core::ptr::null_mut::<ccd_pt_el_t>();
    }
    free(f as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdPtFaceVec3(
    mut face: *const ccd_pt_face_t,
    mut a: *mut *mut ccd_vec3_t,
    mut b: *mut *mut ccd_vec3_t,
    mut c: *mut *mut ccd_vec3_t,
) {
    *a = &raw mut (**(&raw mut (**(&raw const (*face).edge as *const *mut ccd_pt_edge_t)
        .offset(0 as ::core::ffi::c_int as isize))
    .vertex as *mut *mut ccd_pt_vertex_t)
        .offset(0 as ::core::ffi::c_int as isize))
    .v
    .v;
    *b = &raw mut (**(&raw mut (**(&raw const (*face).edge as *const *mut ccd_pt_edge_t)
        .offset(0 as ::core::ffi::c_int as isize))
    .vertex as *mut *mut ccd_pt_vertex_t)
        .offset(1 as ::core::ffi::c_int as isize))
    .v
    .v;
    if (*(*face).edge[1 as ::core::ffi::c_int as usize]).vertex[0 as ::core::ffi::c_int as usize]
        != (*(*face).edge[0 as ::core::ffi::c_int as usize]).vertex
            [0 as ::core::ffi::c_int as usize]
        && (*(*face).edge[1 as ::core::ffi::c_int as usize]).vertex
            [0 as ::core::ffi::c_int as usize]
            != (*(*face).edge[0 as ::core::ffi::c_int as usize]).vertex
                [1 as ::core::ffi::c_int as usize]
    {
        *c = &raw mut (**(&raw mut (**(&raw const (*face).edge as *const *mut ccd_pt_edge_t)
            .offset(1 as ::core::ffi::c_int as isize))
        .vertex as *mut *mut ccd_pt_vertex_t)
            .offset(0 as ::core::ffi::c_int as isize))
        .v
        .v;
    } else {
        *c = &raw mut (**(&raw mut (**(&raw const (*face).edge as *const *mut ccd_pt_edge_t)
            .offset(1 as ::core::ffi::c_int as isize))
        .vertex as *mut *mut ccd_pt_vertex_t)
            .offset(1 as ::core::ffi::c_int as isize))
        .v
        .v;
    };
}
#[inline(always)]
unsafe extern "C" fn ccdPtFaceEdges(
    mut f: *const ccd_pt_face_t,
    mut a: *mut *mut ccd_pt_edge_t,
    mut b: *mut *mut ccd_pt_edge_t,
    mut c: *mut *mut ccd_pt_edge_t,
) {
    *a = (*f).edge[0 as ::core::ffi::c_int as usize];
    *b = (*f).edge[1 as ::core::ffi::c_int as usize];
    *c = (*f).edge[2 as ::core::ffi::c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn ccdPtEdgeVec3(
    mut e: *const ccd_pt_edge_t,
    mut a: *mut *mut ccd_vec3_t,
    mut b: *mut *mut ccd_vec3_t,
) {
    *a = &raw mut (**(&raw const (*e).vertex as *const *mut ccd_pt_vertex_t)
        .offset(0 as ::core::ffi::c_int as isize))
    .v
    .v;
    *b = &raw mut (**(&raw const (*e).vertex as *const *mut ccd_pt_vertex_t)
        .offset(1 as ::core::ffi::c_int as isize))
    .v
    .v;
}
#[inline(always)]
unsafe extern "C" fn ccdPtEdgeVertices(
    mut e: *const ccd_pt_edge_t,
    mut a: *mut *mut ccd_pt_vertex_t,
    mut b: *mut *mut ccd_pt_vertex_t,
) {
    *a = (*e).vertex[0 as ::core::ffi::c_int as usize];
    *b = (*e).vertex[1 as ::core::ffi::c_int as usize];
}
#[inline(always)]
unsafe extern "C" fn ccdPtEdgeFaces(
    mut e: *const ccd_pt_edge_t,
    mut f1: *mut *mut ccd_pt_face_t,
    mut f2: *mut *mut ccd_pt_face_t,
) {
    *f1 = (*e).faces[0 as ::core::ffi::c_int as usize] as *mut ccd_pt_face_t;
    *f2 = (*e).faces[1 as ::core::ffi::c_int as usize] as *mut ccd_pt_face_t;
}
#[inline(always)]
unsafe extern "C" fn ccdListDel(mut item: *mut ccd_list_t) {
    (*(*item).next).prev = (*item).prev;
    (*(*item).prev).next = (*item).next;
    (*item).next = item as *mut _ccd_list_t;
    (*item).prev = item as *mut _ccd_list_t;
}
pub const FLT_EPSILON: ::core::ffi::c_float = __FLT_EPSILON__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __FLT_EPSILON__: ::core::ffi::c_float = 1.19209290e-7f32;
