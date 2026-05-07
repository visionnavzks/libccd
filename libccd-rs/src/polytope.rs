unsafe extern "C" {
    //pub type _IO_wide_data = ::core::ffi::c_void;
    //pub type _IO_codecvt = ::core::ffi::c_void;
    //pub type _IO_marker = ::core::ffi::c_void;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn fabsf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    static mut ccd_vec3_origin: *mut ccd_vec3_t;
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
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut ::core::ffi::c_void,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut ::core::ffi::c_void,
    pub _wide_data: *mut ::core::ffi::c_void,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ccd_real_t = ::core::ffi::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_vec3_t {
    pub v: [ccd_real_t; 3],
}
pub type ccd_vec3_t = _ccd_vec3_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_support_t {
    pub v: ccd_vec3_t,
    pub v1: ccd_vec3_t,
    pub v2: ccd_vec3_t,
}
pub type ccd_support_t = _ccd_support_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_list_t {
    pub next: *mut _ccd_list_t,
    pub prev: *mut _ccd_list_t,
}
pub type ccd_list_t = _ccd_list_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ccd_pt_el_t {
    pub type_0: ::core::ffi::c_int,
    pub dist: ccd_real_t,
    pub witness: ccd_vec3_t,
    pub list: ccd_list_t,
}
pub type ccd_pt_el_t = _ccd_pt_el_t;
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
pub type ccd_pt_face_t = _ccd_pt_face_t;
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
pub type ccd_pt_t = _ccd_pt_t;
pub const CCD_EPS: ::core::ffi::c_float = FLT_EPSILON;
pub const CCD_REAL_MAX: ::core::ffi::c_float = FLT_MAX;
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
unsafe extern "C" fn ccdVec3Len2(mut v: *const ccd_vec3_t) -> ccd_real_t {
    return ccdVec3Dot(v, v);
}
#[inline(always)]
unsafe extern "C" fn ccdVec3Copy(mut v: *mut ccd_vec3_t, mut w: *const ccd_vec3_t) {
    *v = *w;
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
unsafe extern "C" fn ccdSupportCopy(mut d: *mut ccd_support_t, mut s: *const ccd_support_t) {
    *d = *s;
}
pub const CCD_PT_VERTEX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CCD_PT_EDGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CCD_PT_FACE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn ccdPtDelVertex(
    mut pt: *mut ccd_pt_t,
    mut v: *mut ccd_pt_vertex_t,
) -> ::core::ffi::c_int {
    if ccdListEmpty(&raw mut (*v).edges) == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    ccdListDel(&raw mut (*v).list);
    if (*pt).nearest as *mut ::core::ffi::c_void == v as *mut ::core::ffi::c_void {
        (*pt).nearest = ::core::ptr::null_mut::<ccd_pt_el_t>();
    }
    free(v as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
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
unsafe extern "C" fn _ccdPtNearestUpdate(mut pt: *mut ccd_pt_t, mut el: *mut ccd_pt_el_t) {
    if ccdEq((*pt).nearest_dist, (*el).dist) != 0 {
        if (*el).type_0 < (*pt).nearest_type {
            (*pt).nearest = el;
            (*pt).nearest_dist = (*el).dist;
            (*pt).nearest_type = (*el).type_0;
        }
    } else if (*el).dist < (*pt).nearest_dist {
        (*pt).nearest = el;
        (*pt).nearest_dist = (*el).dist;
        (*pt).nearest_type = (*el).type_0;
    }
}
unsafe extern "C" fn _ccdPtNearestRenew(mut pt: *mut ccd_pt_t) {
    let mut v: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut e: *mut ccd_pt_edge_t = ::core::ptr::null_mut::<ccd_pt_edge_t>();
    let mut f: *mut ccd_pt_face_t = ::core::ptr::null_mut::<ccd_pt_face_t>();
    (*pt).nearest_dist = CCD_REAL_MAX as ccd_real_t;
    (*pt).nearest_type = 3 as ::core::ffi::c_int;
    (*pt).nearest = ::core::ptr::null_mut::<ccd_pt_el_t>();
    v = ((*pt).vertices.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    while &raw mut (*v).list != &raw mut (*pt).vertices {
        _ccdPtNearestUpdate(pt, v as *mut ccd_pt_el_t);
        v = ((*v).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    }
    e = ((*pt).edges.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_edge_t;
    while &raw mut (*e).list != &raw mut (*pt).edges {
        _ccdPtNearestUpdate(pt, e as *mut ccd_pt_el_t);
        e = ((*e).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_edge_t;
    }
    f = ((*pt).faces.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_face_t;
    while &raw mut (*f).list != &raw mut (*pt).faces {
        _ccdPtNearestUpdate(pt, f as *mut ccd_pt_el_t);
        f = ((*f).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_face_t;
    }
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtInit(mut pt: *mut ccd_pt_t) {
    ccdListInit(&raw mut (*pt).vertices);
    ccdListInit(&raw mut (*pt).edges);
    ccdListInit(&raw mut (*pt).faces);
    (*pt).nearest = ::core::ptr::null_mut::<ccd_pt_el_t>();
    (*pt).nearest_dist = CCD_REAL_MAX as ccd_real_t;
    (*pt).nearest_type = 3 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtDestroy(mut pt: *mut ccd_pt_t) {
    let mut f: *mut ccd_pt_face_t = ::core::ptr::null_mut::<ccd_pt_face_t>();
    let mut f2: *mut ccd_pt_face_t = ::core::ptr::null_mut::<ccd_pt_face_t>();
    let mut e: *mut ccd_pt_edge_t = ::core::ptr::null_mut::<ccd_pt_edge_t>();
    let mut e2: *mut ccd_pt_edge_t = ::core::ptr::null_mut::<ccd_pt_edge_t>();
    let mut v: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut v2: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    f = ((*pt).faces.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_face_t;
    f2 = ((*f).list.next as *mut ::core::ffi::c_char).offset(-(24 as ::core::ffi::c_ulong as isize))
        as *mut ccd_pt_face_t;
    while &raw mut (*f).list != &raw mut (*pt).faces {
        ccdPtDelFace(pt, f);
        f = f2;
        f2 = ((*f2).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_face_t;
    }
    e = ((*pt).edges.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_edge_t;
    e2 = ((*e).list.next as *mut ::core::ffi::c_char).offset(-(24 as ::core::ffi::c_ulong as isize))
        as *mut ccd_pt_edge_t;
    while &raw mut (*e).list != &raw mut (*pt).edges {
        ccdPtDelEdge(pt, e);
        e = e2;
        e2 = ((*e2).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_edge_t;
    }
    v = ((*pt).vertices.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    v2 = ((*v).list.next as *mut ::core::ffi::c_char).offset(-(24 as ::core::ffi::c_ulong as isize))
        as *mut ccd_pt_vertex_t;
    while &raw mut (*v).list != &raw mut (*pt).vertices {
        ccdPtDelVertex(pt, v);
        v = v2;
        v2 = ((*v2).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    }
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtAddVertex(
    mut pt: *mut ccd_pt_t,
    mut v: *const ccd_support_t,
) -> *mut ccd_pt_vertex_t {
    let mut vert: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    vert = realloc(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::mem::size_of::<ccd_pt_vertex_t>() as size_t,
    ) as *mut ccd_pt_vertex_t;
    if vert.is_null() {
        return ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    }
    (*vert).type_0 = CCD_PT_VERTEX;
    ccdSupportCopy(&raw mut (*vert).v, v);
    (*vert).dist = ccdVec3Len2(&raw mut (*vert).v.v);
    ccdVec3Copy(&raw mut (*vert).witness, &raw mut (*vert).v.v);
    ccdListInit(&raw mut (*vert).edges);
    ccdListAppend(&raw mut (*pt).vertices, &raw mut (*vert).list);
    _ccdPtNearestUpdate(pt, vert as *mut ccd_pt_el_t);
    return vert;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtAddEdge(
    mut pt: *mut ccd_pt_t,
    mut v1: *mut ccd_pt_vertex_t,
    mut v2: *mut ccd_pt_vertex_t,
) -> *mut ccd_pt_edge_t {
    let mut a: *const ccd_vec3_t = ::core::ptr::null::<ccd_vec3_t>();
    let mut b: *const ccd_vec3_t = ::core::ptr::null::<ccd_vec3_t>();
    let mut edge: *mut ccd_pt_edge_t = ::core::ptr::null_mut::<ccd_pt_edge_t>();
    if v1.is_null() || v2.is_null() {
        return ::core::ptr::null_mut::<ccd_pt_edge_t>();
    }
    edge = realloc(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::mem::size_of::<ccd_pt_edge_t>() as size_t,
    ) as *mut ccd_pt_edge_t;
    if edge.is_null() {
        return ::core::ptr::null_mut::<ccd_pt_edge_t>();
    }
    (*edge).type_0 = CCD_PT_EDGE;
    (*edge).vertex[0 as ::core::ffi::c_int as usize] = v1;
    (*edge).vertex[1 as ::core::ffi::c_int as usize] = v2;
    (*edge).faces[1 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<_ccd_pt_face_t>();
    (*edge).faces[0 as ::core::ffi::c_int as usize] =
        (*edge).faces[1 as ::core::ffi::c_int as usize];
    a = &raw mut (**(&raw mut (*edge).vertex as *mut *mut ccd_pt_vertex_t)
        .offset(0 as ::core::ffi::c_int as isize))
    .v
    .v;
    b = &raw mut (**(&raw mut (*edge).vertex as *mut *mut ccd_pt_vertex_t)
        .offset(1 as ::core::ffi::c_int as isize))
    .v
    .v;
    (*edge).dist = ccdVec3PointSegmentDist2(ccd_vec3_origin, a, b, &raw mut (*edge).witness);
    ccdListAppend(
        &raw mut (**(&raw mut (*edge).vertex as *mut *mut ccd_pt_vertex_t)
            .offset(0 as ::core::ffi::c_int as isize))
        .edges,
        (&raw mut (*edge).vertex_list as *mut ccd_list_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut ccd_list_t,
    );
    ccdListAppend(
        &raw mut (**(&raw mut (*edge).vertex as *mut *mut ccd_pt_vertex_t)
            .offset(1 as ::core::ffi::c_int as isize))
        .edges,
        (&raw mut (*edge).vertex_list as *mut ccd_list_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut ccd_list_t,
    );
    ccdListAppend(&raw mut (*pt).edges, &raw mut (*edge).list);
    _ccdPtNearestUpdate(pt, edge as *mut ccd_pt_el_t);
    return edge;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtAddFace(
    mut pt: *mut ccd_pt_t,
    mut e1: *mut ccd_pt_edge_t,
    mut e2: *mut ccd_pt_edge_t,
    mut e3: *mut ccd_pt_edge_t,
) -> *mut ccd_pt_face_t {
    let mut a: *const ccd_vec3_t = ::core::ptr::null::<ccd_vec3_t>();
    let mut b: *const ccd_vec3_t = ::core::ptr::null::<ccd_vec3_t>();
    let mut c: *const ccd_vec3_t = ::core::ptr::null::<ccd_vec3_t>();
    let mut face: *mut ccd_pt_face_t = ::core::ptr::null_mut::<ccd_pt_face_t>();
    let mut e: *mut ccd_pt_edge_t = ::core::ptr::null_mut::<ccd_pt_edge_t>();
    let mut i: size_t = 0;
    if e1.is_null() || e2.is_null() || e3.is_null() {
        return ::core::ptr::null_mut::<ccd_pt_face_t>();
    }
    face = realloc(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::mem::size_of::<ccd_pt_face_t>() as size_t,
    ) as *mut ccd_pt_face_t;
    if face.is_null() {
        return ::core::ptr::null_mut::<ccd_pt_face_t>();
    }
    (*face).type_0 = CCD_PT_FACE;
    (*face).edge[0 as ::core::ffi::c_int as usize] = e1;
    (*face).edge[1 as ::core::ffi::c_int as usize] = e2;
    (*face).edge[2 as ::core::ffi::c_int as usize] = e3;
    a = &raw mut (**(&raw mut (**(&raw mut (*face).edge as *mut *mut ccd_pt_edge_t)
        .offset(0 as ::core::ffi::c_int as isize))
    .vertex as *mut *mut ccd_pt_vertex_t)
        .offset(0 as ::core::ffi::c_int as isize))
    .v
    .v;
    b = &raw mut (**(&raw mut (**(&raw mut (*face).edge as *mut *mut ccd_pt_edge_t)
        .offset(0 as ::core::ffi::c_int as isize))
    .vertex as *mut *mut ccd_pt_vertex_t)
        .offset(1 as ::core::ffi::c_int as isize))
    .v
    .v;
    e = (*face).edge[1 as ::core::ffi::c_int as usize];
    if (*e).vertex[0 as ::core::ffi::c_int as usize]
        != (*(*face).edge[0 as ::core::ffi::c_int as usize]).vertex
            [0 as ::core::ffi::c_int as usize]
        && (*e).vertex[0 as ::core::ffi::c_int as usize]
            != (*(*face).edge[0 as ::core::ffi::c_int as usize]).vertex
                [1 as ::core::ffi::c_int as usize]
    {
        c = &raw mut (**(&raw mut (*e).vertex as *mut *mut ccd_pt_vertex_t)
            .offset(0 as ::core::ffi::c_int as isize))
        .v
        .v;
    } else {
        c = &raw mut (**(&raw mut (*e).vertex as *mut *mut ccd_pt_vertex_t)
            .offset(1 as ::core::ffi::c_int as isize))
        .v
        .v;
    }
    (*face).dist = ccdVec3PointTriDist2(ccd_vec3_origin, a, b, c, &raw mut (*face).witness);
    i = 0 as size_t;
    while i < 3 as size_t {
        if (*(*face).edge[i as usize]).faces[0 as ::core::ffi::c_int as usize].is_null() {
            (*(*face).edge[i as usize]).faces[0 as ::core::ffi::c_int as usize] =
                face as *mut _ccd_pt_face_t;
        } else {
            (*(*face).edge[i as usize]).faces[1 as ::core::ffi::c_int as usize] =
                face as *mut _ccd_pt_face_t;
        }
        i = i.wrapping_add(1);
    }
    ccdListAppend(&raw mut (*pt).faces, &raw mut (*face).list);
    _ccdPtNearestUpdate(pt, face as *mut ccd_pt_el_t);
    return face;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtRecomputeDistances(mut pt: *mut ccd_pt_t) {
    let mut v: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut e: *mut ccd_pt_edge_t = ::core::ptr::null_mut::<ccd_pt_edge_t>();
    let mut f: *mut ccd_pt_face_t = ::core::ptr::null_mut::<ccd_pt_face_t>();
    let mut a: *const ccd_vec3_t = ::core::ptr::null::<ccd_vec3_t>();
    let mut b: *const ccd_vec3_t = ::core::ptr::null::<ccd_vec3_t>();
    let mut c: *const ccd_vec3_t = ::core::ptr::null::<ccd_vec3_t>();
    let mut dist: ccd_real_t = 0.;
    v = ((*pt).vertices.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    while &raw mut (*v).list != &raw mut (*pt).vertices {
        dist = ccdVec3Len2(&raw mut (*v).v.v);
        (*v).dist = dist;
        ccdVec3Copy(&raw mut (*v).witness, &raw mut (*v).v.v);
        v = ((*v).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    }
    e = ((*pt).edges.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_edge_t;
    while &raw mut (*e).list != &raw mut (*pt).edges {
        a = &raw mut (**(&raw mut (*e).vertex as *mut *mut ccd_pt_vertex_t)
            .offset(0 as ::core::ffi::c_int as isize))
        .v
        .v;
        b = &raw mut (**(&raw mut (*e).vertex as *mut *mut ccd_pt_vertex_t)
            .offset(1 as ::core::ffi::c_int as isize))
        .v
        .v;
        dist = ccdVec3PointSegmentDist2(ccd_vec3_origin, a, b, &raw mut (*e).witness);
        (*e).dist = dist;
        e = ((*e).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_edge_t;
    }
    f = ((*pt).faces.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_face_t;
    while &raw mut (*f).list != &raw mut (*pt).faces {
        a = &raw mut (**(&raw mut (**(&raw mut (*f).edge as *mut *mut ccd_pt_edge_t)
            .offset(0 as ::core::ffi::c_int as isize))
        .vertex as *mut *mut ccd_pt_vertex_t)
            .offset(0 as ::core::ffi::c_int as isize))
        .v
        .v;
        b = &raw mut (**(&raw mut (**(&raw mut (*f).edge as *mut *mut ccd_pt_edge_t)
            .offset(0 as ::core::ffi::c_int as isize))
        .vertex as *mut *mut ccd_pt_vertex_t)
            .offset(1 as ::core::ffi::c_int as isize))
        .v
        .v;
        e = (*f).edge[1 as ::core::ffi::c_int as usize];
        if (*e).vertex[0 as ::core::ffi::c_int as usize]
            != (*(*f).edge[0 as ::core::ffi::c_int as usize]).vertex
                [0 as ::core::ffi::c_int as usize]
            && (*e).vertex[0 as ::core::ffi::c_int as usize]
                != (*(*f).edge[0 as ::core::ffi::c_int as usize]).vertex
                    [1 as ::core::ffi::c_int as usize]
        {
            c = &raw mut (**(&raw mut (*e).vertex as *mut *mut ccd_pt_vertex_t)
                .offset(0 as ::core::ffi::c_int as isize))
            .v
            .v;
        } else {
            c = &raw mut (**(&raw mut (*e).vertex as *mut *mut ccd_pt_vertex_t)
                .offset(1 as ::core::ffi::c_int as isize))
            .v
            .v;
        }
        dist = ccdVec3PointTriDist2(ccd_vec3_origin, a, b, c, &raw mut (*f).witness);
        (*f).dist = dist;
        f = ((*f).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_face_t;
    }
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtNearest(mut pt: *mut ccd_pt_t) -> *mut ccd_pt_el_t {
    if (*pt).nearest.is_null() {
        _ccdPtNearestRenew(pt);
    }
    return (*pt).nearest;
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtDumpSVT(mut pt: *mut ccd_pt_t, mut fn_0: *const ::core::ffi::c_char) {
    let mut fout: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fout = fopen(fn_0, b"a\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if fout.is_null() {
        return;
    }
    ccdPtDumpSVT2(pt, fout);
    fclose(fout);
}
#[unsafe(no_mangle)]


pub unsafe extern "C" fn ccdPtDumpSVT2(mut pt: *mut ccd_pt_t, mut fout: *mut FILE) {
    let mut v: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut a: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut b: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut c: *mut ccd_pt_vertex_t = ::core::ptr::null_mut::<ccd_pt_vertex_t>();
    let mut e: *mut ccd_pt_edge_t = ::core::ptr::null_mut::<ccd_pt_edge_t>();
    let mut f: *mut ccd_pt_face_t = ::core::ptr::null_mut::<ccd_pt_face_t>();
    let mut i: size_t = 0;
    fprintf(
        fout,
        b"-----\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        fout,
        b"Points:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as size_t;
    v = ((*pt).vertices.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    while &raw mut (*v).list != &raw mut (*pt).vertices {
        let fresh0 = i;
        i = i.wrapping_add(1);
        (*v).id = fresh0 as ::core::ffi::c_int;
        fprintf(
            fout,
            b"%lf %lf %lf\n\0" as *const u8 as *const ::core::ffi::c_char,
            ccdVec3X(&raw mut (*v).v.v) as ::core::ffi::c_double,
            ccdVec3Y(&raw mut (*v).v.v) as ::core::ffi::c_double,
            ccdVec3Z(&raw mut (*v).v.v) as ::core::ffi::c_double,
        );
        v = ((*v).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_vertex_t;
    }
    fprintf(
        fout,
        b"Edges:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    e = ((*pt).edges.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_edge_t;
    while &raw mut (*e).list != &raw mut (*pt).edges {
        fprintf(
            fout,
            b"%d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*(*e).vertex[0 as ::core::ffi::c_int as usize]).id,
            (*(*e).vertex[1 as ::core::ffi::c_int as usize]).id,
        );
        e = ((*e).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_edge_t;
    }
    fprintf(
        fout,
        b"Faces:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    f = ((*pt).faces.next as *mut ::core::ffi::c_char)
        .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_face_t;
    while &raw mut (*f).list != &raw mut (*pt).faces {
        a = (*(*f).edge[0 as ::core::ffi::c_int as usize]).vertex[0 as ::core::ffi::c_int as usize];
        b = (*(*f).edge[0 as ::core::ffi::c_int as usize]).vertex[1 as ::core::ffi::c_int as usize];
        c = (*(*f).edge[1 as ::core::ffi::c_int as usize]).vertex[0 as ::core::ffi::c_int as usize];
        if c == a || c == b {
            c = (*(*f).edge[1 as ::core::ffi::c_int as usize]).vertex
                [1 as ::core::ffi::c_int as usize];
        }
        fprintf(
            fout,
            b"%d %d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*a).id,
            (*b).id,
            (*c).id,
        );
        f = ((*f).list.next as *mut ::core::ffi::c_char)
            .offset(-(24 as ::core::ffi::c_ulong as isize)) as *mut ccd_pt_face_t;
    }
}
#[inline(always)]
unsafe extern "C" fn ccdListInit(mut l: *mut ccd_list_t) {
    (*l).next = l as *mut _ccd_list_t;
    (*l).prev = l as *mut _ccd_list_t;
}
#[inline(always)]
unsafe extern "C" fn ccdListEmpty(mut head: *const ccd_list_t) -> ::core::ffi::c_int {
    return ((*head).next == head as *mut _ccd_list_t) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ccdListAppend(mut l: *mut ccd_list_t, mut new: *mut ccd_list_t) {
    (*new).prev = (*l).prev;
    (*new).next = l as *mut _ccd_list_t;
    (*(*l).prev).next = new as *mut _ccd_list_t;
    (*l).prev = new as *mut _ccd_list_t;
}
#[inline(always)]
unsafe extern "C" fn ccdListDel(mut item: *mut ccd_list_t) {
    (*(*item).next).prev = (*item).prev;
    (*(*item).prev).next = (*item).next;
    (*item).next = item as *mut _ccd_list_t;
    (*item).prev = item as *mut _ccd_list_t;
}
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const FLT_EPSILON: ::core::ffi::c_float = __FLT_EPSILON__;
pub const __FLT_EPSILON__: ::core::ffi::c_float = 1.19209290e-7f32;
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
