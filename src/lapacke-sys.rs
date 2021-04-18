#[inline]
pub unsafe fn sbdsdc(
    layout: Layout,
    uplo: u8,
    compq: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    q: &mut f32,
    iq: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sbdsdc(
        layout.into(),
        uplo as c_char,
        compq as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        q,
        iq.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dbdsdc(
    layout: Layout,
    uplo: u8,
    compq: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    q: &mut f64,
    iq: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dbdsdc(
        layout.into(),
        uplo as c_char,
        compq as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        q,
        iq.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sbdsqr(
    layout: Layout,
    uplo: u8,
    n: i32,
    ncvt: i32,
    nru: i32,
    ncc: i32,
    d: &mut [f32],
    e: &mut [f32],
    vt: &mut [f32],
    ldvt: i32,
    u: &mut [f32],
    ldu: i32,
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sbdsqr(
        layout.into(),
        uplo as c_char,
        n,
        ncvt,
        nru,
        ncc,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr(),
        ldvt,
        u.as_mut_ptr(),
        ldu,
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dbdsqr(
    layout: Layout,
    uplo: u8,
    n: i32,
    ncvt: i32,
    nru: i32,
    ncc: i32,
    d: &mut [f64],
    e: &mut [f64],
    vt: &mut [f64],
    ldvt: i32,
    u: &mut [f64],
    ldu: i32,
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dbdsqr(
        layout.into(),
        uplo as c_char,
        n,
        ncvt,
        nru,
        ncc,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr(),
        ldvt,
        u.as_mut_ptr(),
        ldu,
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn cbdsqr(
    layout: Layout,
    uplo: u8,
    n: i32,
    ncvt: i32,
    nru: i32,
    ncc: i32,
    d: &mut [f32],
    e: &mut [f32],
    vt: &mut [c32],
    ldvt: i32,
    u: &mut [c32],
    ldu: i32,
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cbdsqr(
        layout.into(),
        uplo as c_char,
        n,
        ncvt,
        nru,
        ncc,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr() as *mut _,
        ldvt,
        u.as_mut_ptr() as *mut _,
        ldu,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zbdsqr(
    layout: Layout,
    uplo: u8,
    n: i32,
    ncvt: i32,
    nru: i32,
    ncc: i32,
    d: &mut [f64],
    e: &mut [f64],
    vt: &mut [c64],
    ldvt: i32,
    u: &mut [c64],
    ldu: i32,
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zbdsqr(
        layout.into(),
        uplo as c_char,
        n,
        ncvt,
        nru,
        ncc,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr() as *mut _,
        ldvt,
        u.as_mut_ptr() as *mut _,
        ldu,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn sbdsvdx(
    layout: Layout,
    uplo: u8,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    superb: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sbdsvdx(
        layout.into(),
        uplo as c_char,
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dbdsvdx(
    layout: Layout,
    uplo: u8,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    superb: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dbdsvdx(
        layout.into(),
        uplo as c_char,
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sdisna(job: u8, m: i32, n: i32, d: &[f32], sep: &mut [f32]) -> i32 {
    ffi::LAPACKE_sdisna(job as c_char, m, n, d.as_ptr(), sep.as_mut_ptr())
}

#[inline]
pub unsafe fn ddisna(job: u8, m: i32, n: i32, d: &[f64], sep: &mut [f64]) -> i32 {
    ffi::LAPACKE_ddisna(job as c_char, m, n, d.as_ptr(), sep.as_mut_ptr())
}

#[inline]
pub unsafe fn sgbbrd(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    ncc: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut f32,
    ldq: i32,
    pt: &mut [f32],
    ldpt: i32,
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sgbbrd(
        layout.into(),
        vect as c_char,
        m,
        n,
        ncc,
        kl,
        ku,
        ab.as_mut_ptr(),
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q,
        ldq,
        pt.as_mut_ptr(),
        ldpt,
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dgbbrd(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    ncc: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut f64,
    ldq: i32,
    pt: &mut [f64],
    ldpt: i32,
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dgbbrd(
        layout.into(),
        vect as c_char,
        m,
        n,
        ncc,
        kl,
        ku,
        ab.as_mut_ptr(),
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q,
        ldq,
        pt.as_mut_ptr(),
        ldpt,
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn cgbbrd(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    ncc: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut c32,
    ldq: i32,
    pt: &mut [c32],
    ldpt: i32,
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cgbbrd(
        layout.into(),
        vect as c_char,
        m,
        n,
        ncc,
        kl,
        ku,
        ab.as_mut_ptr() as *mut _,
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q as *mut _ as *mut _,
        ldq,
        pt.as_mut_ptr() as *mut _,
        ldpt,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zgbbrd(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    ncc: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut c64,
    ldq: i32,
    pt: &mut [c64],
    ldpt: i32,
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zgbbrd(
        layout.into(),
        vect as c_char,
        m,
        n,
        ncc,
        kl,
        ku,
        ab.as_mut_ptr() as *mut _,
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q as *mut _ as *mut _,
        ldq,
        pt.as_mut_ptr() as *mut _,
        ldpt,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn sgbcon(
    layout: Layout,
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgbcon(
        layout.into(),
        norm as c_char,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn dgbcon(
    layout: Layout,
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgbcon(
        layout.into(),
        norm as c_char,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn cgbcon(
    layout: Layout,
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgbcon(
        layout.into(),
        norm as c_char,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zgbcon(
    layout: Layout,
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgbcon(
        layout.into(),
        norm as c_char,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn sgbequ(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgbequ(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn dgbequ(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgbequ(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn cgbequ(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgbequ(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn zgbequ(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgbequ(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn sgbequb(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgbequb(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn dgbequb(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgbequb(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn cgbequb(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgbequb(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn zgbequb(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgbequb(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn sgbrfs(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgbrfs(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbrfs(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgbrfs(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbrfs(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbrfs(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbrfs(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbrfs(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbrfsx(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgbrfsx(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbrfsx(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgbrfsx(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbrfsx(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbrfsx(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbrfsx(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbrfsx(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbsv(
    layout: Layout,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgbsv(
        layout.into(),
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgbsv(
    layout: Layout,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgbsv(
        layout.into(),
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgbsv(
    layout: Layout,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgbsv(
        layout.into(),
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgbsv(
    layout: Layout,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgbsv(
        layout.into(),
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sgbsvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    rpivot: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgbsvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        rpivot.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbsvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    rpivot: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgbsvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        rpivot.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbsvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    rpivot: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbsvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        rpivot.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbsvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    rpivot: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbsvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        rpivot.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbsvxx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgbsvxx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbsvxx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgbsvxx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbsvxx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbsvxx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbsvxx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbsvxx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbtrf(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f32],
    ldab: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgbtrf(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_mut_ptr(),
        ldab,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbtrf(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f64],
    ldab: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgbtrf(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_mut_ptr(),
        ldab,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbtrf(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c32],
    ldab: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgbtrf(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_mut_ptr() as *mut _,
        ldab,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbtrf(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c64],
    ldab: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgbtrf(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_mut_ptr() as *mut _,
        ldab,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbtrs(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgbtrs(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgbtrs(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgbtrs(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgbtrs(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgbtrs(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgbtrs(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgbtrs(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sgebak(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f32],
    m: i32,
    v: &mut [f32],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_sgebak(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        scale.as_ptr(),
        m,
        v.as_mut_ptr(),
        ldv,
    )
}

#[inline]
pub unsafe fn dgebak(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f64],
    m: i32,
    v: &mut [f64],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_dgebak(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        scale.as_ptr(),
        m,
        v.as_mut_ptr(),
        ldv,
    )
}

#[inline]
pub unsafe fn cgebak(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f32],
    m: i32,
    v: &mut [c32],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_cgebak(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        scale.as_ptr(),
        m,
        v.as_mut_ptr() as *mut _,
        ldv,
    )
}

#[inline]
pub unsafe fn zgebak(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f64],
    m: i32,
    v: &mut [c64],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_zgebak(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        scale.as_ptr(),
        m,
        v.as_mut_ptr() as *mut _,
        ldv,
    )
}

#[inline]
pub unsafe fn sgebal(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgebal(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgebal(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgebal(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgebal(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgebal(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgebal(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgebal(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgebrd(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tauq: &mut [f32],
    taup: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgebrd(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr(),
        taup.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgebrd(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [f64],
    taup: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgebrd(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr(),
        taup.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgebrd(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tauq: &mut [c32],
    taup: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgebrd(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr() as *mut _,
        taup.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgebrd(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [c64],
    taup: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgebrd(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr() as *mut _,
        taup.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgecon(
    layout: Layout,
    norm: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgecon(
        layout.into(),
        norm as c_char,
        n,
        a.as_ptr(),
        lda,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn dgecon(
    layout: Layout,
    norm: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgecon(
        layout.into(),
        norm as c_char,
        n,
        a.as_ptr(),
        lda,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn cgecon(
    layout: Layout,
    norm: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgecon(
        layout.into(),
        norm as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zgecon(
    layout: Layout,
    norm: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgecon(
        layout.into(),
        norm as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn sgeequ(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgeequ(
        layout.into(),
        m,
        n,
        a.as_ptr(),
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn dgeequ(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgeequ(
        layout.into(),
        m,
        n,
        a.as_ptr(),
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn cgeequ(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgeequ(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn zgeequ(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgeequ(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn sgeequb(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgeequb(
        layout.into(),
        m,
        n,
        a.as_ptr(),
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn dgeequb(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgeequb(
        layout.into(),
        m,
        n,
        a.as_ptr(),
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn cgeequb(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgeequb(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn zgeequb(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgeequb(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn sgees(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select2F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vs: &mut [f32],
    ldvs: i32,
) -> i32 {
    ffi::LAPACKE_sgees(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        n,
        a.as_mut_ptr(),
        lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        ldvs,
    )
}

#[inline]
pub unsafe fn dgees(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select2F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vs: &mut [f64],
    ldvs: i32,
) -> i32 {
    ffi::LAPACKE_dgees(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        n,
        a.as_mut_ptr(),
        lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        ldvs,
    )
}

#[inline]
pub unsafe fn cgees(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select1C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c32],
    vs: &mut [c32],
    ldvs: i32,
) -> i32 {
    ffi::LAPACKE_cgees(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        ldvs,
    )
}

#[inline]
pub unsafe fn zgees(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select1C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c64],
    vs: &mut [c64],
    ldvs: i32,
) -> i32 {
    ffi::LAPACKE_zgees(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        ldvs,
    )
}

#[inline]
pub unsafe fn sgeesx(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select2F32,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vs: &mut [f32],
    ldvs: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeesx(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgeesx(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select2F64,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vs: &mut [f64],
    ldvs: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeesx(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgeesx(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select1C32,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c32],
    vs: &mut [c32],
    ldvs: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgeesx(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgeesx(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select1C64,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c64],
    vs: &mut [c64],
    ldvs: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgeesx(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgeev(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_sgeev(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
    )
}

#[inline]
pub unsafe fn dgeev(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_dgeev(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
    )
}

#[inline]
pub unsafe fn cgeev(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_cgeev(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
    )
}

#[inline]
pub unsafe fn zgeev(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_zgeev(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
    )
}

#[inline]
pub unsafe fn sgeevx(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
    abnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeevx(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgeevx(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
    abnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeevx(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgeevx(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
    abnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgeevx(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgeevx(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
    abnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgeevx(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgehrd(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgehrd(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgehrd(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgehrd(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgehrd(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgehrd(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgehrd(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgehrd(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgejsv(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sva: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    stat: &mut [f32],
    istat: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgejsv(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        jobr as c_char,
        jobt as c_char,
        jobp as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        stat.as_mut_ptr(),
        istat.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgejsv(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sva: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    stat: &mut [f64],
    istat: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgejsv(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        jobr as c_char,
        jobt as c_char,
        jobp as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        stat.as_mut_ptr(),
        istat.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgejsv(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sva: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    stat: &mut [f32],
    istat: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgejsv(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        jobr as c_char,
        jobt as c_char,
        jobp as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        stat.as_mut_ptr(),
        istat.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgejsv(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sva: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    stat: &mut [f64],
    istat: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgejsv(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        jobr as c_char,
        jobt as c_char,
        jobp as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        stat.as_mut_ptr(),
        istat.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgelq2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgelq2(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dgelq2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgelq2(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn cgelq2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgelq2(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgelq2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgelq2(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgelqf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgelqf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dgelqf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgelqf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn cgelqf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgelqf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgelqf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgelqf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgels(
    layout: Layout,
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgels(
        layout.into(),
        trans as c_char,
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgels(
    layout: Layout,
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgels(
        layout.into(),
        trans as c_char,
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgels(
    layout: Layout,
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgels(
        layout.into(),
        trans as c_char,
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgels(
    layout: Layout,
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgels(
        layout.into(),
        trans as c_char,
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sgelsd(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_sgelsd(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn dgelsd(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_dgelsd(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn cgelsd(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_cgelsd(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn zgelsd(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_zgelsd(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn sgelss(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_sgelss(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn dgelss(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_dgelss(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn cgelss(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_cgelss(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn zgelss(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_zgelss(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn sgelsy(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f32,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_sgelsy(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        jpvt.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn dgelsy(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f64,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_dgelsy(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        jpvt.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn cgelsy(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f32,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_cgelsy(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        jpvt.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn zgelsy(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f64,
    rank: &mut i32,
) -> i32 {
    ffi::LAPACKE_zgelsy(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        jpvt.as_mut_ptr(),
        rcond,
        rank,
    )
}

#[inline]
pub unsafe fn sgeqlf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeqlf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dgeqlf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeqlf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn cgeqlf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgeqlf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgeqlf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgeqlf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgeqp3(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeqp3(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgeqp3(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeqp3(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgeqp3(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgeqp3(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgeqp3(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgeqp3(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgeqpf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeqpf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgeqpf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeqpf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgeqpf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgeqpf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgeqpf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgeqpf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgeqr2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeqr2(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dgeqr2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeqr2(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn cgeqr2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgeqr2(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgeqr2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgeqr2(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgeqrf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeqrf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dgeqrf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeqrf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn cgeqrf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgeqrf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgeqrf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgeqrf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgeqrfp(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeqrfp(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dgeqrfp(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeqrfp(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn cgeqrfp(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgeqrfp(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgeqrfp(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgeqrfp(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgerfs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgerfs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgerfs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgerfs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgerfs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgerfs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgerfs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgerfs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgerfsx(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgerfsx(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgerfsx(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgerfsx(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgerfsx(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgerfsx(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgerfsx(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgerfsx(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgerqf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgerqf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dgerqf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgerqf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn cgerqf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgerqf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgerqf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgerqf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgesdd(
    layout: Layout,
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
) -> i32 {
    ffi::LAPACKE_sgesdd(
        layout.into(),
        jobz as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
    )
}

#[inline]
pub unsafe fn dgesdd(
    layout: Layout,
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
) -> i32 {
    ffi::LAPACKE_dgesdd(
        layout.into(),
        jobz as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
    )
}

#[inline]
pub unsafe fn cgesdd(
    layout: Layout,
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
) -> i32 {
    ffi::LAPACKE_cgesdd(
        layout.into(),
        jobz as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
    )
}

#[inline]
pub unsafe fn zgesdd(
    layout: Layout,
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
) -> i32 {
    ffi::LAPACKE_zgesdd(
        layout.into(),
        jobz as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
    )
}

#[inline]
pub unsafe fn sgesv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgesv(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgesv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgesv(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgesv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgesv(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgesv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgesv(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn dsgesv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    iter: &mut i32,
) -> i32 {
    ffi::LAPACKE_dsgesv(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        iter,
    )
}

#[inline]
pub unsafe fn zcgesv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    iter: &mut i32,
) -> i32 {
    ffi::LAPACKE_zcgesv(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        iter,
    )
}

#[inline]
pub unsafe fn sgesvd(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    superb: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgesvd(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesvd(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    superb: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgesvd(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesvd(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
    superb: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgesvd(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvd(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
    superb: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgesvd(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgesvdx(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    superb: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgesvdx(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        range as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesvdx(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    superb: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgesvdx(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        range as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesvdx(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
    superb: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgesvdx(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        range as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvdx(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
    superb: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgesvdx(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        range as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        superb.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgesvj(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sva: &mut [f32],
    mv: i32,
    v: &mut [f32],
    ldv: i32,
    stat: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgesvj(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        sva.as_mut_ptr(),
        mv,
        v.as_mut_ptr(),
        ldv,
        stat.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesvj(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sva: &mut [f64],
    mv: i32,
    v: &mut [f64],
    ldv: i32,
    stat: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgesvj(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        sva.as_mut_ptr(),
        mv,
        v.as_mut_ptr(),
        ldv,
        stat.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesvj(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sva: &mut [f32],
    mv: i32,
    v: &mut [c32],
    ldv: i32,
    stat: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgesvj(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sva.as_mut_ptr(),
        mv,
        v.as_mut_ptr() as *mut _,
        ldv,
        stat.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvj(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sva: &mut [f64],
    mv: i32,
    v: &mut [c64],
    ldv: i32,
    stat: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgesvj(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sva.as_mut_ptr(),
        mv,
        v.as_mut_ptr() as *mut _,
        ldv,
        stat.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgesvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    rpivot: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgesvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        rpivot.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    rpivot: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgesvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        rpivot.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    rpivot: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgesvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        rpivot.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    rpivot: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgesvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        rpivot.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgesvxx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgesvxx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesvxx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgesvxx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesvxx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgesvxx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvxx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgesvxx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgetf2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgetf2(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn dgetf2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgetf2(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn cgetf2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgetf2(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgetf2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgetf2(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgetrf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgetrf(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn dgetrf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgetrf(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn cgetrf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgetrf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgetrf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgetrf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgetrf2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgetrf2(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn dgetrf2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgetrf2(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn cgetrf2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgetrf2(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgetrf2(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgetrf2(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgetri(layout: Layout, n: i32, a: &mut [f32], lda: i32, ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_sgetri(layout.into(), n, a.as_mut_ptr(), lda, ipiv.as_ptr())
}

#[inline]
pub unsafe fn dgetri(layout: Layout, n: i32, a: &mut [f64], lda: i32, ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_dgetri(layout.into(), n, a.as_mut_ptr(), lda, ipiv.as_ptr())
}

#[inline]
pub unsafe fn cgetri(layout: Layout, n: i32, a: &mut [c32], lda: i32, ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_cgetri(
        layout.into(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn zgetri(layout: Layout, n: i32, a: &mut [c64], lda: i32, ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_zgetri(
        layout.into(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn sgetrs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgetrs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgetrs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgetrs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgetrs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgetrs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgetrs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgetrs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sggbak(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f32],
    rscale: &[f32],
    m: i32,
    v: &mut [f32],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_sggbak(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        m,
        v.as_mut_ptr(),
        ldv,
    )
}

#[inline]
pub unsafe fn dggbak(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f64],
    rscale: &[f64],
    m: i32,
    v: &mut [f64],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_dggbak(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        m,
        v.as_mut_ptr(),
        ldv,
    )
}

#[inline]
pub unsafe fn cggbak(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f32],
    rscale: &[f32],
    m: i32,
    v: &mut [c32],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_cggbak(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        m,
        v.as_mut_ptr() as *mut _,
        ldv,
    )
}

#[inline]
pub unsafe fn zggbak(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f64],
    rscale: &[f64],
    m: i32,
    v: &mut [c64],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_zggbak(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        m,
        v.as_mut_ptr() as *mut _,
        ldv,
    )
}

#[inline]
pub unsafe fn sggbal(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sggbal(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggbal(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dggbal(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggbal(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cggbal(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggbal(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zggbal(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgges(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
) -> i32 {
    ffi::LAPACKE_sgges(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
    )
}

#[inline]
pub unsafe fn dgges(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
) -> i32 {
    ffi::LAPACKE_dgges(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
    )
}

#[inline]
pub unsafe fn cgges(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
) -> i32 {
    ffi::LAPACKE_cgges(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
    )
}

#[inline]
pub unsafe fn zgges(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
) -> i32 {
    ffi::LAPACKE_zgges(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
    )
}

#[inline]
pub unsafe fn sgges3(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
) -> i32 {
    ffi::LAPACKE_sgges3(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
    )
}

#[inline]
pub unsafe fn dgges3(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
) -> i32 {
    ffi::LAPACKE_dgges3(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
    )
}

#[inline]
pub unsafe fn cgges3(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
) -> i32 {
    ffi::LAPACKE_cgges3(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
    )
}

#[inline]
pub unsafe fn zgges3(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
) -> i32 {
    ffi::LAPACKE_zgges3(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
    )
}

#[inline]
pub unsafe fn sggesx(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sggesx(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggesx(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dggesx(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggesx(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cggesx(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggesx(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zggesx(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggev(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_sggev(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
    )
}

#[inline]
pub unsafe fn dggev(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_dggev(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
    )
}

#[inline]
pub unsafe fn cggev(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_cggev(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
    )
}

#[inline]
pub unsafe fn zggev(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_zggev(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
    )
}

#[inline]
pub unsafe fn sggev3(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_sggev3(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
    )
}

#[inline]
pub unsafe fn dggev3(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_dggev3(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
    )
}

#[inline]
pub unsafe fn cggev3(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_cggev3(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
    )
}

#[inline]
pub unsafe fn zggev3(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
) -> i32 {
    ffi::LAPACKE_zggev3(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
    )
}

#[inline]
pub unsafe fn sggevx(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    abnrm: &mut f32,
    bbnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sggevx(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggevx(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    abnrm: &mut f64,
    bbnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dggevx(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggevx(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    abnrm: &mut f32,
    bbnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cggevx(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggevx(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    abnrm: &mut f64,
    bbnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zggevx(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggglm(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    d: &mut [f32],
    x: &mut [f32],
    y: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sggglm(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        y.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggglm(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    d: &mut [f64],
    x: &mut [f64],
    y: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dggglm(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        y.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggglm(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    d: &mut [c32],
    x: &mut [c32],
    y: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cggglm(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        y.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zggglm(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    d: &mut [c64],
    x: &mut [c64],
    y: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zggglm(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        y.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgghrd(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sgghrd(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dgghrd(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dgghrd(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn cgghrd(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_cgghrd(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zgghrd(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zgghrd(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn sgghd3(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sgghd3(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dgghd3(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dgghd3(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn cgghd3(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_cgghd3(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zgghd3(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zgghd3(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn sgglse(
    layout: Layout,
    m: i32,
    n: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    c: &mut [f32],
    d: &mut [f32],
    x: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgglse(
        layout.into(),
        m,
        n,
        p,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        c.as_mut_ptr(),
        d.as_mut_ptr(),
        x.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgglse(
    layout: Layout,
    m: i32,
    n: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    c: &mut [f64],
    d: &mut [f64],
    x: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgglse(
        layout.into(),
        m,
        n,
        p,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        c.as_mut_ptr(),
        d.as_mut_ptr(),
        x.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgglse(
    layout: Layout,
    m: i32,
    n: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    c: &mut [c32],
    d: &mut [c32],
    x: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgglse(
        layout.into(),
        m,
        n,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        c.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgglse(
    layout: Layout,
    m: i32,
    n: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    c: &mut [c64],
    d: &mut [c64],
    x: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgglse(
        layout.into(),
        m,
        n,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        c.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sggqrf(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    taua: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    taub: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sggqrf(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr(),
        lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        taub.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggqrf(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    taua: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    taub: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dggqrf(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr(),
        lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        taub.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggqrf(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    taua: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    taub: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cggqrf(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
        taub.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zggqrf(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    taua: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    taub: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zggqrf(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
        taub.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sggrqf(
    layout: Layout,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    taua: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    taub: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sggrqf(
        layout.into(),
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        taub.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggrqf(
    layout: Layout,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    taua: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    taub: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dggrqf(
        layout.into(),
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        taub.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggrqf(
    layout: Layout,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    taua: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    taub: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cggrqf(
        layout.into(),
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
        taub.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zggrqf(
    layout: Layout,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    taua: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    taub: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zggrqf(
        layout.into(),
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
        taub.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sggsvd(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sggsvd(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggsvd(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dggsvd(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggsvd(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cggsvd(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggsvd(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zggsvd(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggsvd3(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sggsvd3(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggsvd3(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dggsvd3(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggsvd3(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cggsvd3(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggsvd3(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zggsvd3(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggsvp(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_sggsvp(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
    )
}

#[inline]
pub unsafe fn dggsvp(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_dggsvp(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
    )
}

#[inline]
pub unsafe fn cggsvp(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_cggsvp(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
    )
}

#[inline]
pub unsafe fn zggsvp(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_zggsvp(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
    )
}

#[inline]
pub unsafe fn sggsvp3(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_sggsvp3(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
    )
}

#[inline]
pub unsafe fn dggsvp3(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_dggsvp3(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
    )
}

#[inline]
pub unsafe fn cggsvp3(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_cggsvp3(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
    )
}

#[inline]
pub unsafe fn zggsvp3(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_zggsvp3(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
    )
}

#[inline]
pub unsafe fn sgtcon(
    norm: u8,
    n: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgtcon(
        norm as c_char,
        n,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn dgtcon(
    norm: u8,
    n: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgtcon(
        norm as c_char,
        n,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn cgtcon(
    norm: u8,
    n: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgtcon(
        norm as c_char,
        n,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zgtcon(
    norm: u8,
    n: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgtcon(
        norm as c_char,
        n,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn sgtrfs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    dlf: &[f32],
    df: &[f32],
    duf: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgtrfs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_ptr(),
        df.as_ptr(),
        duf.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgtrfs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    dlf: &[f64],
    df: &[f64],
    duf: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgtrfs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_ptr(),
        df.as_ptr(),
        duf.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgtrfs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    dlf: &[c32],
    df: &[c32],
    duf: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgtrfs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_ptr() as *const _,
        df.as_ptr() as *const _,
        duf.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgtrfs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    dlf: &[c64],
    df: &[c64],
    duf: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgtrfs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_ptr() as *const _,
        df.as_ptr() as *const _,
        duf.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgtsv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    dl: &mut [f32],
    d: &mut [f32],
    du: &mut [f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgtsv(
        layout.into(),
        n,
        nrhs,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgtsv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    dl: &mut [f64],
    d: &mut [f64],
    du: &mut [f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgtsv(
        layout.into(),
        n,
        nrhs,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgtsv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    dl: &mut [c32],
    d: &mut [c32],
    du: &mut [c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgtsv(
        layout.into(),
        n,
        nrhs,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgtsv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    dl: &mut [c64],
    d: &mut [c64],
    du: &mut [c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgtsv(
        layout.into(),
        n,
        nrhs,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sgtsvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    dlf: &mut [f32],
    df: &mut [f32],
    duf: &mut [f32],
    du2: &mut [f32],
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgtsvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_mut_ptr(),
        df.as_mut_ptr(),
        duf.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgtsvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    dlf: &mut [f64],
    df: &mut [f64],
    duf: &mut [f64],
    du2: &mut [f64],
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgtsvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_mut_ptr(),
        df.as_mut_ptr(),
        duf.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgtsvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    dlf: &mut [c32],
    df: &mut [c32],
    duf: &mut [c32],
    du2: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgtsvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_mut_ptr() as *mut _,
        df.as_mut_ptr() as *mut _,
        duf.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgtsvx(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    dlf: &mut [c64],
    df: &mut [c64],
    duf: &mut [c64],
    du2: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgtsvx(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_mut_ptr() as *mut _,
        df.as_mut_ptr() as *mut _,
        duf.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgttrf(
    n: i32,
    dl: &mut [f32],
    d: &mut [f32],
    du: &mut [f32],
    du2: &mut [f32],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgttrf(
        n,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgttrf(
    n: i32,
    dl: &mut [f64],
    d: &mut [f64],
    du: &mut [f64],
    du2: &mut [f64],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgttrf(
        n,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgttrf(
    n: i32,
    dl: &mut [c32],
    d: &mut [c32],
    du: &mut [c32],
    du2: &mut [c32],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgttrf(
        n,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgttrf(
    n: i32,
    dl: &mut [c64],
    d: &mut [c64],
    du: &mut [c64],
    du2: &mut [c64],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgttrf(
        n,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgttrs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgttrs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgttrs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgttrs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgttrs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgttrs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgttrs(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgttrs(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn chbev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chbev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhbev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhbev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chbevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chbevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhbevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhbevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chbevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    q: &mut c32,
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chbevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        q as *mut _ as *mut _,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhbevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    q: &mut c64,
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhbevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        q as *mut _ as *mut _,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chbgst(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &[c32],
    ldbb: i32,
    x: &mut [c32],
    ldx: i32,
) -> i32 {
    ffi::LAPACKE_chbgst(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_ptr() as *const _,
        ldbb,
        x.as_mut_ptr() as *mut _,
        ldx,
    )
}

#[inline]
pub unsafe fn zhbgst(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &[c64],
    ldbb: i32,
    x: &mut [c64],
    ldx: i32,
) -> i32 {
    ffi::LAPACKE_zhbgst(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_ptr() as *const _,
        ldbb,
        x.as_mut_ptr() as *mut _,
        ldx,
    )
}

#[inline]
pub unsafe fn chbgv(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chbgv(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhbgv(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhbgv(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chbgvd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chbgvd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhbgvd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhbgvd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chbgvx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    q: &mut c32,
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chbgvx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        q as *mut _ as *mut _,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhbgvx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    q: &mut c64,
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhbgvx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        q as *mut _ as *mut _,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chbtrd(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut c32,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_chbtrd(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q as *mut _ as *mut _,
        ldq,
    )
}

#[inline]
pub unsafe fn zhbtrd(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut c64,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_zhbtrd(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q as *mut _ as *mut _,
        ldq,
    )
}

#[inline]
pub unsafe fn checon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_checon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zhecon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zhecon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn cheequb(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cheequb(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zheequb(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zheequb(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cheev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cheev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zheev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zheev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cheevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cheevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zheevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zheevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cheevr(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cheevr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zheevr(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zheevr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cheevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cheevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zheevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zheevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chegst(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chegst(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_ptr() as *const _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhegst(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhegst(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_ptr() as *const _,
        ldb,
    )
}

#[inline]
pub unsafe fn chegv(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    w: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chegv(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhegv(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    w: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhegv(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chegvd(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    w: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chegvd(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhegvd(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    w: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhegvd(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chegvx(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chegvx(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhegvx(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhegvx(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cherfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cherfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zherfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zherfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cherfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cherfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zherfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zherfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chesv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chesv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhesv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhesv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn chesvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chesvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhesvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhesvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chesvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chesvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhesvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhesvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chetrd(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chetrd(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhetrd(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhetrd(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn chetrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chetrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhetrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhetrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chetri(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
) -> i32 {
    ffi::LAPACKE_chetri(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn zhetri(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
) -> i32 {
    ffi::LAPACKE_zhetri(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn chetrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chetrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhetrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhetrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn chfrk(
    layout: Layout,
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: f32,
    a: &[c32],
    lda: i32,
    beta: f32,
    c: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chfrk(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        trans as c_char,
        n,
        k,
        alpha,
        a.as_ptr() as *const _,
        lda,
        beta,
        c.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhfrk(
    layout: Layout,
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: f64,
    a: &[c64],
    lda: i32,
    beta: f64,
    c: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhfrk(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        trans as c_char,
        n,
        k,
        alpha,
        a.as_ptr() as *const _,
        lda,
        beta,
        c.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn shgeqz(
    layout: Layout,
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f32],
    ldh: i32,
    t: &mut [f32],
    ldt: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_shgeqz(
        layout.into(),
        job as c_char,
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr(),
        ldh,
        t.as_mut_ptr(),
        ldt,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dhgeqz(
    layout: Layout,
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f64],
    ldh: i32,
    t: &mut [f64],
    ldt: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dhgeqz(
        layout.into(),
        job as c_char,
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr(),
        ldh,
        t.as_mut_ptr(),
        ldt,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn chgeqz(
    layout: Layout,
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c32],
    ldh: i32,
    t: &mut [c32],
    ldt: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chgeqz(
        layout.into(),
        job as c_char,
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr() as *mut _,
        ldh,
        t.as_mut_ptr() as *mut _,
        ldt,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhgeqz(
    layout: Layout,
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c64],
    ldh: i32,
    t: &mut [c64],
    ldt: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhgeqz(
        layout.into(),
        job as c_char,
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr() as *mut _,
        ldh,
        t.as_mut_ptr() as *mut _,
        ldt,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chpcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_chpcon(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zhpcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zhpcon(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn chpev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chpev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhpev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhpev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chpevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chpevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhpevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhpevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chpevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chpevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhpevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhpevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chpgst(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &[c32],
) -> i32 {
    ffi::LAPACKE_chpgst(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zhpgst(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &[c64],
) -> i32 {
    ffi::LAPACKE_zhpgst(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn chpgv(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chpgv(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhpgv(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhpgv(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chpgvd(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chpgvd(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhpgvd(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhpgvd(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn chpgvx(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chpgvx(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhpgvx(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhpgvx(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chpsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chpsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhpsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhpsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn chpsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chpsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhpsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhpsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chptrd(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chptrd(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhptrd(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhptrd(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn chptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [c32], ipiv: &mut [i32]) -> i32 {
    ffi::LAPACKE_chptrf(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [c64], ipiv: &mut [i32]) -> i32 {
    ffi::LAPACKE_zhptrf(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chptri(layout: Layout, uplo: u8, n: i32, ap: &mut [c32], ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_chptri(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn zhptri(layout: Layout, uplo: u8, n: i32, ap: &mut [c64], ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_zhptri(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn chptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn shsein(
    layout: Layout,
    job: u8,
    eigsrc: u8,
    initv: u8,
    select: &mut [i32],
    n: i32,
    h: &[f32],
    ldh: i32,
    wr: &mut [f32],
    wi: &[f32],
    vl: &mut f32,
    ldvl: i32,
    vr: &mut f32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    ifaill: &mut [i32],
    ifailr: &mut [i32],
) -> i32 {
    ffi::LAPACKE_shsein(
        layout.into(),
        job as c_char,
        eigsrc as c_char,
        initv as c_char,
        select.as_mut_ptr(),
        n,
        h.as_ptr(),
        ldh,
        wr.as_mut_ptr(),
        wi.as_ptr(),
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dhsein(
    layout: Layout,
    job: u8,
    eigsrc: u8,
    initv: u8,
    select: &mut [i32],
    n: i32,
    h: &[f64],
    ldh: i32,
    wr: &mut [f64],
    wi: &[f64],
    vl: &mut f64,
    ldvl: i32,
    vr: &mut f64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    ifaill: &mut [i32],
    ifailr: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dhsein(
        layout.into(),
        job as c_char,
        eigsrc as c_char,
        initv as c_char,
        select.as_mut_ptr(),
        n,
        h.as_ptr(),
        ldh,
        wr.as_mut_ptr(),
        wi.as_ptr(),
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chsein(
    layout: Layout,
    job: u8,
    eigsrc: u8,
    initv: u8,
    select: &[i32],
    n: i32,
    h: &[c32],
    ldh: i32,
    w: &mut [c32],
    vl: &mut c32,
    ldvl: i32,
    vr: &mut c32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    ifaill: &mut [i32],
    ifailr: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chsein(
        layout.into(),
        job as c_char,
        eigsrc as c_char,
        initv as c_char,
        select.as_ptr(),
        n,
        h.as_ptr() as *const _,
        ldh,
        w.as_mut_ptr() as *mut _,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhsein(
    layout: Layout,
    job: u8,
    eigsrc: u8,
    initv: u8,
    select: &[i32],
    n: i32,
    h: &[c64],
    ldh: i32,
    w: &mut [c64],
    vl: &mut c64,
    ldvl: i32,
    vr: &mut c64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    ifaill: &mut [i32],
    ifailr: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhsein(
        layout.into(),
        job as c_char,
        eigsrc as c_char,
        initv as c_char,
        select.as_ptr(),
        n,
        h.as_ptr() as *const _,
        ldh,
        w.as_mut_ptr() as *mut _,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn shseqr(
    layout: Layout,
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f32],
    ldh: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_shseqr(
        layout.into(),
        job as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr(),
        ldh,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dhseqr(
    layout: Layout,
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f64],
    ldh: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dhseqr(
        layout.into(),
        job as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr(),
        ldh,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn chseqr(
    layout: Layout,
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c32],
    ldh: i32,
    w: &mut [c32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_chseqr(
        layout.into(),
        job as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr() as *mut _,
        ldh,
        w.as_mut_ptr() as *mut _,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zhseqr(
    layout: Layout,
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c64],
    ldh: i32,
    w: &mut [c64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zhseqr(
        layout.into(),
        job as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr() as *mut _,
        ldh,
        w.as_mut_ptr() as *mut _,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn clacgv(n: i32, x: &mut [c32], incx: i32) -> i32 {
    ffi::LAPACKE_clacgv(n, x.as_mut_ptr() as *mut _, incx)
}

#[inline]
pub unsafe fn zlacgv(n: i32, x: &mut [c64], incx: i32) -> i32 {
    ffi::LAPACKE_zlacgv(n, x.as_mut_ptr() as *mut _, incx)
}

#[inline]
pub unsafe fn slacn2(
    n: i32,
    v: &mut [f32],
    x: &mut [f32],
    isgn: &mut [i32],
    est: &mut [f32],
    kase: &mut i32,
    isave: &mut [i32],
) -> i32 {
    ffi::LAPACKE_slacn2(
        n,
        v.as_mut_ptr(),
        x.as_mut_ptr(),
        isgn.as_mut_ptr(),
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlacn2(
    n: i32,
    v: &mut [f64],
    x: &mut [f64],
    isgn: &mut [i32],
    est: &mut [f64],
    kase: &mut i32,
    isave: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dlacn2(
        n,
        v.as_mut_ptr(),
        x.as_mut_ptr(),
        isgn.as_mut_ptr(),
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clacn2(
    n: i32,
    v: &mut [c32],
    x: &mut [c32],
    est: &mut [f32],
    kase: &mut i32,
    isave: &mut [i32],
) -> i32 {
    ffi::LAPACKE_clacn2(
        n,
        v.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlacn2(
    n: i32,
    v: &mut [c64],
    x: &mut [c64],
    est: &mut [f64],
    kase: &mut i32,
    isave: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zlacn2(
        n,
        v.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slacpy(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_slacpy(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dlacpy(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dlacpy(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn clacpy(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_clacpy(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zlacpy(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zlacpy(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn clacp2(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_clacp2(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zlacp2(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zlacp2(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zlag2c(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    sa: &mut [c32],
    ldsa: i32,
) -> i32 {
    ffi::LAPACKE_zlag2c(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        sa.as_mut_ptr() as *mut _,
        ldsa,
    )
}

#[inline]
pub unsafe fn slag2d(
    layout: Layout,
    m: i32,
    n: i32,
    sa: &[f32],
    ldsa: i32,
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_slag2d(layout.into(), m, n, sa.as_ptr(), ldsa, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dlag2s(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    sa: &mut [f32],
    ldsa: i32,
) -> i32 {
    ffi::LAPACKE_dlag2s(layout.into(), m, n, a.as_ptr(), lda, sa.as_mut_ptr(), ldsa)
}

#[inline]
pub unsafe fn clag2z(
    layout: Layout,
    m: i32,
    n: i32,
    sa: &[c32],
    ldsa: i32,
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_clag2z(
        layout.into(),
        m,
        n,
        sa.as_ptr() as *const _,
        ldsa,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn slagge(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f32],
    a: &mut [f32],
    lda: i32,
    iseed: &mut [i32],
) -> i32 {
    ffi::LAPACKE_slagge(
        layout.into(),
        m,
        n,
        kl,
        ku,
        d.as_ptr(),
        a.as_mut_ptr(),
        lda,
        iseed.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlagge(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f64],
    a: &mut [f64],
    lda: i32,
    iseed: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dlagge(
        layout.into(),
        m,
        n,
        kl,
        ku,
        d.as_ptr(),
        a.as_mut_ptr(),
        lda,
        iseed.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clagge(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f32],
    a: &mut [c32],
    lda: i32,
    iseed: &mut [i32],
) -> i32 {
    ffi::LAPACKE_clagge(
        layout.into(),
        m,
        n,
        kl,
        ku,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        lda,
        iseed.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlagge(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f64],
    a: &mut [c64],
    lda: i32,
    iseed: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zlagge(
        layout.into(),
        m,
        n,
        kl,
        ku,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        lda,
        iseed.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slamch(cmach: u8) -> f32 {
    ffi::LAPACKE_slamch(cmach as c_char)
}

#[inline]
pub unsafe fn dlamch(cmach: u8) -> f64 {
    ffi::LAPACKE_dlamch(cmach as c_char)
}

#[inline]
pub unsafe fn slange(layout: Layout, norm: u8, m: i32, n: i32, a: &[f32], lda: i32) -> f32 {
    ffi::LAPACKE_slange(layout.into(), norm as c_char, m, n, a.as_ptr(), lda)
}

#[inline]
pub unsafe fn dlange(layout: Layout, norm: u8, m: i32, n: i32, a: &[f64], lda: i32) -> f64 {
    ffi::LAPACKE_dlange(layout.into(), norm as c_char, m, n, a.as_ptr(), lda)
}

#[inline]
pub unsafe fn clange(layout: Layout, norm: u8, m: i32, n: i32, a: &[c32], lda: i32) -> f32 {
    ffi::LAPACKE_clange(
        layout.into(),
        norm as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
    )
}

#[inline]
pub unsafe fn zlange(layout: Layout, norm: u8, m: i32, n: i32, a: &[c64], lda: i32) -> f64 {
    ffi::LAPACKE_zlange(
        layout.into(),
        norm as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
    )
}

#[inline]
pub unsafe fn clanhe(layout: Layout, norm: u8, uplo: u8, n: i32, a: &[c32], lda: i32) -> f32 {
    ffi::LAPACKE_clanhe(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
    )
}

#[inline]
pub unsafe fn zlanhe(layout: Layout, norm: u8, uplo: u8, n: i32, a: &[c64], lda: i32) -> f64 {
    ffi::LAPACKE_zlanhe(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
    )
}

#[inline]
pub unsafe fn slansy(layout: Layout, norm: u8, uplo: u8, n: i32, a: &[f32], lda: i32) -> f32 {
    ffi::LAPACKE_slansy(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dlansy(layout: Layout, norm: u8, uplo: u8, n: i32, a: &[f64], lda: i32) -> f64 {
    ffi::LAPACKE_dlansy(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn clansy(layout: Layout, norm: u8, uplo: u8, n: i32, a: &[c32], lda: i32) -> f32 {
    ffi::LAPACKE_clansy(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
    )
}

#[inline]
pub unsafe fn zlansy(layout: Layout, norm: u8, uplo: u8, n: i32, a: &[c64], lda: i32) -> f64 {
    ffi::LAPACKE_zlansy(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
    )
}

#[inline]
pub unsafe fn slantr(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
) -> f32 {
    ffi::LAPACKE_slantr(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dlantr(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
) -> f64 {
    ffi::LAPACKE_dlantr(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn clantr(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
) -> f32 {
    ffi::LAPACKE_clantr(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
    )
}

#[inline]
pub unsafe fn zlantr(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
) -> f64 {
    ffi::LAPACKE_zlantr(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
    )
}

#[inline]
pub unsafe fn slarfb(
    layout: Layout,
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[f32],
    ldv: i32,
    t: &[f32],
    ldt: i32,
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_slarfb(
        layout.into(),
        side as c_char,
        trans as c_char,
        direct as c_char,
        storev as c_char,
        m,
        n,
        k,
        v.as_ptr(),
        ldv,
        t.as_ptr(),
        ldt,
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dlarfb(
    layout: Layout,
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[f64],
    ldv: i32,
    t: &[f64],
    ldt: i32,
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dlarfb(
        layout.into(),
        side as c_char,
        trans as c_char,
        direct as c_char,
        storev as c_char,
        m,
        n,
        k,
        v.as_ptr(),
        ldv,
        t.as_ptr(),
        ldt,
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn clarfb(
    layout: Layout,
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[c32],
    ldv: i32,
    t: &[c32],
    ldt: i32,
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_clarfb(
        layout.into(),
        side as c_char,
        trans as c_char,
        direct as c_char,
        storev as c_char,
        m,
        n,
        k,
        v.as_ptr() as *const _,
        ldv,
        t.as_ptr() as *const _,
        ldt,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zlarfb(
    layout: Layout,
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[c64],
    ldv: i32,
    t: &[c64],
    ldt: i32,
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zlarfb(
        layout.into(),
        side as c_char,
        trans as c_char,
        direct as c_char,
        storev as c_char,
        m,
        n,
        k,
        v.as_ptr() as *const _,
        ldv,
        t.as_ptr() as *const _,
        ldt,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn slarfg(n: i32, alpha: &mut f32, x: &mut [f32], incx: i32, tau: &mut [f32]) -> i32 {
    ffi::LAPACKE_slarfg(n, alpha, x.as_mut_ptr(), incx, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dlarfg(n: i32, alpha: &mut f64, x: &mut [f64], incx: i32, tau: &mut [f64]) -> i32 {
    ffi::LAPACKE_dlarfg(n, alpha, x.as_mut_ptr(), incx, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn clarfg(n: i32, alpha: &mut c32, x: &mut [c32], incx: i32, tau: &mut [c32]) -> i32 {
    ffi::LAPACKE_clarfg(
        n,
        alpha as *mut _ as *mut _,
        x.as_mut_ptr() as *mut _,
        incx,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zlarfg(n: i32, alpha: &mut c64, x: &mut [c64], incx: i32, tau: &mut [c64]) -> i32 {
    ffi::LAPACKE_zlarfg(
        n,
        alpha as *mut _ as *mut _,
        x.as_mut_ptr() as *mut _,
        incx,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn slarft(
    layout: Layout,
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[f32],
    ldv: i32,
    tau: &[f32],
    t: &mut [f32],
    ldt: i32,
) -> i32 {
    ffi::LAPACKE_slarft(
        layout.into(),
        direct as c_char,
        storev as c_char,
        n,
        k,
        v.as_ptr(),
        ldv,
        tau.as_ptr(),
        t.as_mut_ptr(),
        ldt,
    )
}

#[inline]
pub unsafe fn dlarft(
    layout: Layout,
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[f64],
    ldv: i32,
    tau: &[f64],
    t: &mut [f64],
    ldt: i32,
) -> i32 {
    ffi::LAPACKE_dlarft(
        layout.into(),
        direct as c_char,
        storev as c_char,
        n,
        k,
        v.as_ptr(),
        ldv,
        tau.as_ptr(),
        t.as_mut_ptr(),
        ldt,
    )
}

#[inline]
pub unsafe fn clarft(
    layout: Layout,
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[c32],
    ldv: i32,
    tau: &[c32],
    t: &mut [c32],
    ldt: i32,
) -> i32 {
    ffi::LAPACKE_clarft(
        layout.into(),
        direct as c_char,
        storev as c_char,
        n,
        k,
        v.as_ptr() as *const _,
        ldv,
        tau.as_ptr() as *const _,
        t.as_mut_ptr() as *mut _,
        ldt,
    )
}

#[inline]
pub unsafe fn zlarft(
    layout: Layout,
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[c64],
    ldv: i32,
    tau: &[c64],
    t: &mut [c64],
    ldt: i32,
) -> i32 {
    ffi::LAPACKE_zlarft(
        layout.into(),
        direct as c_char,
        storev as c_char,
        n,
        k,
        v.as_ptr() as *const _,
        ldv,
        tau.as_ptr() as *const _,
        t.as_mut_ptr() as *mut _,
        ldt,
    )
}

#[inline]
pub unsafe fn slarfx(
    layout: Layout,
    side: u8,
    m: i32,
    n: i32,
    v: &[f32],
    tau: f32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_slarfx(
        layout.into(),
        side as c_char,
        m,
        n,
        v.as_ptr(),
        tau,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlarfx(
    layout: Layout,
    side: u8,
    m: i32,
    n: i32,
    v: &[f64],
    tau: f64,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dlarfx(
        layout.into(),
        side as c_char,
        m,
        n,
        v.as_ptr(),
        tau,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clarfx(
    layout: Layout,
    side: u8,
    m: i32,
    n: i32,
    v: &[c32],
    tau: c32,
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_clarfx(
        layout.into(),
        side as c_char,
        m,
        n,
        v.as_ptr() as *const _,
        transmute(tau),
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zlarfx(
    layout: Layout,
    side: u8,
    m: i32,
    n: i32,
    v: &[c64],
    tau: c64,
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zlarfx(
        layout.into(),
        side as c_char,
        m,
        n,
        v.as_ptr() as *const _,
        transmute(tau),
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn slarnv(idist: i32, iseed: &mut [i32], n: i32, x: &mut [f32]) -> i32 {
    ffi::LAPACKE_slarnv(idist, iseed.as_mut_ptr(), n, x.as_mut_ptr())
}

#[inline]
pub unsafe fn dlarnv(idist: i32, iseed: &mut [i32], n: i32, x: &mut [f64]) -> i32 {
    ffi::LAPACKE_dlarnv(idist, iseed.as_mut_ptr(), n, x.as_mut_ptr())
}

#[inline]
pub unsafe fn clarnv(idist: i32, iseed: &mut [i32], n: i32, x: &mut [c32]) -> i32 {
    ffi::LAPACKE_clarnv(idist, iseed.as_mut_ptr(), n, x.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn zlarnv(idist: i32, iseed: &mut [i32], n: i32, x: &mut [c64]) -> i32 {
    ffi::LAPACKE_zlarnv(idist, iseed.as_mut_ptr(), n, x.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn slascl(
    layout: Layout,
    _type: u8,
    kl: i32,
    ku: i32,
    cfrom: f32,
    cto: f32,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_slascl(
        layout.into(),
        _type as c_char,
        kl,
        ku,
        cfrom,
        cto,
        m,
        n,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dlascl(
    layout: Layout,
    _type: u8,
    kl: i32,
    ku: i32,
    cfrom: f64,
    cto: f64,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_dlascl(
        layout.into(),
        _type as c_char,
        kl,
        ku,
        cfrom,
        cto,
        m,
        n,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn clascl(
    layout: Layout,
    _type: u8,
    kl: i32,
    ku: i32,
    cfrom: f32,
    cto: f32,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_clascl(
        layout.into(),
        _type as c_char,
        kl,
        ku,
        cfrom,
        cto,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zlascl(
    layout: Layout,
    _type: u8,
    kl: i32,
    ku: i32,
    cfrom: f64,
    cto: f64,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_zlascl(
        layout.into(),
        _type as c_char,
        kl,
        ku,
        cfrom,
        cto,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn slaset(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    alpha: f32,
    beta: f32,
    a: &mut [f32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_slaset(
        layout.into(),
        uplo as c_char,
        m,
        n,
        alpha,
        beta,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dlaset(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    alpha: f64,
    beta: f64,
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_dlaset(
        layout.into(),
        uplo as c_char,
        m,
        n,
        alpha,
        beta,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn claset(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    alpha: c32,
    beta: c32,
    a: &mut [c32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_claset(
        layout.into(),
        uplo as c_char,
        m,
        n,
        transmute(alpha),
        transmute(beta),
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zlaset(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    alpha: c64,
    beta: c64,
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_zlaset(
        layout.into(),
        uplo as c_char,
        m,
        n,
        transmute(alpha),
        transmute(beta),
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn slasrt(id: u8, n: i32, d: &mut [f32]) -> i32 {
    ffi::LAPACKE_slasrt(id as c_char, n, d.as_mut_ptr())
}

#[inline]
pub unsafe fn dlasrt(id: u8, n: i32, d: &mut [f64]) -> i32 {
    ffi::LAPACKE_dlasrt(id as c_char, n, d.as_mut_ptr())
}

#[inline]
pub unsafe fn slaswp(
    layout: Layout,
    n: i32,
    a: &mut [f32],
    lda: i32,
    k1: i32,
    k2: i32,
    ipiv: &[i32],
    incx: i32,
) -> i32 {
    ffi::LAPACKE_slaswp(
        layout.into(),
        n,
        a.as_mut_ptr(),
        lda,
        k1,
        k2,
        ipiv.as_ptr(),
        incx,
    )
}

#[inline]
pub unsafe fn dlaswp(
    layout: Layout,
    n: i32,
    a: &mut [f64],
    lda: i32,
    k1: i32,
    k2: i32,
    ipiv: &[i32],
    incx: i32,
) -> i32 {
    ffi::LAPACKE_dlaswp(
        layout.into(),
        n,
        a.as_mut_ptr(),
        lda,
        k1,
        k2,
        ipiv.as_ptr(),
        incx,
    )
}

#[inline]
pub unsafe fn claswp(
    layout: Layout,
    n: i32,
    a: &mut [c32],
    lda: i32,
    k1: i32,
    k2: i32,
    ipiv: &[i32],
    incx: i32,
) -> i32 {
    ffi::LAPACKE_claswp(
        layout.into(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        k1,
        k2,
        ipiv.as_ptr(),
        incx,
    )
}

#[inline]
pub unsafe fn zlaswp(
    layout: Layout,
    n: i32,
    a: &mut [c64],
    lda: i32,
    k1: i32,
    k2: i32,
    ipiv: &[i32],
    incx: i32,
) -> i32 {
    ffi::LAPACKE_zlaswp(
        layout.into(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        k1,
        k2,
        ipiv.as_ptr(),
        incx,
    )
}

#[inline]
pub unsafe fn slatms(
    layout: Layout,
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f32],
    mode: i32,
    cond: f32,
    dmax: f32,
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [f32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_slatms(
        layout.into(),
        m,
        n,
        dist as c_char,
        iseed.as_mut_ptr(),
        sym as c_char,
        d.as_mut_ptr(),
        mode,
        cond,
        dmax,
        kl,
        ku,
        pack as c_char,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dlatms(
    layout: Layout,
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f64],
    mode: i32,
    cond: f64,
    dmax: f64,
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_dlatms(
        layout.into(),
        m,
        n,
        dist as c_char,
        iseed.as_mut_ptr(),
        sym as c_char,
        d.as_mut_ptr(),
        mode,
        cond,
        dmax,
        kl,
        ku,
        pack as c_char,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn clatms(
    layout: Layout,
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f32],
    mode: i32,
    cond: f32,
    dmax: f32,
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [c32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_clatms(
        layout.into(),
        m,
        n,
        dist as c_char,
        iseed.as_mut_ptr(),
        sym as c_char,
        d.as_mut_ptr(),
        mode,
        cond,
        dmax,
        kl,
        ku,
        pack as c_char,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zlatms(
    layout: Layout,
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f64],
    mode: i32,
    cond: f64,
    dmax: f64,
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_zlatms(
        layout.into(),
        m,
        n,
        dist as c_char,
        iseed.as_mut_ptr(),
        sym as c_char,
        d.as_mut_ptr(),
        mode,
        cond,
        dmax,
        kl,
        ku,
        pack as c_char,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn slauum(layout: Layout, uplo: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_slauum(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dlauum(layout: Layout, uplo: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dlauum(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn clauum(layout: Layout, uplo: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_clauum(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zlauum(layout: Layout, uplo: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_zlauum(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn sopgtr(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    tau: &[f32],
    q: &mut f32,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_sopgtr(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        tau.as_ptr(),
        q,
        ldq,
    )
}

#[inline]
pub unsafe fn dopgtr(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    tau: &[f64],
    q: &mut f64,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_dopgtr(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        tau.as_ptr(),
        q,
        ldq,
    )
}

#[inline]
pub unsafe fn sopmtr(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[f32],
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sopmtr(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        ap.as_ptr(),
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dopmtr(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[f64],
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dopmtr(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        ap.as_ptr(),
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn sorgbr(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
) -> i32 {
    ffi::LAPACKE_sorgbr(
        layout.into(),
        vect as c_char,
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
    )
}

#[inline]
pub unsafe fn dorgbr(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
) -> i32 {
    ffi::LAPACKE_dorgbr(
        layout.into(),
        vect as c_char,
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
    )
}

#[inline]
pub unsafe fn sorghr(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
) -> i32 {
    ffi::LAPACKE_sorghr(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
    )
}

#[inline]
pub unsafe fn dorghr(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
) -> i32 {
    ffi::LAPACKE_dorghr(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
    )
}

#[inline]
pub unsafe fn sorglq(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
) -> i32 {
    ffi::LAPACKE_sorglq(layout.into(), m, n, k, a.as_mut_ptr(), lda, tau.as_ptr())
}

#[inline]
pub unsafe fn dorglq(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
) -> i32 {
    ffi::LAPACKE_dorglq(layout.into(), m, n, k, a.as_mut_ptr(), lda, tau.as_ptr())
}

#[inline]
pub unsafe fn sorgql(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
) -> i32 {
    ffi::LAPACKE_sorgql(layout.into(), m, n, k, a.as_mut_ptr(), lda, tau.as_ptr())
}

#[inline]
pub unsafe fn dorgql(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
) -> i32 {
    ffi::LAPACKE_dorgql(layout.into(), m, n, k, a.as_mut_ptr(), lda, tau.as_ptr())
}

#[inline]
pub unsafe fn sorgqr(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
) -> i32 {
    ffi::LAPACKE_sorgqr(layout.into(), m, n, k, a.as_mut_ptr(), lda, tau.as_ptr())
}

#[inline]
pub unsafe fn dorgqr(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
) -> i32 {
    ffi::LAPACKE_dorgqr(layout.into(), m, n, k, a.as_mut_ptr(), lda, tau.as_ptr())
}

#[inline]
pub unsafe fn sorgrq(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
) -> i32 {
    ffi::LAPACKE_sorgrq(layout.into(), m, n, k, a.as_mut_ptr(), lda, tau.as_ptr())
}

#[inline]
pub unsafe fn dorgrq(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
) -> i32 {
    ffi::LAPACKE_dorgrq(layout.into(), m, n, k, a.as_mut_ptr(), lda, tau.as_ptr())
}

#[inline]
pub unsafe fn sorgtr(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
) -> i32 {
    ffi::LAPACKE_sorgtr(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
    )
}

#[inline]
pub unsafe fn dorgtr(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
) -> i32 {
    ffi::LAPACKE_dorgtr(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
    )
}

#[inline]
pub unsafe fn sormbr(
    layout: Layout,
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sormbr(
        layout.into(),
        vect as c_char,
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dormbr(
    layout: Layout,
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dormbr(
        layout.into(),
        vect as c_char,
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn sormhr(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sormhr(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        ilo,
        ihi,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dormhr(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dormhr(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        ilo,
        ihi,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn sormlq(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sormlq(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dormlq(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dormlq(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn sormql(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sormql(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dormql(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dormql(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn sormqr(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sormqr(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dormqr(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dormqr(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn sormrq(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sormrq(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dormrq(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dormrq(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn sormrz(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sormrz(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        l,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dormrz(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dormrz(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        l,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn sormtr(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_sormtr(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn dormtr(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_dormtr(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
    )
}

#[inline]
pub unsafe fn spbcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_spbcon(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn dpbcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dpbcon(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn cpbcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_cpbcon(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zpbcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zpbcon(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn spbequ(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_spbequ(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dpbequ(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dpbequ(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cpbequ(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cpbequ(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zpbequ(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zpbequ(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn spbrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_spbrfs(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpbrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dpbrfs(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpbrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpbrfs(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpbrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpbrfs(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spbstf(layout: Layout, uplo: u8, n: i32, kb: i32, bb: &mut [f32], ldbb: i32) -> i32 {
    ffi::LAPACKE_spbstf(layout.into(), uplo as c_char, n, kb, bb.as_mut_ptr(), ldbb)
}

#[inline]
pub unsafe fn dpbstf(layout: Layout, uplo: u8, n: i32, kb: i32, bb: &mut [f64], ldbb: i32) -> i32 {
    ffi::LAPACKE_dpbstf(layout.into(), uplo as c_char, n, kb, bb.as_mut_ptr(), ldbb)
}

#[inline]
pub unsafe fn cpbstf(layout: Layout, uplo: u8, n: i32, kb: i32, bb: &mut [c32], ldbb: i32) -> i32 {
    ffi::LAPACKE_cpbstf(
        layout.into(),
        uplo as c_char,
        n,
        kb,
        bb.as_mut_ptr() as *mut _,
        ldbb,
    )
}

#[inline]
pub unsafe fn zpbstf(layout: Layout, uplo: u8, n: i32, kb: i32, bb: &mut [c64], ldbb: i32) -> i32 {
    ffi::LAPACKE_zpbstf(
        layout.into(),
        uplo as c_char,
        n,
        kb,
        bb.as_mut_ptr() as *mut _,
        ldbb,
    )
}

#[inline]
pub unsafe fn spbsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spbsv(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpbsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpbsv(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpbsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpbsv(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpbsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpbsv(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn spbsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_spbsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpbsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dpbsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpbsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpbsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpbsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpbsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spbtrf(layout: Layout, uplo: u8, n: i32, kd: i32, ab: &mut [f32], ldab: i32) -> i32 {
    ffi::LAPACKE_spbtrf(layout.into(), uplo as c_char, n, kd, ab.as_mut_ptr(), ldab)
}

#[inline]
pub unsafe fn dpbtrf(layout: Layout, uplo: u8, n: i32, kd: i32, ab: &mut [f64], ldab: i32) -> i32 {
    ffi::LAPACKE_dpbtrf(layout.into(), uplo as c_char, n, kd, ab.as_mut_ptr(), ldab)
}

#[inline]
pub unsafe fn cpbtrf(layout: Layout, uplo: u8, n: i32, kd: i32, ab: &mut [c32], ldab: i32) -> i32 {
    ffi::LAPACKE_cpbtrf(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
    )
}

#[inline]
pub unsafe fn zpbtrf(layout: Layout, uplo: u8, n: i32, kd: i32, ab: &mut [c64], ldab: i32) -> i32 {
    ffi::LAPACKE_zpbtrf(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
    )
}

#[inline]
pub unsafe fn spbtrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spbtrs(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpbtrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpbtrs(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpbtrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpbtrs(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpbtrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpbtrs(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn spftrf(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [f32]) -> i32 {
    ffi::LAPACKE_spftrf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpftrf(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpftrf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpftrf(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpftrf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zpftrf(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpftrf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn spftri(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [f32]) -> i32 {
    ffi::LAPACKE_spftri(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpftri(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpftri(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpftri(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpftri(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zpftri(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpftri(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn spftrs(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spftrs(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpftrs(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpftrs(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpftrs(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpftrs(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpftrs(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpftrs(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn spocon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_spocon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn dpocon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dpocon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn cpocon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_cpocon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zpocon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zpocon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn spoequ(
    layout: Layout,
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_spoequ(
        layout.into(),
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dpoequ(
    layout: Layout,
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dpoequ(
        layout.into(),
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cpoequ(
    layout: Layout,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cpoequ(
        layout.into(),
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zpoequ(
    layout: Layout,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zpoequ(
        layout.into(),
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn spoequb(
    layout: Layout,
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_spoequb(
        layout.into(),
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dpoequb(
    layout: Layout,
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dpoequb(
        layout.into(),
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cpoequb(
    layout: Layout,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cpoequb(
        layout.into(),
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zpoequb(
    layout: Layout,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zpoequb(
        layout.into(),
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn sporfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sporfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dporfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dporfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cporfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cporfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zporfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zporfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sporfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    s: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sporfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        s.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dporfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    s: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dporfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        s.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cporfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    s: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cporfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zporfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    s: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zporfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sposv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sposv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dposv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dposv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cposv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cposv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zposv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zposv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn dsposv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    iter: &mut i32,
) -> i32 {
    ffi::LAPACKE_dsposv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        iter,
    )
}

#[inline]
pub unsafe fn zcposv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    iter: &mut i32,
) -> i32 {
    ffi::LAPACKE_zcposv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        iter,
    )
}

#[inline]
pub unsafe fn sposvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sposvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dposvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dposvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cposvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cposvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zposvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zposvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sposvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sposvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dposvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dposvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cposvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cposvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zposvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zposvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spotrf2(layout: Layout, uplo: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_spotrf2(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dpotrf2(layout: Layout, uplo: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dpotrf2(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn cpotrf2(layout: Layout, uplo: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_cpotrf2(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zpotrf2(layout: Layout, uplo: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_zpotrf2(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn spotrf(layout: Layout, uplo: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_spotrf(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dpotrf(layout: Layout, uplo: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dpotrf(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn cpotrf(layout: Layout, uplo: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_cpotrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zpotrf(layout: Layout, uplo: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_zpotrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn spotri(layout: Layout, uplo: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_spotri(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dpotri(layout: Layout, uplo: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dpotri(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn cpotri(layout: Layout, uplo: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_cpotri(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zpotri(layout: Layout, uplo: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_zpotri(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn spotrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spotrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpotrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpotrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpotrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpotrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpotrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpotrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sppcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_sppcon(layout.into(), uplo as c_char, n, ap.as_ptr(), anorm, rcond)
}

#[inline]
pub unsafe fn dppcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dppcon(layout.into(), uplo as c_char, n, ap.as_ptr(), anorm, rcond)
}

#[inline]
pub unsafe fn cppcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_cppcon(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zppcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zppcon(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn sppequ(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sppequ(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dppequ(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dppequ(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cppequ(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cppequ(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zppequ(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zppequ(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn spprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_spprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dpprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sppsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sppsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dppsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dppsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cppsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cppsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zppsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zppsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sppsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    afp: &mut [f32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sppsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        afp.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dppsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    afp: &mut [f64],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dppsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        afp.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cppsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    afp: &mut [c32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cppsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        afp.as_mut_ptr() as *mut _,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zppsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    afp: &mut [c64],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zppsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        afp.as_mut_ptr() as *mut _,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [f32]) -> i32 {
    ffi::LAPACKE_spptrf(layout.into(), uplo as c_char, n, ap.as_mut_ptr())
}

#[inline]
pub unsafe fn dpptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpptrf(layout.into(), uplo as c_char, n, ap.as_mut_ptr())
}

#[inline]
pub unsafe fn cpptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpptrf(layout.into(), uplo as c_char, n, ap.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn zpptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpptrf(layout.into(), uplo as c_char, n, ap.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn spptri(layout: Layout, uplo: u8, n: i32, ap: &mut [f32]) -> i32 {
    ffi::LAPACKE_spptri(layout.into(), uplo as c_char, n, ap.as_mut_ptr())
}

#[inline]
pub unsafe fn dpptri(layout: Layout, uplo: u8, n: i32, ap: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpptri(layout.into(), uplo as c_char, n, ap.as_mut_ptr())
}

#[inline]
pub unsafe fn cpptri(layout: Layout, uplo: u8, n: i32, ap: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpptri(layout.into(), uplo as c_char, n, ap.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn zpptri(layout: Layout, uplo: u8, n: i32, ap: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpptri(layout.into(), uplo as c_char, n, ap.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn spptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn spstrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f32,
) -> i32 {
    ffi::LAPACKE_spstrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        piv.as_mut_ptr(),
        rank,
        tol,
    )
}

#[inline]
pub unsafe fn dpstrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f64,
) -> i32 {
    ffi::LAPACKE_dpstrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        piv.as_mut_ptr(),
        rank,
        tol,
    )
}

#[inline]
pub unsafe fn cpstrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f32,
) -> i32 {
    ffi::LAPACKE_cpstrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        piv.as_mut_ptr(),
        rank,
        tol,
    )
}

#[inline]
pub unsafe fn zpstrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f64,
) -> i32 {
    ffi::LAPACKE_zpstrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        piv.as_mut_ptr(),
        rank,
        tol,
    )
}

#[inline]
pub unsafe fn sptcon(n: i32, d: &[f32], e: &[f32], anorm: f32, rcond: &mut f32) -> i32 {
    ffi::LAPACKE_sptcon(n, d.as_ptr(), e.as_ptr(), anorm, rcond)
}

#[inline]
pub unsafe fn dptcon(n: i32, d: &[f64], e: &[f64], anorm: f64, rcond: &mut f64) -> i32 {
    ffi::LAPACKE_dptcon(n, d.as_ptr(), e.as_ptr(), anorm, rcond)
}

#[inline]
pub unsafe fn cptcon(n: i32, d: &[f32], e: &[c32], anorm: f32, rcond: &mut f32) -> i32 {
    ffi::LAPACKE_cptcon(n, d.as_ptr(), e.as_ptr() as *const _, anorm, rcond)
}

#[inline]
pub unsafe fn zptcon(n: i32, d: &[f64], e: &[c64], anorm: f64, rcond: &mut f64) -> i32 {
    ffi::LAPACKE_zptcon(n, d.as_ptr(), e.as_ptr() as *const _, anorm, rcond)
}

#[inline]
pub unsafe fn spteqr(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_spteqr(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dpteqr(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dpteqr(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn cpteqr(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_cpteqr(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zpteqr(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zpteqr(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn sptrfs(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    df: &[f32],
    ef: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sptrfs(
        layout.into(),
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_ptr(),
        ef.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dptrfs(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    df: &[f64],
    ef: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dptrfs(
        layout.into(),
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_ptr(),
        ef.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cptrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    df: &[f32],
    ef: &[c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cptrfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_ptr(),
        ef.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zptrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    df: &[f64],
    ef: &[c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zptrfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_ptr(),
        ef.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sptsv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &mut [f32],
    e: &mut [f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sptsv(
        layout.into(),
        n,
        nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dptsv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &mut [f64],
    e: &mut [f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dptsv(
        layout.into(),
        n,
        nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cptsv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &mut [f32],
    e: &mut [c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cptsv(
        layout.into(),
        n,
        nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zptsv(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &mut [f64],
    e: &mut [c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zptsv(
        layout.into(),
        n,
        nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sptsvx(
    layout: Layout,
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    df: &mut [f32],
    ef: &mut [f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sptsvx(
        layout.into(),
        fact as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_mut_ptr(),
        ef.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dptsvx(
    layout: Layout,
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    df: &mut [f64],
    ef: &mut [f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dptsvx(
        layout.into(),
        fact as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_mut_ptr(),
        ef.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cptsvx(
    layout: Layout,
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    df: &mut [f32],
    ef: &mut [c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cptsvx(
        layout.into(),
        fact as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_mut_ptr(),
        ef.as_mut_ptr() as *mut _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zptsvx(
    layout: Layout,
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    df: &mut [f64],
    ef: &mut [c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zptsvx(
        layout.into(),
        fact as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_mut_ptr(),
        ef.as_mut_ptr() as *mut _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spttrf(n: i32, d: &mut [f32], e: &mut [f32]) -> i32 {
    ffi::LAPACKE_spttrf(n, d.as_mut_ptr(), e.as_mut_ptr())
}

#[inline]
pub unsafe fn dpttrf(n: i32, d: &mut [f64], e: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpttrf(n, d.as_mut_ptr(), e.as_mut_ptr())
}

#[inline]
pub unsafe fn cpttrf(n: i32, d: &mut [f32], e: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpttrf(n, d.as_mut_ptr(), e.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn zpttrf(n: i32, d: &mut [f64], e: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpttrf(n, d.as_mut_ptr(), e.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn spttrs(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spttrs(
        layout.into(),
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpttrs(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpttrs(
        layout.into(),
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpttrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpttrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpttrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpttrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ssbev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_ssbev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dsbev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dsbev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn ssbevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_ssbevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dsbevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dsbevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn ssbevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    q: &mut f32,
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssbevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        q,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsbevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    q: &mut f64,
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsbevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        q,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssbgst(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &[f32],
    ldbb: i32,
    x: &mut [f32],
    ldx: i32,
) -> i32 {
    ffi::LAPACKE_ssbgst(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_ptr(),
        ldbb,
        x.as_mut_ptr(),
        ldx,
    )
}

#[inline]
pub unsafe fn dsbgst(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &[f64],
    ldbb: i32,
    x: &mut [f64],
    ldx: i32,
) -> i32 {
    ffi::LAPACKE_dsbgst(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_ptr(),
        ldbb,
        x.as_mut_ptr(),
        ldx,
    )
}

#[inline]
pub unsafe fn ssbgv(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_ssbgv(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dsbgv(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dsbgv(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn ssbgvd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_ssbgvd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dsbgvd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dsbgvd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn ssbgvx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    q: &mut f32,
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssbgvx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        q,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsbgvx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    q: &mut f64,
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsbgvx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        q,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssbtrd(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut f32,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_ssbtrd(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q,
        ldq,
    )
}

#[inline]
pub unsafe fn dsbtrd(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut f64,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_dsbtrd(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q,
        ldq,
    )
}

#[inline]
pub unsafe fn ssfrk(
    layout: Layout,
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    beta: f32,
    c: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssfrk(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        trans as c_char,
        n,
        k,
        alpha,
        a.as_ptr(),
        lda,
        beta,
        c.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsfrk(
    layout: Layout,
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: f64,
    a: &[f64],
    lda: i32,
    beta: f64,
    c: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsfrk(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        trans as c_char,
        n,
        k,
        alpha,
        a.as_ptr(),
        lda,
        beta,
        c.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sspcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_sspcon(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn dspcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dspcon(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn cspcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_cspcon(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zspcon(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zspcon(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn sspev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sspev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dspev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dspev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn sspevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sspevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dspevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dspevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn sspevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sspevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dspevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sspgst(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &[f32],
) -> i32 {
    ffi::LAPACKE_sspgst(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_ptr(),
    )
}

#[inline]
pub unsafe fn dspgst(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &[f64],
) -> i32 {
    ffi::LAPACKE_dspgst(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_ptr(),
    )
}

#[inline]
pub unsafe fn sspgv(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sspgv(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dspgv(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dspgv(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn sspgvd(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sspgvd(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dspgvd(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dspgvd(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn sspgvx(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sspgvx(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspgvx(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dspgvx(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &[f32],
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &[f64],
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsprfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsprfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sspsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sspsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dspsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dspsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cspsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cspsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zspsv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zspsv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sspsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &mut [f32],
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sspsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &mut [f64],
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dspsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cspsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cspsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zspsvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zspsvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssptrd(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssptrd(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsptrd(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsptrd(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [f32], ipiv: &mut [i32]) -> i32 {
    ffi::LAPACKE_ssptrf(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [f64], ipiv: &mut [i32]) -> i32 {
    ffi::LAPACKE_dsptrf(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [c32], ipiv: &mut [i32]) -> i32 {
    ffi::LAPACKE_csptrf(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsptrf(layout: Layout, uplo: u8, n: i32, ap: &mut [c64], ipiv: &mut [i32]) -> i32 {
    ffi::LAPACKE_zsptrf(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssptri(layout: Layout, uplo: u8, n: i32, ap: &mut [f32], ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_ssptri(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn dsptri(layout: Layout, uplo: u8, n: i32, ap: &mut [f64], ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_dsptri(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn csptri(layout: Layout, uplo: u8, n: i32, ap: &mut [c32], ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_csptri(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn zsptri(layout: Layout, uplo: u8, n: i32, ap: &mut [c64], ipiv: &[i32]) -> i32 {
    ffi::LAPACKE_zsptri(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn ssptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ssptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dsptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dsptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn csptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_csptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zsptrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zsptrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sstebz(
    range: u8,
    order: u8,
    n: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    d: &[f32],
    e: &[f32],
    m: &mut i32,
    nsplit: &mut [i32],
    w: &mut [f32],
    iblock: &mut [i32],
    isplit: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sstebz(
        range as c_char,
        order as c_char,
        n,
        vl,
        vu,
        il,
        iu,
        abstol,
        d.as_ptr(),
        e.as_ptr(),
        m,
        nsplit.as_mut_ptr(),
        w.as_mut_ptr(),
        iblock.as_mut_ptr(),
        isplit.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstebz(
    range: u8,
    order: u8,
    n: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    d: &[f64],
    e: &[f64],
    m: &mut i32,
    nsplit: &mut [i32],
    w: &mut [f64],
    iblock: &mut [i32],
    isplit: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dstebz(
        range as c_char,
        order as c_char,
        n,
        vl,
        vu,
        il,
        iu,
        abstol,
        d.as_ptr(),
        e.as_ptr(),
        m,
        nsplit.as_mut_ptr(),
        w.as_mut_ptr(),
        iblock.as_mut_ptr(),
        isplit.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sstedc(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sstedc(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dstedc(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dstedc(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn cstedc(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_cstedc(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zstedc(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zstedc(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn sstegr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sstegr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstegr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dstegr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cstegr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cstegr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zstegr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zstegr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sstein(
    layout: Layout,
    n: i32,
    d: &[f32],
    e: &[f32],
    m: i32,
    w: &[f32],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [f32],
    ldz: i32,
    ifailv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sstein(
        layout.into(),
        n,
        d.as_ptr(),
        e.as_ptr(),
        m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifailv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstein(
    layout: Layout,
    n: i32,
    d: &[f64],
    e: &[f64],
    m: i32,
    w: &[f64],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [f64],
    ldz: i32,
    ifailv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dstein(
        layout.into(),
        n,
        d.as_ptr(),
        e.as_ptr(),
        m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifailv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cstein(
    layout: Layout,
    n: i32,
    d: &[f32],
    e: &[f32],
    m: i32,
    w: &[f32],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [c32],
    ldz: i32,
    ifailv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cstein(
        layout.into(),
        n,
        d.as_ptr(),
        e.as_ptr(),
        m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifailv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zstein(
    layout: Layout,
    n: i32,
    d: &[f64],
    e: &[f64],
    m: i32,
    w: &[f64],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [c64],
    ldz: i32,
    ifailv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zstein(
        layout.into(),
        n,
        d.as_ptr(),
        e.as_ptr(),
        m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        ifailv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sstemr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    nzc: i32,
    isuppz: &mut [i32],
    tryrac: &mut i32,
) -> i32 {
    ffi::LAPACKE_sstemr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        nzc,
        isuppz.as_mut_ptr(),
        tryrac,
    )
}

#[inline]
pub unsafe fn dstemr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    nzc: i32,
    isuppz: &mut [i32],
    tryrac: &mut i32,
) -> i32 {
    ffi::LAPACKE_dstemr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        nzc,
        isuppz.as_mut_ptr(),
        tryrac,
    )
}

#[inline]
pub unsafe fn cstemr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    nzc: i32,
    isuppz: &mut [i32],
    tryrac: &mut i32,
) -> i32 {
    ffi::LAPACKE_cstemr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        nzc,
        isuppz.as_mut_ptr(),
        tryrac,
    )
}

#[inline]
pub unsafe fn zstemr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    nzc: i32,
    isuppz: &mut [i32],
    tryrac: &mut i32,
) -> i32 {
    ffi::LAPACKE_zstemr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        nzc,
        isuppz.as_mut_ptr(),
        tryrac,
    )
}

#[inline]
pub unsafe fn ssteqr(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_ssteqr(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dsteqr(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dsteqr(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn csteqr(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_csteqr(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zsteqr(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zsteqr(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn ssterf(n: i32, d: &mut [f32], e: &mut [f32]) -> i32 {
    ffi::LAPACKE_ssterf(n, d.as_mut_ptr(), e.as_mut_ptr())
}

#[inline]
pub unsafe fn dsterf(n: i32, d: &mut [f64], e: &mut [f64]) -> i32 {
    ffi::LAPACKE_dsterf(n, d.as_mut_ptr(), e.as_mut_ptr())
}

#[inline]
pub unsafe fn sstev(
    layout: Layout,
    jobz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sstev(
        layout.into(),
        jobz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dstev(
    layout: Layout,
    jobz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dstev(
        layout.into(),
        jobz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn sstevd(
    layout: Layout,
    jobz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sstevd(
        layout.into(),
        jobz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dstevd(
    layout: Layout,
    jobz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dstevd(
        layout.into(),
        jobz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn sstevr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sstevr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstevr(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dstevr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sstevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sstevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dstevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssycon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_ssycon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn dsycon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dsycon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn csycon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_csycon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn zsycon(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_zsycon(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
    )
}

#[inline]
pub unsafe fn ssyequb(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_ssyequb(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dsyequb(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dsyequb(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn csyequb(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_csyequb(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zsyequb(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zsyequb(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn ssyev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    w: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssyev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyev(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    w: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsyev(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssyevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    w: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssyevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyevd(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    w: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsyevd(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssyevr(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssyevr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyevr(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsyevr(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssyevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssyevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyevx(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsyevx(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssygst(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ssygst(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dsygst(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dsygst(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ssygv(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    w: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssygv(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsygv(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    w: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsygv(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssygvd(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    w: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssygvd(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsygvd(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    w: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsygvd(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        w.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssygvx(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssygvx(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsygvx(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsygvx(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssyrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssyrfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsyrfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csyrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csyrfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsyrfs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsyrfs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssyrfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssyrfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyrfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsyrfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csyrfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csyrfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsyrfsx(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsyrfsx(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssysv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ssysv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dsysv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dsysv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn csysv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_csysv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zsysv(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zsysv(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ssysvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssysvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsysvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsysvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csysvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csysvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsysvx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsysvx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssysvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssysvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsysvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsysvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csysvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csysvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsysvxx(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsysvxx(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssytrd(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssytrd(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsytrd(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsytrd(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssytrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssytrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsytrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsytrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csytrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_csytrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsytrf(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zsytrf(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssytri(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
) -> i32 {
    ffi::LAPACKE_ssytri(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn dsytri(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
) -> i32 {
    ffi::LAPACKE_dsytri(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn csytri(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
) -> i32 {
    ffi::LAPACKE_csytri(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn zsytri(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
) -> i32 {
    ffi::LAPACKE_zsytri(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
    )
}

#[inline]
pub unsafe fn ssytrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ssytrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dsytrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dsytrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn csytrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_csytrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zsytrs(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zsytrs(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn stbcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_stbcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        rcond,
    )
}

#[inline]
pub unsafe fn dtbcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dtbcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        rcond,
    )
}

#[inline]
pub unsafe fn ctbcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_ctbcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        rcond,
    )
}

#[inline]
pub unsafe fn ztbcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_ztbcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        rcond,
    )
}

#[inline]
pub unsafe fn stbrfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &[f32],
    ldb: i32,
    x: &[f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stbrfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtbrfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &[f64],
    ldb: i32,
    x: &[f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtbrfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctbrfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &[c32],
    ldb: i32,
    x: &[c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctbrfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztbrfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &[c64],
    ldb: i32,
    x: &[c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztbrfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stbtrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_stbtrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dtbtrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dtbtrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ctbtrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ctbtrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ztbtrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ztbtrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn stfsm(
    layout: Layout,
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: f32,
    a: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_stfsm(
        layout.into(),
        transr as c_char,
        side as c_char,
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        m,
        n,
        alpha,
        a.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dtfsm(
    layout: Layout,
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: f64,
    a: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dtfsm(
        layout.into(),
        transr as c_char,
        side as c_char,
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        m,
        n,
        alpha,
        a.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ctfsm(
    layout: Layout,
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: c32,
    a: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ctfsm(
        layout.into(),
        transr as c_char,
        side as c_char,
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        m,
        n,
        transmute(alpha),
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ztfsm(
    layout: Layout,
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: c64,
    a: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ztfsm(
        layout.into(),
        transr as c_char,
        side as c_char,
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        m,
        n,
        transmute(alpha),
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn stftri(layout: Layout, transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [f32]) -> i32 {
    ffi::LAPACKE_stftri(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtftri(layout: Layout, transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [f64]) -> i32 {
    ffi::LAPACKE_dtftri(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctftri(layout: Layout, transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [c32]) -> i32 {
    ffi::LAPACKE_ctftri(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztftri(layout: Layout, transr: u8, uplo: u8, diag: u8, n: i32, a: &mut [c64]) -> i32 {
    ffi::LAPACKE_ztftri(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stfttp(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f32],
    ap: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stfttp(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr(),
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtfttp(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f64],
    ap: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtfttp(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr(),
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctfttp(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c32],
    ap: &mut [c32],
) -> i32 {
    ffi::LAPACKE_ctfttp(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr() as *const _,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztfttp(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c64],
    ap: &mut [c64],
) -> i32 {
    ffi::LAPACKE_ztfttp(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr() as *const _,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stfttr(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f32],
    a: &mut [f32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_stfttr(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr(),
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dtfttr(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f64],
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_dtfttr(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr(),
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn ctfttr(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c32],
    a: &mut [c32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_ctfttr(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn ztfttr(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c64],
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_ztfttr(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn stgevc(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[f32],
    lds: i32,
    p: &[f32],
    ldp: i32,
    vl: &mut f32,
    ldvl: i32,
    vr: &mut f32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_stgevc(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        s.as_ptr(),
        lds,
        p.as_ptr(),
        ldp,
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn dtgevc(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[f64],
    lds: i32,
    p: &[f64],
    ldp: i32,
    vl: &mut f64,
    ldvl: i32,
    vr: &mut f64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_dtgevc(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        s.as_ptr(),
        lds,
        p.as_ptr(),
        ldp,
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn ctgevc(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[c32],
    lds: i32,
    p: &[c32],
    ldp: i32,
    vl: &mut c32,
    ldvl: i32,
    vr: &mut c32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_ctgevc(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        s.as_ptr() as *const _,
        lds,
        p.as_ptr() as *const _,
        ldp,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn ztgevc(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[c64],
    lds: i32,
    p: &[c64],
    ldp: i32,
    vl: &mut c64,
    ldvl: i32,
    vr: &mut c64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_ztgevc(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        s.as_ptr() as *const _,
        lds,
        p.as_ptr() as *const _,
        ldp,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn stgexc(
    layout: Layout,
    wantq: i32,
    wantz: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stgexc(
        layout.into(),
        wantq,
        wantz,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtgexc(
    layout: Layout,
    wantq: i32,
    wantz: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtgexc(
        layout.into(),
        wantq,
        wantz,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctgexc(
    layout: Layout,
    wantq: i32,
    wantz: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    ifst: i32,
    ilst: i32,
) -> i32 {
    ffi::LAPACKE_ctgexc(
        layout.into(),
        wantq,
        wantz,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        ifst,
        ilst,
    )
}

#[inline]
pub unsafe fn ztgexc(
    layout: Layout,
    wantq: i32,
    wantz: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    ifst: i32,
    ilst: i32,
) -> i32 {
    ffi::LAPACKE_ztgexc(
        layout.into(),
        wantq,
        wantz,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        ifst,
        ilst,
    )
}

#[inline]
pub unsafe fn stgsen(
    layout: Layout,
    ijob: i32,
    wantq: i32,
    wantz: i32,
    select: &[i32],
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f32],
    pr: &mut [f32],
    dif: &mut f32,
) -> i32 {
    ffi::LAPACKE_stgsen(
        layout.into(),
        ijob,
        wantq,
        wantz,
        select.as_ptr(),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif,
    )
}

#[inline]
pub unsafe fn dtgsen(
    layout: Layout,
    ijob: i32,
    wantq: i32,
    wantz: i32,
    select: &[i32],
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f64],
    pr: &mut [f64],
    dif: &mut f64,
) -> i32 {
    ffi::LAPACKE_dtgsen(
        layout.into(),
        ijob,
        wantq,
        wantz,
        select.as_ptr(),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif,
    )
}

#[inline]
pub unsafe fn ctgsen(
    layout: Layout,
    ijob: i32,
    wantq: i32,
    wantz: i32,
    select: &[i32],
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f32],
    pr: &mut [f32],
    dif: &mut f32,
) -> i32 {
    ffi::LAPACKE_ctgsen(
        layout.into(),
        ijob,
        wantq,
        wantz,
        select.as_ptr(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif,
    )
}

#[inline]
pub unsafe fn ztgsen(
    layout: Layout,
    ijob: i32,
    wantq: i32,
    wantz: i32,
    select: &[i32],
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f64],
    pr: &mut [f64],
    dif: &mut f64,
) -> i32 {
    ffi::LAPACKE_ztgsen(
        layout.into(),
        ijob,
        wantq,
        wantz,
        select.as_ptr(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif,
    )
}

#[inline]
pub unsafe fn stgsja(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
    ncycle: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stgsja(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        ncycle.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtgsja(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
    ncycle: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtgsja(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        ncycle.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctgsja(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
    ncycle: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ctgsja(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        ncycle.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztgsja(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
    ncycle: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ztgsja(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        ncycle.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stgsna(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    vl: &[f32],
    ldvl: i32,
    vr: &[f32],
    ldvr: i32,
    s: &mut [f32],
    dif: &mut f32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_stgsna(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        vl.as_ptr(),
        ldvl,
        vr.as_ptr(),
        ldvr,
        s.as_mut_ptr(),
        dif,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn dtgsna(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    vl: &[f64],
    ldvl: i32,
    vr: &[f64],
    ldvr: i32,
    s: &mut [f64],
    dif: &mut f64,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_dtgsna(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        vl.as_ptr(),
        ldvl,
        vr.as_ptr(),
        ldvr,
        s.as_mut_ptr(),
        dif,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn ctgsna(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    vl: &[c32],
    ldvl: i32,
    vr: &[c32],
    ldvr: i32,
    s: &mut [f32],
    dif: &mut f32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_ctgsna(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        vl.as_ptr() as *const _,
        ldvl,
        vr.as_ptr() as *const _,
        ldvr,
        s.as_mut_ptr(),
        dif,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn ztgsna(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    vl: &[c64],
    ldvl: i32,
    vr: &[c64],
    ldvr: i32,
    s: &mut [f64],
    dif: &mut f64,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_ztgsna(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        vl.as_ptr() as *const _,
        ldvl,
        vr.as_ptr() as *const _,
        ldvr,
        s.as_mut_ptr(),
        dif,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn stgsyl(
    layout: Layout,
    trans: u8,
    ijob: i32,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    c: &mut [f32],
    ldc: i32,
    d: &[f32],
    ldd: i32,
    e: &[f32],
    lde: i32,
    f: &mut [f32],
    ldf: i32,
    scale: &mut [f32],
    dif: &mut f32,
) -> i32 {
    ffi::LAPACKE_stgsyl(
        layout.into(),
        trans as c_char,
        ijob,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        c.as_mut_ptr(),
        ldc,
        d.as_ptr(),
        ldd,
        e.as_ptr(),
        lde,
        f.as_mut_ptr(),
        ldf,
        scale.as_mut_ptr(),
        dif,
    )
}

#[inline]
pub unsafe fn dtgsyl(
    layout: Layout,
    trans: u8,
    ijob: i32,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    c: &mut [f64],
    ldc: i32,
    d: &[f64],
    ldd: i32,
    e: &[f64],
    lde: i32,
    f: &mut [f64],
    ldf: i32,
    scale: &mut [f64],
    dif: &mut f64,
) -> i32 {
    ffi::LAPACKE_dtgsyl(
        layout.into(),
        trans as c_char,
        ijob,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        c.as_mut_ptr(),
        ldc,
        d.as_ptr(),
        ldd,
        e.as_ptr(),
        lde,
        f.as_mut_ptr(),
        ldf,
        scale.as_mut_ptr(),
        dif,
    )
}

#[inline]
pub unsafe fn ctgsyl(
    layout: Layout,
    trans: u8,
    ijob: i32,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    c: &mut [c32],
    ldc: i32,
    d: &[c32],
    ldd: i32,
    e: &[c32],
    lde: i32,
    f: &mut [c32],
    ldf: i32,
    scale: &mut [f32],
    dif: &mut f32,
) -> i32 {
    ffi::LAPACKE_ctgsyl(
        layout.into(),
        trans as c_char,
        ijob,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        c.as_mut_ptr() as *mut _,
        ldc,
        d.as_ptr() as *const _,
        ldd,
        e.as_ptr() as *const _,
        lde,
        f.as_mut_ptr() as *mut _,
        ldf,
        scale.as_mut_ptr(),
        dif,
    )
}

#[inline]
pub unsafe fn ztgsyl(
    layout: Layout,
    trans: u8,
    ijob: i32,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    c: &mut [c64],
    ldc: i32,
    d: &[c64],
    ldd: i32,
    e: &[c64],
    lde: i32,
    f: &mut [c64],
    ldf: i32,
    scale: &mut [f64],
    dif: &mut f64,
) -> i32 {
    ffi::LAPACKE_ztgsyl(
        layout.into(),
        trans as c_char,
        ijob,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        c.as_mut_ptr() as *mut _,
        ldc,
        d.as_ptr() as *const _,
        ldd,
        e.as_ptr() as *const _,
        lde,
        f.as_mut_ptr() as *mut _,
        ldf,
        scale.as_mut_ptr(),
        dif,
    )
}

#[inline]
pub unsafe fn stpcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[f32],
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_stpcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_ptr(),
        rcond,
    )
}

#[inline]
pub unsafe fn dtpcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[f64],
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dtpcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_ptr(),
        rcond,
    )
}

#[inline]
pub unsafe fn ctpcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[c32],
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_ctpcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_ptr() as *const _,
        rcond,
    )
}

#[inline]
pub unsafe fn ztpcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[c64],
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_ztpcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_ptr() as *const _,
        rcond,
    )
}

#[inline]
pub unsafe fn stprfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &[f32],
    ldb: i32,
    x: &[f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stprfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtprfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &[f64],
    ldb: i32,
    x: &[f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtprfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctprfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &[c32],
    ldb: i32,
    x: &[c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctprfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztprfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &[c64],
    ldb: i32,
    x: &[c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztprfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stptri(layout: Layout, uplo: u8, diag: u8, n: i32, ap: &mut [f32]) -> i32 {
    ffi::LAPACKE_stptri(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtptri(layout: Layout, uplo: u8, diag: u8, n: i32, ap: &mut [f64]) -> i32 {
    ffi::LAPACKE_dtptri(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctptri(layout: Layout, uplo: u8, diag: u8, n: i32, ap: &mut [c32]) -> i32 {
    ffi::LAPACKE_ctptri(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztptri(layout: Layout, uplo: u8, diag: u8, n: i32, ap: &mut [c64]) -> i32 {
    ffi::LAPACKE_ztptri(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stptrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_stptrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dtptrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dtptrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ctptrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ctptrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ztptrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ztptrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn stpttf(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    ap: &[f32],
    arf: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stpttf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        ap.as_ptr(),
        arf.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtpttf(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    ap: &[f64],
    arf: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtpttf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        ap.as_ptr(),
        arf.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctpttf(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    ap: &[c32],
    arf: &mut [c32],
) -> i32 {
    ffi::LAPACKE_ctpttf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        arf.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztpttf(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    ap: &[c64],
    arf: &mut [c64],
) -> i32 {
    ffi::LAPACKE_ztpttf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        arf.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stpttr(layout: Layout, uplo: u8, n: i32, ap: &[f32], a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_stpttr(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dtpttr(layout: Layout, uplo: u8, n: i32, ap: &[f64], a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dtpttr(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn ctpttr(layout: Layout, uplo: u8, n: i32, ap: &[c32], a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_ctpttr(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn ztpttr(layout: Layout, uplo: u8, n: i32, ap: &[c64], a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_ztpttr(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn strcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_strcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_ptr(),
        lda,
        rcond,
    )
}

#[inline]
pub unsafe fn dtrcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_dtrcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_ptr(),
        lda,
        rcond,
    )
}

#[inline]
pub unsafe fn ctrcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    rcond: &mut f32,
) -> i32 {
    ffi::LAPACKE_ctrcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        rcond,
    )
}

#[inline]
pub unsafe fn ztrcon(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    rcond: &mut f64,
) -> i32 {
    ffi::LAPACKE_ztrcon(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        rcond,
    )
}

#[inline]
pub unsafe fn strevc(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &mut [i32],
    n: i32,
    t: &[f32],
    ldt: i32,
    vl: &mut f32,
    ldvl: i32,
    vr: &mut f32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_strevc(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_mut_ptr(),
        n,
        t.as_ptr(),
        ldt,
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn dtrevc(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &mut [i32],
    n: i32,
    t: &[f64],
    ldt: i32,
    vl: &mut f64,
    ldvl: i32,
    vr: &mut f64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_dtrevc(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_mut_ptr(),
        n,
        t.as_ptr(),
        ldt,
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn ctrevc(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &mut [c32],
    ldt: i32,
    vl: &mut c32,
    ldvl: i32,
    vr: &mut c32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_ctrevc(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        t.as_mut_ptr() as *mut _,
        ldt,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn ztrevc(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &mut [c64],
    ldt: i32,
    vl: &mut c64,
    ldvl: i32,
    vr: &mut c64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_ztrevc(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        t.as_mut_ptr() as *mut _,
        ldt,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
    )
}

#[inline]
pub unsafe fn strexc(
    layout: Layout,
    compq: u8,
    n: i32,
    t: &mut [f32],
    ldt: i32,
    q: &mut f32,
    ldq: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
) -> i32 {
    ffi::LAPACKE_strexc(
        layout.into(),
        compq as c_char,
        n,
        t.as_mut_ptr(),
        ldt,
        q,
        ldq,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtrexc(
    layout: Layout,
    compq: u8,
    n: i32,
    t: &mut [f64],
    ldt: i32,
    q: &mut f64,
    ldq: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtrexc(
        layout.into(),
        compq as c_char,
        n,
        t.as_mut_ptr(),
        ldt,
        q,
        ldq,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctrexc(
    layout: Layout,
    compq: u8,
    n: i32,
    t: &mut [c32],
    ldt: i32,
    q: &mut c32,
    ldq: i32,
    ifst: i32,
    ilst: i32,
) -> i32 {
    ffi::LAPACKE_ctrexc(
        layout.into(),
        compq as c_char,
        n,
        t.as_mut_ptr() as *mut _,
        ldt,
        q as *mut _ as *mut _,
        ldq,
        ifst,
        ilst,
    )
}

#[inline]
pub unsafe fn ztrexc(
    layout: Layout,
    compq: u8,
    n: i32,
    t: &mut [c64],
    ldt: i32,
    q: &mut c64,
    ldq: i32,
    ifst: i32,
    ilst: i32,
) -> i32 {
    ffi::LAPACKE_ztrexc(
        layout.into(),
        compq as c_char,
        n,
        t.as_mut_ptr() as *mut _,
        ldt,
        q as *mut _ as *mut _,
        ldq,
        ifst,
        ilst,
    )
}

#[inline]
pub unsafe fn strrfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    x: &[f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_strrfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtrrfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    x: &[f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtrrfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctrrfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    x: &[c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctrrfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztrrfs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    x: &[c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztrrfs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn strsen(
    layout: Layout,
    job: u8,
    compq: u8,
    select: &[i32],
    n: i32,
    t: &mut [f32],
    ldt: i32,
    q: &mut f32,
    ldq: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    m: &mut i32,
    s: &mut [f32],
    sep: &mut [f32],
) -> i32 {
    ffi::LAPACKE_strsen(
        layout.into(),
        job as c_char,
        compq as c_char,
        select.as_ptr(),
        n,
        t.as_mut_ptr(),
        ldt,
        q,
        ldq,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        m,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtrsen(
    layout: Layout,
    job: u8,
    compq: u8,
    select: &[i32],
    n: i32,
    t: &mut [f64],
    ldt: i32,
    q: &mut f64,
    ldq: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    m: &mut i32,
    s: &mut [f64],
    sep: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtrsen(
        layout.into(),
        job as c_char,
        compq as c_char,
        select.as_ptr(),
        n,
        t.as_mut_ptr(),
        ldt,
        q,
        ldq,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        m,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctrsen(
    layout: Layout,
    job: u8,
    compq: u8,
    select: &[i32],
    n: i32,
    t: &mut [c32],
    ldt: i32,
    q: &mut c32,
    ldq: i32,
    w: &mut [c32],
    m: &mut i32,
    s: &mut [f32],
    sep: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctrsen(
        layout.into(),
        job as c_char,
        compq as c_char,
        select.as_ptr(),
        n,
        t.as_mut_ptr() as *mut _,
        ldt,
        q as *mut _ as *mut _,
        ldq,
        w.as_mut_ptr() as *mut _,
        m,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztrsen(
    layout: Layout,
    job: u8,
    compq: u8,
    select: &[i32],
    n: i32,
    t: &mut [c64],
    ldt: i32,
    q: &mut c64,
    ldq: i32,
    w: &mut [c64],
    m: &mut i32,
    s: &mut [f64],
    sep: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztrsen(
        layout.into(),
        job as c_char,
        compq as c_char,
        select.as_ptr(),
        n,
        t.as_mut_ptr() as *mut _,
        ldt,
        q as *mut _ as *mut _,
        ldq,
        w.as_mut_ptr() as *mut _,
        m,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn strsna(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &[f32],
    ldt: i32,
    vl: &[f32],
    ldvl: i32,
    vr: &[f32],
    ldvr: i32,
    s: &mut [f32],
    sep: &mut [f32],
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_strsna(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        t.as_ptr(),
        ldt,
        vl.as_ptr(),
        ldvl,
        vr.as_ptr(),
        ldvr,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        mm,
        m,
    )
}

#[inline]
pub unsafe fn dtrsna(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &[f64],
    ldt: i32,
    vl: &[f64],
    ldvl: i32,
    vr: &[f64],
    ldvr: i32,
    s: &mut [f64],
    sep: &mut [f64],
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_dtrsna(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        t.as_ptr(),
        ldt,
        vl.as_ptr(),
        ldvl,
        vr.as_ptr(),
        ldvr,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        mm,
        m,
    )
}

#[inline]
pub unsafe fn ctrsna(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &[c32],
    ldt: i32,
    vl: &[c32],
    ldvl: i32,
    vr: &[c32],
    ldvr: i32,
    s: &mut [f32],
    sep: &mut [f32],
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_ctrsna(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        t.as_ptr() as *const _,
        ldt,
        vl.as_ptr() as *const _,
        ldvl,
        vr.as_ptr() as *const _,
        ldvr,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        mm,
        m,
    )
}

#[inline]
pub unsafe fn ztrsna(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &[c64],
    ldt: i32,
    vl: &[c64],
    ldvl: i32,
    vr: &[c64],
    ldvr: i32,
    s: &mut [f64],
    sep: &mut [f64],
    mm: i32,
    m: &mut i32,
) -> i32 {
    ffi::LAPACKE_ztrsna(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        t.as_ptr() as *const _,
        ldt,
        vl.as_ptr() as *const _,
        ldvl,
        vr.as_ptr() as *const _,
        ldvr,
        s.as_mut_ptr(),
        sep.as_mut_ptr(),
        mm,
        m,
    )
}

#[inline]
pub unsafe fn strsyl(
    layout: Layout,
    trana: u8,
    tranb: u8,
    isgn: i32,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    c: &mut [f32],
    ldc: i32,
    scale: &mut [f32],
) -> i32 {
    ffi::LAPACKE_strsyl(
        layout.into(),
        trana as c_char,
        tranb as c_char,
        isgn,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        c.as_mut_ptr(),
        ldc,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtrsyl(
    layout: Layout,
    trana: u8,
    tranb: u8,
    isgn: i32,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    c: &mut [f64],
    ldc: i32,
    scale: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtrsyl(
        layout.into(),
        trana as c_char,
        tranb as c_char,
        isgn,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        c.as_mut_ptr(),
        ldc,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctrsyl(
    layout: Layout,
    trana: u8,
    tranb: u8,
    isgn: i32,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    c: &mut [c32],
    ldc: i32,
    scale: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctrsyl(
        layout.into(),
        trana as c_char,
        tranb as c_char,
        isgn,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        c.as_mut_ptr() as *mut _,
        ldc,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztrsyl(
    layout: Layout,
    trana: u8,
    tranb: u8,
    isgn: i32,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    c: &mut [c64],
    ldc: i32,
    scale: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztrsyl(
        layout.into(),
        trana as c_char,
        tranb as c_char,
        isgn,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        c.as_mut_ptr() as *mut _,
        ldc,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn strtri(layout: Layout, uplo: u8, diag: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_strtri(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dtrtri(layout: Layout, uplo: u8, diag: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dtrtri(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn ctrtri(layout: Layout, uplo: u8, diag: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_ctrtri(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn ztrtri(layout: Layout, uplo: u8, diag: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_ztrtri(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn strtrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_strtrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dtrtrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dtrtrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ctrtrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ctrtrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ztrtrs(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ztrtrs(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn strttf(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    arf: &mut [f32],
) -> i32 {
    ffi::LAPACKE_strttf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        arf.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtrttf(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    arf: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtrttf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        arf.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctrttf(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    arf: &mut [c32],
) -> i32 {
    ffi::LAPACKE_ctrttf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        arf.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztrttf(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    arf: &mut [c64],
) -> i32 {
    ffi::LAPACKE_ztrttf(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        arf.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn strttp(layout: Layout, uplo: u8, n: i32, a: &[f32], lda: i32, ap: &mut [f32]) -> i32 {
    ffi::LAPACKE_strttp(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtrttp(layout: Layout, uplo: u8, n: i32, a: &[f64], lda: i32, ap: &mut [f64]) -> i32 {
    ffi::LAPACKE_dtrttp(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctrttp(layout: Layout, uplo: u8, n: i32, a: &[c32], lda: i32, ap: &mut [c32]) -> i32 {
    ffi::LAPACKE_ctrttp(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztrttp(layout: Layout, uplo: u8, n: i32, a: &[c64], lda: i32, ap: &mut [c64]) -> i32 {
    ffi::LAPACKE_ztrttp(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stzrzf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stzrzf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dtzrzf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtzrzf(layout.into(), m, n, a.as_mut_ptr(), lda, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn ctzrzf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_ctzrzf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztzrzf(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_ztzrzf(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn cungbr(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
) -> i32 {
    ffi::LAPACKE_cungbr(
        layout.into(),
        vect as c_char,
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zungbr(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
) -> i32 {
    ffi::LAPACKE_zungbr(
        layout.into(),
        vect as c_char,
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn cunghr(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
) -> i32 {
    ffi::LAPACKE_cunghr(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zunghr(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
) -> i32 {
    ffi::LAPACKE_zunghr(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn cunglq(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
) -> i32 {
    ffi::LAPACKE_cunglq(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zunglq(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
) -> i32 {
    ffi::LAPACKE_zunglq(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn cungql(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
) -> i32 {
    ffi::LAPACKE_cungql(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zungql(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
) -> i32 {
    ffi::LAPACKE_zungql(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn cungqr(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
) -> i32 {
    ffi::LAPACKE_cungqr(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zungqr(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
) -> i32 {
    ffi::LAPACKE_zungqr(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn cungrq(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
) -> i32 {
    ffi::LAPACKE_cungrq(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zungrq(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
) -> i32 {
    ffi::LAPACKE_zungrq(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn cungtr(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &[c32],
) -> i32 {
    ffi::LAPACKE_cungtr(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zungtr(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &[c64],
) -> i32 {
    ffi::LAPACKE_zungtr(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn cunmbr(
    layout: Layout,
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cunmbr(
        layout.into(),
        vect as c_char,
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zunmbr(
    layout: Layout,
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zunmbr(
        layout.into(),
        vect as c_char,
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn cunmhr(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cunmhr(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        ilo,
        ihi,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zunmhr(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zunmhr(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        ilo,
        ihi,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn cunmlq(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cunmlq(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zunmlq(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zunmlq(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn cunmql(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cunmql(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zunmql(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zunmql(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn cunmqr(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cunmqr(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zunmqr(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zunmqr(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn cunmrq(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cunmrq(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zunmrq(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zunmrq(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn cunmrz(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cunmrz(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        l,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zunmrz(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zunmrz(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        l,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn cunmtr(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cunmtr(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zunmtr(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zunmtr(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn cupgtr(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    tau: &[c32],
    q: &mut c32,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_cupgtr(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        q as *mut _ as *mut _,
        ldq,
    )
}

#[inline]
pub unsafe fn zupgtr(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    tau: &[c64],
    q: &mut c64,
    ldq: i32,
) -> i32 {
    ffi::LAPACKE_zupgtr(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        q as *mut _ as *mut _,
        ldq,
    )
}

#[inline]
pub unsafe fn cupmtr(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[c32],
    tau: &[c32],
    c: &mut [c32],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_cupmtr(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        ap.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn zupmtr(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[c64],
    tau: &[c64],
    c: &mut [c64],
    ldc: i32,
) -> i32 {
    ffi::LAPACKE_zupmtr(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        ap.as_ptr() as *const _,
        tau.as_ptr() as *const _,
        c.as_mut_ptr() as *mut _,
        ldc,
    )
}

#[inline]
pub unsafe fn sbdsdc_work(
    layout: Layout,
    uplo: u8,
    compq: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    q: &mut f32,
    iq: &mut [i32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sbdsdc_work(
        layout.into(),
        uplo as c_char,
        compq as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        q,
        iq.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dbdsdc_work(
    layout: Layout,
    uplo: u8,
    compq: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    q: &mut f64,
    iq: &mut [i32],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dbdsdc_work(
        layout.into(),
        uplo as c_char,
        compq as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        q,
        iq.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sbdsvdx_work(
    layout: Layout,
    uplo: u8,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sbdsvdx_work(
        layout.into(),
        uplo as c_char,
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dbdsvdx_work(
    layout: Layout,
    uplo: u8,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dbdsvdx_work(
        layout.into(),
        uplo as c_char,
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sbdsqr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ncvt: i32,
    nru: i32,
    ncc: i32,
    d: &mut [f32],
    e: &mut [f32],
    vt: &mut [f32],
    ldvt: i32,
    u: &mut [f32],
    ldu: i32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sbdsqr_work(
        layout.into(),
        uplo as c_char,
        n,
        ncvt,
        nru,
        ncc,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr(),
        ldvt,
        u.as_mut_ptr(),
        ldu,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dbdsqr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ncvt: i32,
    nru: i32,
    ncc: i32,
    d: &mut [f64],
    e: &mut [f64],
    vt: &mut [f64],
    ldvt: i32,
    u: &mut [f64],
    ldu: i32,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dbdsqr_work(
        layout.into(),
        uplo as c_char,
        n,
        ncvt,
        nru,
        ncc,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr(),
        ldvt,
        u.as_mut_ptr(),
        ldu,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cbdsqr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ncvt: i32,
    nru: i32,
    ncc: i32,
    d: &mut [f32],
    e: &mut [f32],
    vt: &mut [c32],
    ldvt: i32,
    u: &mut [c32],
    ldu: i32,
    c: &mut [c32],
    ldc: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cbdsqr_work(
        layout.into(),
        uplo as c_char,
        n,
        ncvt,
        nru,
        ncc,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr() as *mut _,
        ldvt,
        u.as_mut_ptr() as *mut _,
        ldu,
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zbdsqr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ncvt: i32,
    nru: i32,
    ncc: i32,
    d: &mut [f64],
    e: &mut [f64],
    vt: &mut [c64],
    ldvt: i32,
    u: &mut [c64],
    ldu: i32,
    c: &mut [c64],
    ldc: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zbdsqr_work(
        layout.into(),
        uplo as c_char,
        n,
        ncvt,
        nru,
        ncc,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vt.as_mut_ptr() as *mut _,
        ldvt,
        u.as_mut_ptr() as *mut _,
        ldu,
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sdisna_work(job: u8, m: i32, n: i32, d: &[f32], sep: &mut [f32]) -> i32 {
    ffi::LAPACKE_sdisna_work(job as c_char, m, n, d.as_ptr(), sep.as_mut_ptr())
}

#[inline]
pub unsafe fn ddisna_work(job: u8, m: i32, n: i32, d: &[f64], sep: &mut [f64]) -> i32 {
    ffi::LAPACKE_ddisna_work(job as c_char, m, n, d.as_ptr(), sep.as_mut_ptr())
}

#[inline]
pub unsafe fn sgbbrd_work(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    ncc: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut f32,
    ldq: i32,
    pt: &mut [f32],
    ldpt: i32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgbbrd_work(
        layout.into(),
        vect as c_char,
        m,
        n,
        ncc,
        kl,
        ku,
        ab.as_mut_ptr(),
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q,
        ldq,
        pt.as_mut_ptr(),
        ldpt,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbbrd_work(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    ncc: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut f64,
    ldq: i32,
    pt: &mut [f64],
    ldpt: i32,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgbbrd_work(
        layout.into(),
        vect as c_char,
        m,
        n,
        ncc,
        kl,
        ku,
        ab.as_mut_ptr(),
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q,
        ldq,
        pt.as_mut_ptr(),
        ldpt,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbbrd_work(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    ncc: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut c32,
    ldq: i32,
    pt: &mut [c32],
    ldpt: i32,
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbbrd_work(
        layout.into(),
        vect as c_char,
        m,
        n,
        ncc,
        kl,
        ku,
        ab.as_mut_ptr() as *mut _,
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q as *mut _ as *mut _,
        ldq,
        pt.as_mut_ptr() as *mut _,
        ldpt,
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbbrd_work(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    ncc: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut c64,
    ldq: i32,
    pt: &mut [c64],
    ldpt: i32,
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbbrd_work(
        layout.into(),
        vect as c_char,
        m,
        n,
        ncc,
        kl,
        ku,
        ab.as_mut_ptr() as *mut _,
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q as *mut _ as *mut _,
        ldq,
        pt.as_mut_ptr() as *mut _,
        ldpt,
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbcon_work(
    layout: Layout,
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgbcon_work(
        layout.into(),
        norm as c_char,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbcon_work(
    layout: Layout,
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgbcon_work(
        layout.into(),
        norm as c_char,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbcon_work(
    layout: Layout,
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbcon_work(
        layout.into(),
        norm as c_char,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbcon_work(
    layout: Layout,
    norm: u8,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbcon_work(
        layout.into(),
        norm as c_char,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbequ_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgbequ_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn dgbequ_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgbequ_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn cgbequ_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgbequ_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn zgbequ_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgbequ_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn sgbequb_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgbequb_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn dgbequb_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[f64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgbequb_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr(),
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn cgbequb_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c32],
    ldab: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgbequb_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn zgbequb_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &[c64],
    ldab: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgbequb_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_ptr() as *const _,
        ldab,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn sgbrfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgbrfs_work(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbrfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgbrfs_work(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbrfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbrfs_work(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbrfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbrfs_work(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbrfsx_work(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgbrfsx_work(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbrfsx_work(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgbrfsx_work(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbrfsx_work(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbrfsx_work(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbrfsx_work(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbrfsx_work(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbsv_work(
    layout: Layout,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgbsv_work(
        layout.into(),
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgbsv_work(
    layout: Layout,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgbsv_work(
        layout.into(),
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgbsv_work(
    layout: Layout,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgbsv_work(
        layout.into(),
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgbsv_work(
    layout: Layout,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgbsv_work(
        layout.into(),
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sgbsvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgbsvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbsvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgbsvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbsvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbsvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbsvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbsvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbsvxx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgbsvxx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbsvxx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgbsvxx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbsvxx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgbsvxx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbsvxx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgbsvxx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbtrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f32],
    ldab: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgbtrf_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_mut_ptr(),
        ldab,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgbtrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [f64],
    ldab: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgbtrf_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_mut_ptr(),
        ldab,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgbtrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c32],
    ldab: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgbtrf_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_mut_ptr() as *mut _,
        ldab,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgbtrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    ab: &mut [c64],
    ldab: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgbtrf_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        ab.as_mut_ptr() as *mut _,
        ldab,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgbtrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgbtrs_work(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgbtrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgbtrs_work(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr(),
        ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgbtrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgbtrs_work(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgbtrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    kl: i32,
    ku: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgbtrs_work(
        layout.into(),
        trans as c_char,
        n,
        kl,
        ku,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sgebak_work(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f32],
    m: i32,
    v: &mut [f32],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_sgebak_work(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        scale.as_ptr(),
        m,
        v.as_mut_ptr(),
        ldv,
    )
}

#[inline]
pub unsafe fn dgebak_work(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f64],
    m: i32,
    v: &mut [f64],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_dgebak_work(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        scale.as_ptr(),
        m,
        v.as_mut_ptr(),
        ldv,
    )
}

#[inline]
pub unsafe fn cgebak_work(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f32],
    m: i32,
    v: &mut [c32],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_cgebak_work(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        scale.as_ptr(),
        m,
        v.as_mut_ptr() as *mut _,
        ldv,
    )
}

#[inline]
pub unsafe fn zgebak_work(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    scale: &[f64],
    m: i32,
    v: &mut [c64],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_zgebak_work(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        scale.as_ptr(),
        m,
        v.as_mut_ptr() as *mut _,
        ldv,
    )
}

#[inline]
pub unsafe fn sgebal_work(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgebal_work(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgebal_work(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgebal_work(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgebal_work(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgebal_work(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgebal_work(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgebal_work(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ilo,
        ihi,
        scale.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgebrd_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tauq: &mut [f32],
    taup: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgebrd_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr(),
        taup.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgebrd_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [f64],
    taup: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgebrd_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr(),
        taup.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgebrd_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tauq: &mut [c32],
    taup: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgebrd_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr() as *mut _,
        taup.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgebrd_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tauq: &mut [c64],
    taup: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgebrd_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tauq.as_mut_ptr() as *mut _,
        taup.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgecon_work(
    layout: Layout,
    norm: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgecon_work(
        layout.into(),
        norm as c_char,
        n,
        a.as_ptr(),
        lda,
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgecon_work(
    layout: Layout,
    norm: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgecon_work(
        layout.into(),
        norm as c_char,
        n,
        a.as_ptr(),
        lda,
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgecon_work(
    layout: Layout,
    norm: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgecon_work(
        layout.into(),
        norm as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgecon_work(
    layout: Layout,
    norm: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgecon_work(
        layout.into(),
        norm as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgeequ_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgeequ_work(
        layout.into(),
        m,
        n,
        a.as_ptr(),
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn dgeequ_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgeequ_work(
        layout.into(),
        m,
        n,
        a.as_ptr(),
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn cgeequ_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgeequ_work(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn zgeequ_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgeequ_work(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn sgeequb_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sgeequb_work(
        layout.into(),
        m,
        n,
        a.as_ptr(),
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn dgeequb_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dgeequb_work(
        layout.into(),
        m,
        n,
        a.as_ptr(),
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn cgeequb_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    r: &mut [f32],
    c: &mut [f32],
    rowcnd: &mut f32,
    colcnd: &mut f32,
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cgeequb_work(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn zgeequb_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    r: &mut [f64],
    c: &mut [f64],
    rowcnd: &mut f64,
    colcnd: &mut f64,
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zgeequb_work(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        rowcnd,
        colcnd,
        amax,
    )
}

#[inline]
pub unsafe fn sgees_work(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select2F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vs: &mut [f32],
    ldvs: i32,
    work: &mut [f32],
    lwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgees_work(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        n,
        a.as_mut_ptr(),
        lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        ldvs,
        work.as_mut_ptr(),
        lwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgees_work(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select2F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vs: &mut [f64],
    ldvs: i32,
    work: &mut [f64],
    lwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgees_work(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        n,
        a.as_mut_ptr(),
        lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        ldvs,
        work.as_mut_ptr(),
        lwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgees_work(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select1C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c32],
    vs: &mut [c32],
    ldvs: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgees_work(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        ldvs,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgees_work(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select1C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c64],
    vs: &mut [c64],
    ldvs: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgees_work(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        ldvs,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgeesx_work(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select2F32,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vs: &mut [f32],
    ldvs: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgeesx_work(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgeesx_work(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select2F64,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sdim: &mut i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vs: &mut [f64],
    ldvs: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgeesx_work(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        sdim,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vs.as_mut_ptr(),
        ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgeesx_work(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select1C32,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c32],
    vs: &mut [c32],
    ldvs: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgeesx_work(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgeesx_work(
    layout: Layout,
    jobvs: u8,
    sort: u8,
    select: Select1C64,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sdim: &mut i32,
    w: &mut [c64],
    vs: &mut [c64],
    ldvs: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgeesx_work(
        layout.into(),
        jobvs as c_char,
        sort as c_char,
        transmute(select),
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sdim,
        w.as_mut_ptr() as *mut _,
        vs.as_mut_ptr() as *mut _,
        ldvs,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgeev_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgeev_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgeev_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgeev_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgeev_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgeev_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgeev_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgeev_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgeevx_work(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
    abnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgeevx_work(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgeevx_work(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
    abnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgeevx_work(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgeevx_work(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f32],
    abnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgeevx_work(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgeevx_work(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    scale: &mut [f64],
    abnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgeevx_work(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        ilo,
        ihi,
        scale.as_mut_ptr(),
        abnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgehrd_work(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgehrd_work(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgehrd_work(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgehrd_work(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgehrd_work(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgehrd_work(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgehrd_work(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgehrd_work(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgejsv_work(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sva: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgejsv_work(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        jobr as c_char,
        jobt as c_char,
        jobp as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgejsv_work(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sva: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgejsv_work(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        jobr as c_char,
        jobt as c_char,
        jobp as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgejsv_work(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sva: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    cwork: &mut [c32],
    lwork: i32,
    work: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgejsv_work(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        jobr as c_char,
        jobt as c_char,
        jobp as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        cwork.as_mut_ptr() as *mut _,
        lwork,
        work.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgejsv_work(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    jobr: u8,
    jobt: u8,
    jobp: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sva: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    cwork: &mut [c64],
    lwork: i32,
    work: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgejsv_work(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        jobr as c_char,
        jobt as c_char,
        jobp as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sva.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        cwork.as_mut_ptr() as *mut _,
        lwork,
        work.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgelq2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgelq2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgelq2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgelq2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgelq2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgelq2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgelq2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgelq2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgelqf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgelqf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgelqf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgelqf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgelqf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgelqf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgelqf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgelqf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgels_work(
    layout: Layout,
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgels_work(
        layout.into(),
        trans as c_char,
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgels_work(
    layout: Layout,
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgels_work(
        layout.into(),
        trans as c_char,
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgels_work(
    layout: Layout,
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgels_work(
        layout.into(),
        trans as c_char,
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgels_work(
    layout: Layout,
    trans: u8,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgels_work(
        layout.into(),
        trans as c_char,
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgelsd_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgelsd_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgelsd_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgelsd_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgelsd_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgelsd_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgelsd_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgelsd_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgelss_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgelss_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgelss_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgelss_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgelss_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    s: &mut [f32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgelss_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgelss_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    s: &mut [f64],
    rcond: f64,
    rank: &mut i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgelss_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        s.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgelsy_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgelsy_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        jpvt.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgelsy_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f64,
    rank: &mut i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgelsy_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        jpvt.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgelsy_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f32,
    rank: &mut i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgelsy_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        jpvt.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgelsy_work(
    layout: Layout,
    m: i32,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    jpvt: &mut [i32],
    rcond: f64,
    rank: &mut i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgelsy_work(
        layout.into(),
        m,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        jpvt.as_mut_ptr(),
        rcond,
        rank,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgeqlf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgeqlf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgeqlf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgeqlf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgeqlf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgeqlf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgeqlf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgeqlf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgeqp3_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgeqp3_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgeqp3_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgeqp3_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgeqp3_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgeqp3_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgeqp3_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgeqp3_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgeqpf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeqpf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgeqpf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [f64],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeqpf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgeqpf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgeqpf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgeqpf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    jpvt: &mut [i32],
    tau: &mut [c64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgeqpf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        jpvt.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgeqr2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sgeqr2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgeqr2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dgeqr2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgeqr2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgeqr2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgeqr2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgeqr2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgeqrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgeqrf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgeqrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgeqrf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgeqrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgeqrf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgeqrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgeqrf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgeqrfp_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgeqrfp_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgeqrfp_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgeqrfp_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgeqrfp_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgeqrfp_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgeqrfp_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgeqrfp_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgerfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgerfs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgerfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgerfs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgerfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgerfs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgerfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgerfs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgerfsx_work(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgerfsx_work(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgerfsx_work(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgerfsx_work(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgerfsx_work(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f32],
    c: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgerfsx_work(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgerfsx_work(
    layout: Layout,
    trans: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    r: &[f64],
    c: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgerfsx_work(
        layout.into(),
        trans as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        r.as_ptr(),
        c.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgerqf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgerqf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgerqf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgerqf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgerqf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgerqf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgerqf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgerqf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgesdd_work(
    layout: Layout,
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgesdd_work(
        layout.into(),
        jobz as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesdd_work(
    layout: Layout,
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgesdd_work(
        layout.into(),
        jobz as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesdd_work(
    layout: Layout,
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgesdd_work(
        layout.into(),
        jobz as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesdd_work(
    layout: Layout,
    jobz: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgesdd_work(
        layout.into(),
        jobz as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgesv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgesv_work(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgesv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgesv_work(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgesv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgesv_work(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgesv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgesv_work(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn dsgesv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    work: &mut [f64],
    swork: &mut [f32],
    iter: &mut i32,
) -> i32 {
    ffi::LAPACKE_dsgesv_work(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        work.as_mut_ptr(),
        swork.as_mut_ptr(),
        iter,
    )
}

#[inline]
pub unsafe fn zcgesv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    work: &mut [c64],
    swork: &mut [c32],
    rwork: &mut [f64],
    iter: &mut i32,
) -> i32 {
    ffi::LAPACKE_zcgesv_work(
        layout.into(),
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        work.as_mut_ptr() as *mut _,
        swork.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iter,
    )
}

#[inline]
pub unsafe fn sgesvd_work(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgesvd_work(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgesvd_work(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgesvd_work(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgesvd_work(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgesvd_work(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvd_work(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgesvd_work(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgesvdx_work(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    vt: &mut [f32],
    ldvt: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgesvdx_work(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        range as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesvdx_work(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    vt: &mut [f64],
    ldvt: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgesvdx_work(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        range as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        vt.as_mut_ptr(),
        ldvt,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesvdx_work(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    vt: &mut [c32],
    ldvt: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgesvdx_work(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        range as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvdx_work(
    layout: Layout,
    jobu: u8,
    jobvt: u8,
    range: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    ns: i32,
    s: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    vt: &mut [c64],
    ldvt: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgesvdx_work(
        layout.into(),
        jobu as c_char,
        jobvt as c_char,
        range as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        ns,
        s.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        vt.as_mut_ptr() as *mut _,
        ldvt,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgesvj_work(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    sva: &mut [f32],
    mv: i32,
    v: &mut [f32],
    ldv: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgesvj_work(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        sva.as_mut_ptr(),
        mv,
        v.as_mut_ptr(),
        ldv,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgesvj_work(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    sva: &mut [f64],
    mv: i32,
    v: &mut [f64],
    ldv: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgesvj_work(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        m,
        n,
        a.as_mut_ptr(),
        lda,
        sva.as_mut_ptr(),
        mv,
        v.as_mut_ptr(),
        ldv,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgesvj_work(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    sva: &mut [f32],
    mv: i32,
    v: &mut [c32],
    ldv: i32,
    cwork: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
) -> i32 {
    ffi::LAPACKE_cgesvj_work(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sva.as_mut_ptr(),
        mv,
        v.as_mut_ptr() as *mut _,
        ldv,
        cwork.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
    )
}

#[inline]
pub unsafe fn zgesvj_work(
    layout: Layout,
    joba: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    sva: &mut [f64],
    mv: i32,
    v: &mut [c64],
    ldv: i32,
    cwork: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
) -> i32 {
    ffi::LAPACKE_zgesvj_work(
        layout.into(),
        joba as c_char,
        jobu as c_char,
        jobv as c_char,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        sva.as_mut_ptr(),
        mv,
        v.as_mut_ptr() as *mut _,
        ldv,
        cwork.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
    )
}

#[inline]
pub unsafe fn sgesvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgesvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgesvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgesvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgesvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgesvxx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgesvxx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgesvxx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgesvxx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgesvxx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f32],
    c: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgesvxx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgesvxx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    r: &mut [f64],
    c: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgesvxx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        r.as_mut_ptr(),
        c.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgetf2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgetf2_work(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn dgetf2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgetf2_work(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn cgetf2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgetf2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgetf2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgetf2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgetrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgetrf_work(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn dgetrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgetrf_work(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn cgetrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgetrf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgetrf_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgetrf_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgetrf2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgetrf2_work(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn dgetrf2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgetrf2_work(layout.into(), m, n, a.as_mut_ptr(), lda, ipiv.as_mut_ptr())
}

#[inline]
pub unsafe fn cgetrf2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgetrf2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgetrf2_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgetrf2_work(
        layout.into(),
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgetri_work(
    layout: Layout,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgetri_work(
        layout.into(),
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgetri_work(
    layout: Layout,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgetri_work(
        layout.into(),
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgetri_work(
    layout: Layout,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgetri_work(
        layout.into(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgetri_work(
    layout: Layout,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgetri_work(
        layout.into(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgetrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgetrs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgetrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgetrs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgetrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgetrs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgetrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgetrs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sggbak_work(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f32],
    rscale: &[f32],
    m: i32,
    v: &mut [f32],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_sggbak_work(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        m,
        v.as_mut_ptr(),
        ldv,
    )
}

#[inline]
pub unsafe fn dggbak_work(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f64],
    rscale: &[f64],
    m: i32,
    v: &mut [f64],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_dggbak_work(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        m,
        v.as_mut_ptr(),
        ldv,
    )
}

#[inline]
pub unsafe fn cggbak_work(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f32],
    rscale: &[f32],
    m: i32,
    v: &mut [c32],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_cggbak_work(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        m,
        v.as_mut_ptr() as *mut _,
        ldv,
    )
}

#[inline]
pub unsafe fn zggbak_work(
    layout: Layout,
    job: u8,
    side: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    lscale: &[f64],
    rscale: &[f64],
    m: i32,
    v: &mut [c64],
    ldv: i32,
) -> i32 {
    ffi::LAPACKE_zggbak_work(
        layout.into(),
        job as c_char,
        side as c_char,
        n,
        ilo,
        ihi,
        lscale.as_ptr(),
        rscale.as_ptr(),
        m,
        v.as_mut_ptr() as *mut _,
        ldv,
    )
}

#[inline]
pub unsafe fn sggbal_work(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sggbal_work(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggbal_work(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dggbal_work(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggbal_work(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cggbal_work(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggbal_work(
    layout: Layout,
    job: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zggbal_work(
        layout.into(),
        job as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgges_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
    work: &mut [f32],
    lwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgges_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
        work.as_mut_ptr(),
        lwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgges_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
    work: &mut [f64],
    lwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgges_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
        work.as_mut_ptr(),
        lwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgges_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgges_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgges_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgges_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgges3_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
    work: &mut [f32],
    lwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgges3_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
        work.as_mut_ptr(),
        lwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgges3_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
    work: &mut [f64],
    lwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgges3_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
        work.as_mut_ptr(),
        lwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgges3_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgges3_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgges3_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgges3_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggesx_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F32,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vsl: &mut [f32],
    ldvsl: i32,
    vsr: &mut [f32],
    ldvsr: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sggesx_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggesx_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select3F64,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    sdim: &mut i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vsl: &mut [f64],
    ldvsl: i32,
    vsr: &mut [f64],
    ldvsr: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dggesx_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        sdim,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vsl.as_mut_ptr(),
        ldvsl,
        vsr.as_mut_ptr(),
        ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggesx_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C32,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vsl: &mut [c32],
    ldvsl: i32,
    vsr: &mut [c32],
    ldvsr: i32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cggesx_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        liwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggesx_work(
    layout: Layout,
    jobvsl: u8,
    jobvsr: u8,
    sort: u8,
    selctg: Select2C64,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    sdim: &mut i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vsl: &mut [c64],
    ldvsl: i32,
    vsr: &mut [c64],
    ldvsr: i32,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    liwork: i32,
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zggesx_work(
        layout.into(),
        jobvsl as c_char,
        jobvsr as c_char,
        sort as c_char,
        transmute(selctg),
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        sdim,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vsl.as_mut_ptr() as *mut _,
        ldvsl,
        vsr.as_mut_ptr() as *mut _,
        ldvsr,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        liwork,
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggev_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sggev_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dggev_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dggev_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cggev_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cggev_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggev_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zggev_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggev3_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sggev3_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dggev3_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dggev3_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cggev3_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cggev3_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggev3_work(
    layout: Layout,
    jobvl: u8,
    jobvr: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zggev3_work(
        layout.into(),
        jobvl as c_char,
        jobvr as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggevx_work(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    vl: &mut [f32],
    ldvl: i32,
    vr: &mut [f32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    abnrm: &mut f32,
    bbnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sggevx_work(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggevx_work(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    vl: &mut [f64],
    ldvl: i32,
    vr: &mut [f64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    abnrm: &mut f64,
    bbnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dggevx_work(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        vl.as_mut_ptr(),
        ldvl,
        vr.as_mut_ptr(),
        ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggevx_work(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    vl: &mut [c32],
    ldvl: i32,
    vr: &mut [c32],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f32],
    rscale: &mut [f32],
    abnrm: &mut f32,
    bbnrm: &mut f32,
    rconde: &mut [f32],
    rcondv: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cggevx_work(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggevx_work(
    layout: Layout,
    balanc: u8,
    jobvl: u8,
    jobvr: u8,
    sense: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    vl: &mut [c64],
    ldvl: i32,
    vr: &mut [c64],
    ldvr: i32,
    ilo: &mut i32,
    ihi: &mut i32,
    lscale: &mut [f64],
    rscale: &mut [f64],
    abnrm: &mut f64,
    bbnrm: &mut f64,
    rconde: &mut [f64],
    rcondv: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    bwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zggevx_work(
        layout.into(),
        balanc as c_char,
        jobvl as c_char,
        jobvr as c_char,
        sense as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        vl.as_mut_ptr() as *mut _,
        ldvl,
        vr.as_mut_ptr() as *mut _,
        ldvr,
        ilo,
        ihi,
        lscale.as_mut_ptr(),
        rscale.as_mut_ptr(),
        abnrm,
        bbnrm,
        rconde.as_mut_ptr(),
        rcondv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        bwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggglm_work(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    d: &mut [f32],
    x: &mut [f32],
    y: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sggglm_work(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        y.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dggglm_work(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    d: &mut [f64],
    x: &mut [f64],
    y: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dggglm_work(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        y.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cggglm_work(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    d: &mut [c32],
    x: &mut [c32],
    y: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cggglm_work(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        y.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zggglm_work(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    d: &mut [c64],
    x: &mut [c64],
    y: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zggglm_work(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        y.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgghrd_work(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_sgghrd_work(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn dgghrd_work(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_dgghrd_work(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
    )
}

#[inline]
pub unsafe fn cgghrd_work(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_cgghrd_work(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn zgghrd_work(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
) -> i32 {
    ffi::LAPACKE_zgghrd_work(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
    )
}

#[inline]
pub unsafe fn sgghd3_work(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgghd3_work(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgghd3_work(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgghd3_work(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgghd3_work(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgghd3_work(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgghd3_work(
    layout: Layout,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgghd3_work(
        layout.into(),
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgglse_work(
    layout: Layout,
    m: i32,
    n: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    c: &mut [f32],
    d: &mut [f32],
    x: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sgglse_work(
        layout.into(),
        m,
        n,
        p,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        c.as_mut_ptr(),
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dgglse_work(
    layout: Layout,
    m: i32,
    n: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    c: &mut [f64],
    d: &mut [f64],
    x: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dgglse_work(
        layout.into(),
        m,
        n,
        p,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        c.as_mut_ptr(),
        d.as_mut_ptr(),
        x.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cgglse_work(
    layout: Layout,
    m: i32,
    n: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    c: &mut [c32],
    d: &mut [c32],
    x: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cgglse_work(
        layout.into(),
        m,
        n,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        c.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zgglse_work(
    layout: Layout,
    m: i32,
    n: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    c: &mut [c64],
    d: &mut [c64],
    x: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zgglse_work(
        layout.into(),
        m,
        n,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        c.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sggqrf_work(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f32],
    lda: i32,
    taua: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    taub: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sggqrf_work(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr(),
        lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        taub.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dggqrf_work(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [f64],
    lda: i32,
    taua: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    taub: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dggqrf_work(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr(),
        lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        taub.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cggqrf_work(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c32],
    lda: i32,
    taua: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    taub: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cggqrf_work(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
        taub.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zggqrf_work(
    layout: Layout,
    n: i32,
    m: i32,
    p: i32,
    a: &mut [c64],
    lda: i32,
    taua: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    taub: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zggqrf_work(
        layout.into(),
        n,
        m,
        p,
        a.as_mut_ptr() as *mut _,
        lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
        taub.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sggrqf_work(
    layout: Layout,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    taua: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    taub: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sggrqf_work(
        layout.into(),
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        taub.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dggrqf_work(
    layout: Layout,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    taua: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    taub: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dggrqf_work(
        layout.into(),
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        taua.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        taub.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cggrqf_work(
    layout: Layout,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    taua: &mut [c32],
    b: &mut [c32],
    ldb: i32,
    taub: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cggrqf_work(
        layout.into(),
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
        taub.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zggrqf_work(
    layout: Layout,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    taua: &mut [c64],
    b: &mut [c64],
    ldb: i32,
    taub: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zggrqf_work(
        layout.into(),
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        taua.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
        taub.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sggsvd_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sggsvd_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggsvd_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dggsvd_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggsvd_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cggsvd_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggsvd_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zggsvd_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggsvd3_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sggsvd3_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggsvd3_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dggsvd3_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggsvd3_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cggsvd3_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zggsvd3_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    n: i32,
    p: i32,
    k: &mut i32,
    l: &mut i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zggsvd3_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        n,
        p,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sggsvp_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
    iwork: &mut [i32],
    tau: &mut [f32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sggsvp_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        iwork.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dggsvp_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
    iwork: &mut [i32],
    tau: &mut [f64],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dggsvp_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        iwork.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cggsvp_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
    iwork: &mut [i32],
    rwork: &mut [f32],
    tau: &mut [c32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cggsvp_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        iwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zggsvp_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
    iwork: &mut [i32],
    rwork: &mut [f64],
    tau: &mut [c64],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zggsvp_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        iwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sggsvp3_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
    iwork: &mut [i32],
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sggsvp3_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        iwork.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dggsvp3_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
    iwork: &mut [i32],
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dggsvp3_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        iwork.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn cggsvp3_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
    iwork: &mut [i32],
    rwork: &mut [f32],
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_cggsvp3_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        iwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zggsvp3_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    k: &mut i32,
    l: &mut i32,
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
    iwork: &mut [i32],
    rwork: &mut [f64],
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zggsvp3_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        k,
        l,
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        iwork.as_mut_ptr(),
        rwork.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn sgtcon_work(
    norm: u8,
    n: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgtcon_work(
        norm as c_char,
        n,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgtcon_work(
    norm: u8,
    n: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgtcon_work(
        norm as c_char,
        n,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgtcon_work(
    norm: u8,
    n: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cgtcon_work(
        norm as c_char,
        n,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zgtcon_work(
    norm: u8,
    n: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zgtcon_work(
        norm as c_char,
        n,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sgtrfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    dlf: &[f32],
    df: &[f32],
    duf: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgtrfs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_ptr(),
        df.as_ptr(),
        duf.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgtrfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    dlf: &[f64],
    df: &[f64],
    duf: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgtrfs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_ptr(),
        df.as_ptr(),
        duf.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgtrfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    dlf: &[c32],
    df: &[c32],
    duf: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgtrfs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_ptr() as *const _,
        df.as_ptr() as *const _,
        duf.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgtrfs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    dlf: &[c64],
    df: &[c64],
    duf: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgtrfs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_ptr() as *const _,
        df.as_ptr() as *const _,
        duf.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgtsv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    dl: &mut [f32],
    d: &mut [f32],
    du: &mut [f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgtsv_work(
        layout.into(),
        n,
        nrhs,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgtsv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    dl: &mut [f64],
    d: &mut [f64],
    du: &mut [f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgtsv_work(
        layout.into(),
        n,
        nrhs,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgtsv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    dl: &mut [c32],
    d: &mut [c32],
    du: &mut [c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgtsv_work(
        layout.into(),
        n,
        nrhs,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgtsv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    dl: &mut [c64],
    d: &mut [c64],
    du: &mut [c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgtsv_work(
        layout.into(),
        n,
        nrhs,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sgtsvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    dlf: &mut [f32],
    df: &mut [f32],
    duf: &mut [f32],
    du2: &mut [f32],
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgtsvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_mut_ptr(),
        df.as_mut_ptr(),
        duf.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgtsvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    dlf: &mut [f64],
    df: &mut [f64],
    duf: &mut [f64],
    du2: &mut [f64],
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgtsvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        dlf.as_mut_ptr(),
        df.as_mut_ptr(),
        duf.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgtsvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    dlf: &mut [c32],
    df: &mut [c32],
    duf: &mut [c32],
    du2: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cgtsvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_mut_ptr() as *mut _,
        df.as_mut_ptr() as *mut _,
        duf.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgtsvx_work(
    layout: Layout,
    fact: u8,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    dlf: &mut [c64],
    df: &mut [c64],
    duf: &mut [c64],
    du2: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zgtsvx_work(
        layout.into(),
        fact as c_char,
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        dlf.as_mut_ptr() as *mut _,
        df.as_mut_ptr() as *mut _,
        duf.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgttrf_work(
    n: i32,
    dl: &mut [f32],
    d: &mut [f32],
    du: &mut [f32],
    du2: &mut [f32],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sgttrf_work(
        n,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dgttrf_work(
    n: i32,
    dl: &mut [f64],
    d: &mut [f64],
    du: &mut [f64],
    du2: &mut [f64],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dgttrf_work(
        n,
        dl.as_mut_ptr(),
        d.as_mut_ptr(),
        du.as_mut_ptr(),
        du2.as_mut_ptr(),
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cgttrf_work(
    n: i32,
    dl: &mut [c32],
    d: &mut [c32],
    du: &mut [c32],
    du2: &mut [c32],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cgttrf_work(
        n,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zgttrf_work(
    n: i32,
    dl: &mut [c64],
    d: &mut [c64],
    du: &mut [c64],
    du2: &mut [c64],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zgttrf_work(
        n,
        dl.as_mut_ptr() as *mut _,
        d.as_mut_ptr() as *mut _,
        du.as_mut_ptr() as *mut _,
        du2.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sgttrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f32],
    d: &[f32],
    du: &[f32],
    du2: &[f32],
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sgttrs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dgttrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[f64],
    d: &[f64],
    du: &[f64],
    du2: &[f64],
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dgttrs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr(),
        d.as_ptr(),
        du.as_ptr(),
        du2.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cgttrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c32],
    d: &[c32],
    du: &[c32],
    du2: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cgttrs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zgttrs_work(
    layout: Layout,
    trans: u8,
    n: i32,
    nrhs: i32,
    dl: &[c64],
    d: &[c64],
    du: &[c64],
    du2: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zgttrs_work(
        layout.into(),
        trans as c_char,
        n,
        nrhs,
        dl.as_ptr() as *const _,
        d.as_ptr() as *const _,
        du.as_ptr() as *const _,
        du2.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn chbev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chbev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhbev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhbev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chbevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_chbevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zhbevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zhbevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn chbevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    q: &mut c32,
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chbevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        q as *mut _ as *mut _,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhbevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    q: &mut c64,
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhbevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        q as *mut _ as *mut _,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chbgst_work(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &[c32],
    ldbb: i32,
    x: &mut [c32],
    ldx: i32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chbgst_work(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_ptr() as *const _,
        ldbb,
        x.as_mut_ptr() as *mut _,
        ldx,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhbgst_work(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &[c64],
    ldbb: i32,
    x: &mut [c64],
    ldx: i32,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhbgst_work(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_ptr() as *const _,
        ldbb,
        x.as_mut_ptr() as *mut _,
        ldx,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chbgv_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chbgv_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhbgv_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhbgv_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chbgvd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_chbgvd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zhbgvd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zhbgvd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn chbgvx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c32],
    ldab: i32,
    bb: &mut [c32],
    ldbb: i32,
    q: &mut c32,
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chbgvx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        q as *mut _ as *mut _,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhbgvx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [c64],
    ldab: i32,
    bb: &mut [c64],
    ldbb: i32,
    q: &mut c64,
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhbgvx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr() as *mut _,
        ldab,
        bb.as_mut_ptr() as *mut _,
        ldbb,
        q as *mut _ as *mut _,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chbtrd_work(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut c32,
    ldq: i32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chbtrd_work(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q as *mut _ as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhbtrd_work(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut c64,
    ldq: i32,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhbtrd_work(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q as *mut _ as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn checon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_checon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhecon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhecon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn cheequb_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cheequb_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zheequb_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zheequb_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn cheev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cheev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zheev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zheev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cheevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_cheevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zheevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zheevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn cheevr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_cheevr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zheevr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zheevr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn cheevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cheevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zheevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zheevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chegst_work(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chegst_work(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_ptr() as *const _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhegst_work(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhegst_work(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_ptr() as *const _,
        ldb,
    )
}

#[inline]
pub unsafe fn chegv_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chegv_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhegv_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhegv_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chegvd_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_chegvd_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zhegvd_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zhegvd_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn chegvx_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chegvx_work(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhegvx_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhegvx_work(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cherfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cherfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zherfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zherfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cherfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cherfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zherfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zherfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chesv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_chesv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zhesv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zhesv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn chesvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chesvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhesvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhesvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chesvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chesvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhesvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhesvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chetrd_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [c32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_chetrd_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zhetrd_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [c64],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zhetrd_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn chetrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_chetrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zhetrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zhetrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn chetri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chetri_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhetri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhetri_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn chetrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chetrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhetrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhetrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn chfrk_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: f32,
    a: &[c32],
    lda: i32,
    beta: f32,
    c: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chfrk_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        trans as c_char,
        n,
        k,
        alpha,
        a.as_ptr() as *const _,
        lda,
        beta,
        c.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhfrk_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: f64,
    a: &[c64],
    lda: i32,
    beta: f64,
    c: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhfrk_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        trans as c_char,
        n,
        k,
        alpha,
        a.as_ptr() as *const _,
        lda,
        beta,
        c.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn shgeqz_work(
    layout: Layout,
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f32],
    ldh: i32,
    t: &mut [f32],
    ldt: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_shgeqz_work(
        layout.into(),
        job as c_char,
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr(),
        ldh,
        t.as_mut_ptr(),
        ldt,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dhgeqz_work(
    layout: Layout,
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f64],
    ldh: i32,
    t: &mut [f64],
    ldt: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dhgeqz_work(
        layout.into(),
        job as c_char,
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr(),
        ldh,
        t.as_mut_ptr(),
        ldt,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn chgeqz_work(
    layout: Layout,
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c32],
    ldh: i32,
    t: &mut [c32],
    ldt: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chgeqz_work(
        layout.into(),
        job as c_char,
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr() as *mut _,
        ldh,
        t.as_mut_ptr() as *mut _,
        ldt,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhgeqz_work(
    layout: Layout,
    job: u8,
    compq: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c64],
    ldh: i32,
    t: &mut [c64],
    ldt: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhgeqz_work(
        layout.into(),
        job as c_char,
        compq as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr() as *mut _,
        ldh,
        t.as_mut_ptr() as *mut _,
        ldt,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chpcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chpcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhpcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhpcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn chpev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chpev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhpev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhpev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chpevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_chpevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zhpevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zhpevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn chpevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chpevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhpevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhpevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chpgst_work(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &[c32],
) -> i32 {
    ffi::LAPACKE_chpgst_work(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn zhpgst_work(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &[c64],
) -> i32 {
    ffi::LAPACKE_zhpgst_work(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_ptr() as *const _,
    )
}

#[inline]
pub unsafe fn chpgv_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chpgv_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhpgv_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhpgv_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chpgvd_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_chpgvd_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zhpgvd_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zhpgvd_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn chpgvx_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    bp: &mut [c32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    rwork: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chpgvx_work(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhpgvx_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    bp: &mut [c64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    rwork: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhpgvx_work(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        bp.as_mut_ptr() as *mut _,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chpsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chpsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhpsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhpsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn chpsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_chpsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhpsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zhpsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chptrd_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chptrd_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhptrd_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhptrd_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn chptrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chptrf_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhptrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhptrf_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chptri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    ipiv: &[i32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_chptri_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zhptri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    ipiv: &[i32],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zhptri_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn chptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_chptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zhptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zhptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn shsein_work(
    layout: Layout,
    job: u8,
    eigsrc: u8,
    initv: u8,
    select: &mut [i32],
    n: i32,
    h: &[f32],
    ldh: i32,
    wr: &mut [f32],
    wi: &[f32],
    vl: &mut f32,
    ldvl: i32,
    vr: &mut f32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
    ifaill: &mut [i32],
    ifailr: &mut [i32],
) -> i32 {
    ffi::LAPACKE_shsein_work(
        layout.into(),
        job as c_char,
        eigsrc as c_char,
        initv as c_char,
        select.as_mut_ptr(),
        n,
        h.as_ptr(),
        ldh,
        wr.as_mut_ptr(),
        wi.as_ptr(),
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
        work.as_mut_ptr(),
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dhsein_work(
    layout: Layout,
    job: u8,
    eigsrc: u8,
    initv: u8,
    select: &mut [i32],
    n: i32,
    h: &[f64],
    ldh: i32,
    wr: &mut [f64],
    wi: &[f64],
    vl: &mut f64,
    ldvl: i32,
    vr: &mut f64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
    ifaill: &mut [i32],
    ifailr: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dhsein_work(
        layout.into(),
        job as c_char,
        eigsrc as c_char,
        initv as c_char,
        select.as_mut_ptr(),
        n,
        h.as_ptr(),
        ldh,
        wr.as_mut_ptr(),
        wi.as_ptr(),
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
        work.as_mut_ptr(),
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn chsein_work(
    layout: Layout,
    job: u8,
    eigsrc: u8,
    initv: u8,
    select: &[i32],
    n: i32,
    h: &[c32],
    ldh: i32,
    w: &mut [c32],
    vl: &mut c32,
    ldvl: i32,
    vr: &mut c32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    rwork: &mut [f32],
    ifaill: &mut [i32],
    ifailr: &mut [i32],
) -> i32 {
    ffi::LAPACKE_chsein_work(
        layout.into(),
        job as c_char,
        eigsrc as c_char,
        initv as c_char,
        select.as_ptr(),
        n,
        h.as_ptr() as *const _,
        ldh,
        w.as_mut_ptr() as *mut _,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zhsein_work(
    layout: Layout,
    job: u8,
    eigsrc: u8,
    initv: u8,
    select: &[i32],
    n: i32,
    h: &[c64],
    ldh: i32,
    w: &mut [c64],
    vl: &mut c64,
    ldvl: i32,
    vr: &mut c64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    rwork: &mut [f64],
    ifaill: &mut [i32],
    ifailr: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zhsein_work(
        layout.into(),
        job as c_char,
        eigsrc as c_char,
        initv as c_char,
        select.as_ptr(),
        n,
        h.as_ptr() as *const _,
        ldh,
        w.as_mut_ptr() as *mut _,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        ifaill.as_mut_ptr(),
        ifailr.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn shseqr_work(
    layout: Layout,
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f32],
    ldh: i32,
    wr: &mut [f32],
    wi: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_shseqr_work(
        layout.into(),
        job as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr(),
        ldh,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dhseqr_work(
    layout: Layout,
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [f64],
    ldh: i32,
    wr: &mut [f64],
    wi: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dhseqr_work(
        layout.into(),
        job as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr(),
        ldh,
        wr.as_mut_ptr(),
        wi.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn chseqr_work(
    layout: Layout,
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c32],
    ldh: i32,
    w: &mut [c32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_chseqr_work(
        layout.into(),
        job as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr() as *mut _,
        ldh,
        w.as_mut_ptr() as *mut _,
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zhseqr_work(
    layout: Layout,
    job: u8,
    compz: u8,
    n: i32,
    ilo: i32,
    ihi: i32,
    h: &mut [c64],
    ldh: i32,
    w: &mut [c64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zhseqr_work(
        layout.into(),
        job as c_char,
        compz as c_char,
        n,
        ilo,
        ihi,
        h.as_mut_ptr() as *mut _,
        ldh,
        w.as_mut_ptr() as *mut _,
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn clacgv_work(n: i32, x: &mut [c32], incx: i32) -> i32 {
    ffi::LAPACKE_clacgv_work(n, x.as_mut_ptr() as *mut _, incx)
}

#[inline]
pub unsafe fn zlacgv_work(n: i32, x: &mut [c64], incx: i32) -> i32 {
    ffi::LAPACKE_zlacgv_work(n, x.as_mut_ptr() as *mut _, incx)
}

#[inline]
pub unsafe fn slacn2_work(
    n: i32,
    v: &mut [f32],
    x: &mut [f32],
    isgn: &mut [i32],
    est: &mut [f32],
    kase: &mut i32,
    isave: &mut [i32],
) -> i32 {
    ffi::LAPACKE_slacn2_work(
        n,
        v.as_mut_ptr(),
        x.as_mut_ptr(),
        isgn.as_mut_ptr(),
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlacn2_work(
    n: i32,
    v: &mut [f64],
    x: &mut [f64],
    isgn: &mut [i32],
    est: &mut [f64],
    kase: &mut i32,
    isave: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dlacn2_work(
        n,
        v.as_mut_ptr(),
        x.as_mut_ptr(),
        isgn.as_mut_ptr(),
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clacn2_work(
    n: i32,
    v: &mut [c32],
    x: &mut [c32],
    est: &mut [f32],
    kase: &mut i32,
    isave: &mut [i32],
) -> i32 {
    ffi::LAPACKE_clacn2_work(
        n,
        v.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlacn2_work(
    n: i32,
    v: &mut [c64],
    x: &mut [c64],
    est: &mut [f64],
    kase: &mut i32,
    isave: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zlacn2_work(
        n,
        v.as_mut_ptr() as *mut _,
        x.as_mut_ptr() as *mut _,
        est.as_mut_ptr(),
        kase,
        isave.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slacpy_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_slacpy_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dlacpy_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dlacpy_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn clacpy_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_clacpy_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zlacpy_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zlacpy_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn clacp2_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_clacp2_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zlacp2_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zlacp2_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zlag2c_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    sa: &mut [c32],
    ldsa: i32,
) -> i32 {
    ffi::LAPACKE_zlag2c_work(
        layout.into(),
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        sa.as_mut_ptr() as *mut _,
        ldsa,
    )
}

#[inline]
pub unsafe fn slag2d_work(
    layout: Layout,
    m: i32,
    n: i32,
    sa: &[f32],
    ldsa: i32,
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_slag2d_work(layout.into(), m, n, sa.as_ptr(), ldsa, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dlag2s_work(
    layout: Layout,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    sa: &mut [f32],
    ldsa: i32,
) -> i32 {
    ffi::LAPACKE_dlag2s_work(layout.into(), m, n, a.as_ptr(), lda, sa.as_mut_ptr(), ldsa)
}

#[inline]
pub unsafe fn clag2z_work(
    layout: Layout,
    m: i32,
    n: i32,
    sa: &[c32],
    ldsa: i32,
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_clag2z_work(
        layout.into(),
        m,
        n,
        sa.as_ptr() as *const _,
        ldsa,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn slagge_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f32],
    a: &mut [f32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_slagge_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        d.as_ptr(),
        a.as_mut_ptr(),
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlagge_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f64],
    a: &mut [f64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dlagge_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        d.as_ptr(),
        a.as_mut_ptr(),
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clagge_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f32],
    a: &mut [c32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_clagge_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zlagge_work(
    layout: Layout,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    d: &[f64],
    a: &mut [c64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zlagge_work(
        layout.into(),
        m,
        n,
        kl,
        ku,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn claghe_work(
    layout: Layout,
    n: i32,
    k: i32,
    d: &[f32],
    a: &mut [c32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_claghe_work(
        layout.into(),
        n,
        k,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zlaghe_work(
    layout: Layout,
    n: i32,
    k: i32,
    d: &[f64],
    a: &mut [c64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zlaghe_work(
        layout.into(),
        n,
        k,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn slagsy_work(
    layout: Layout,
    n: i32,
    k: i32,
    d: &[f32],
    a: &mut [f32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_slagsy_work(
        layout.into(),
        n,
        k,
        d.as_ptr(),
        a.as_mut_ptr(),
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlagsy_work(
    layout: Layout,
    n: i32,
    k: i32,
    d: &[f64],
    a: &mut [f64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dlagsy_work(
        layout.into(),
        n,
        k,
        d.as_ptr(),
        a.as_mut_ptr(),
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clagsy_work(
    layout: Layout,
    n: i32,
    k: i32,
    d: &[f32],
    a: &mut [c32],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_clagsy_work(
        layout.into(),
        n,
        k,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zlagsy_work(
    layout: Layout,
    n: i32,
    k: i32,
    d: &[f64],
    a: &mut [c64],
    lda: i32,
    iseed: &mut [i32],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zlagsy_work(
        layout.into(),
        n,
        k,
        d.as_ptr(),
        a.as_mut_ptr() as *mut _,
        lda,
        iseed.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn slapmr_work(
    layout: Layout,
    forwrd: i32,
    m: i32,
    n: i32,
    x: &mut [f32],
    ldx: i32,
    k: &mut i32,
) -> i32 {
    ffi::LAPACKE_slapmr_work(layout.into(), forwrd, m, n, x.as_mut_ptr(), ldx, k)
}

#[inline]
pub unsafe fn dlapmr_work(
    layout: Layout,
    forwrd: i32,
    m: i32,
    n: i32,
    x: &mut [f64],
    ldx: i32,
    k: &mut i32,
) -> i32 {
    ffi::LAPACKE_dlapmr_work(layout.into(), forwrd, m, n, x.as_mut_ptr(), ldx, k)
}

#[inline]
pub unsafe fn clapmr_work(
    layout: Layout,
    forwrd: i32,
    m: i32,
    n: i32,
    x: &mut [c32],
    ldx: i32,
    k: &mut i32,
) -> i32 {
    ffi::LAPACKE_clapmr_work(
        layout.into(),
        forwrd,
        m,
        n,
        x.as_mut_ptr() as *mut _,
        ldx,
        k,
    )
}

#[inline]
pub unsafe fn zlapmr_work(
    layout: Layout,
    forwrd: i32,
    m: i32,
    n: i32,
    x: &mut [c64],
    ldx: i32,
    k: &mut i32,
) -> i32 {
    ffi::LAPACKE_zlapmr_work(
        layout.into(),
        forwrd,
        m,
        n,
        x.as_mut_ptr() as *mut _,
        ldx,
        k,
    )
}

#[inline]
pub unsafe fn slapmt_work(
    layout: Layout,
    forwrd: i32,
    m: i32,
    n: i32,
    x: &mut [f32],
    ldx: i32,
    k: &mut i32,
) -> i32 {
    ffi::LAPACKE_slapmt_work(layout.into(), forwrd, m, n, x.as_mut_ptr(), ldx, k)
}

#[inline]
pub unsafe fn dlapmt_work(
    layout: Layout,
    forwrd: i32,
    m: i32,
    n: i32,
    x: &mut [f64],
    ldx: i32,
    k: &mut i32,
) -> i32 {
    ffi::LAPACKE_dlapmt_work(layout.into(), forwrd, m, n, x.as_mut_ptr(), ldx, k)
}

#[inline]
pub unsafe fn clapmt_work(
    layout: Layout,
    forwrd: i32,
    m: i32,
    n: i32,
    x: &mut [c32],
    ldx: i32,
    k: &mut i32,
) -> i32 {
    ffi::LAPACKE_clapmt_work(
        layout.into(),
        forwrd,
        m,
        n,
        x.as_mut_ptr() as *mut _,
        ldx,
        k,
    )
}

#[inline]
pub unsafe fn zlapmt_work(
    layout: Layout,
    forwrd: i32,
    m: i32,
    n: i32,
    x: &mut [c64],
    ldx: i32,
    k: &mut i32,
) -> i32 {
    ffi::LAPACKE_zlapmt_work(
        layout.into(),
        forwrd,
        m,
        n,
        x.as_mut_ptr() as *mut _,
        ldx,
        k,
    )
}

#[inline]
pub unsafe fn slartgp_work(f: f32, g: f32, cs: &mut [f32], sn: &mut [f32], r: &mut [f32]) -> i32 {
    ffi::LAPACKE_slartgp_work(f, g, cs.as_mut_ptr(), sn.as_mut_ptr(), r.as_mut_ptr())
}

#[inline]
pub unsafe fn dlartgp_work(f: f64, g: f64, cs: &mut [f64], sn: &mut [f64], r: &mut [f64]) -> i32 {
    ffi::LAPACKE_dlartgp_work(f, g, cs.as_mut_ptr(), sn.as_mut_ptr(), r.as_mut_ptr())
}

#[inline]
pub unsafe fn slartgs_work(x: f32, y: f32, sigma: f32, cs: &mut [f32], sn: &mut [f32]) -> i32 {
    ffi::LAPACKE_slartgs_work(x, y, sigma, cs.as_mut_ptr(), sn.as_mut_ptr())
}

#[inline]
pub unsafe fn dlartgs_work(x: f64, y: f64, sigma: f64, cs: &mut [f64], sn: &mut [f64]) -> i32 {
    ffi::LAPACKE_dlartgs_work(x, y, sigma, cs.as_mut_ptr(), sn.as_mut_ptr())
}

#[inline]
pub unsafe fn slapy2_work(x: f32, y: f32) -> f32 {
    ffi::LAPACKE_slapy2_work(x, y)
}

#[inline]
pub unsafe fn dlapy2_work(x: f64, y: f64) -> f64 {
    ffi::LAPACKE_dlapy2_work(x, y)
}

#[inline]
pub unsafe fn slapy3_work(x: f32, y: f32, z: f32) -> f32 {
    ffi::LAPACKE_slapy3_work(x, y, z)
}

#[inline]
pub unsafe fn dlapy3_work(x: f64, y: f64, z: f64) -> f64 {
    ffi::LAPACKE_dlapy3_work(x, y, z)
}

#[inline]
pub unsafe fn slamch_work(cmach: u8) -> f32 {
    ffi::LAPACKE_slamch_work(cmach as c_char)
}

#[inline]
pub unsafe fn dlamch_work(cmach: u8) -> f64 {
    ffi::LAPACKE_dlamch_work(cmach as c_char)
}

#[inline]
pub unsafe fn slange_work(
    layout: Layout,
    norm: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::LAPACKE_slange_work(
        layout.into(),
        norm as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlange_work(
    layout: Layout,
    norm: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::LAPACKE_dlange_work(
        layout.into(),
        norm as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clange_work(
    layout: Layout,
    norm: u8,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::LAPACKE_clange_work(
        layout.into(),
        norm as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlange_work(
    layout: Layout,
    norm: u8,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::LAPACKE_zlange_work(
        layout.into(),
        norm as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clanhe_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::LAPACKE_clanhe_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlanhe_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::LAPACKE_zlanhe_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slansy_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::LAPACKE_slansy_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlansy_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::LAPACKE_dlansy_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clansy_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::LAPACKE_clansy_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlansy_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::LAPACKE_zlansy_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slantr_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::LAPACKE_slantr_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlantr_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::LAPACKE_dlantr_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clantr_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    work: &mut [f32],
) -> f32 {
    ffi::LAPACKE_clantr_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zlantr_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    work: &mut [f64],
) -> f64 {
    ffi::LAPACKE_zlantr_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn slarfb_work(
    layout: Layout,
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[f32],
    ldv: i32,
    t: &[f32],
    ldt: i32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    ldwork: i32,
) -> i32 {
    ffi::LAPACKE_slarfb_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        direct as c_char,
        storev as c_char,
        m,
        n,
        k,
        v.as_ptr(),
        ldv,
        t.as_ptr(),
        ldt,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        ldwork,
    )
}

#[inline]
pub unsafe fn dlarfb_work(
    layout: Layout,
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[f64],
    ldv: i32,
    t: &[f64],
    ldt: i32,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    ldwork: i32,
) -> i32 {
    ffi::LAPACKE_dlarfb_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        direct as c_char,
        storev as c_char,
        m,
        n,
        k,
        v.as_ptr(),
        ldv,
        t.as_ptr(),
        ldt,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        ldwork,
    )
}

#[inline]
pub unsafe fn clarfb_work(
    layout: Layout,
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[c32],
    ldv: i32,
    t: &[c32],
    ldt: i32,
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
    ldwork: i32,
) -> i32 {
    ffi::LAPACKE_clarfb_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        direct as c_char,
        storev as c_char,
        m,
        n,
        k,
        v.as_ptr() as *const _,
        ldv,
        t.as_ptr() as *const _,
        ldt,
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr() as *mut _,
        ldwork,
    )
}

#[inline]
pub unsafe fn zlarfb_work(
    layout: Layout,
    side: u8,
    trans: u8,
    direct: u8,
    storev: u8,
    m: i32,
    n: i32,
    k: i32,
    v: &[c64],
    ldv: i32,
    t: &[c64],
    ldt: i32,
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
    ldwork: i32,
) -> i32 {
    ffi::LAPACKE_zlarfb_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        direct as c_char,
        storev as c_char,
        m,
        n,
        k,
        v.as_ptr() as *const _,
        ldv,
        t.as_ptr() as *const _,
        ldt,
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr() as *mut _,
        ldwork,
    )
}

#[inline]
pub unsafe fn slarfg_work(
    n: i32,
    alpha: &mut f32,
    x: &mut [f32],
    incx: i32,
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_slarfg_work(n, alpha, x.as_mut_ptr(), incx, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn dlarfg_work(
    n: i32,
    alpha: &mut f64,
    x: &mut [f64],
    incx: i32,
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dlarfg_work(n, alpha, x.as_mut_ptr(), incx, tau.as_mut_ptr())
}

#[inline]
pub unsafe fn clarfg_work(
    n: i32,
    alpha: &mut c32,
    x: &mut [c32],
    incx: i32,
    tau: &mut [c32],
) -> i32 {
    ffi::LAPACKE_clarfg_work(
        n,
        alpha as *mut _ as *mut _,
        x.as_mut_ptr() as *mut _,
        incx,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zlarfg_work(
    n: i32,
    alpha: &mut c64,
    x: &mut [c64],
    incx: i32,
    tau: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zlarfg_work(
        n,
        alpha as *mut _ as *mut _,
        x.as_mut_ptr() as *mut _,
        incx,
        tau.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn slarft_work(
    layout: Layout,
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[f32],
    ldv: i32,
    tau: &[f32],
    t: &mut [f32],
    ldt: i32,
) -> i32 {
    ffi::LAPACKE_slarft_work(
        layout.into(),
        direct as c_char,
        storev as c_char,
        n,
        k,
        v.as_ptr(),
        ldv,
        tau.as_ptr(),
        t.as_mut_ptr(),
        ldt,
    )
}

#[inline]
pub unsafe fn dlarft_work(
    layout: Layout,
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[f64],
    ldv: i32,
    tau: &[f64],
    t: &mut [f64],
    ldt: i32,
) -> i32 {
    ffi::LAPACKE_dlarft_work(
        layout.into(),
        direct as c_char,
        storev as c_char,
        n,
        k,
        v.as_ptr(),
        ldv,
        tau.as_ptr(),
        t.as_mut_ptr(),
        ldt,
    )
}

#[inline]
pub unsafe fn clarft_work(
    layout: Layout,
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[c32],
    ldv: i32,
    tau: &[c32],
    t: &mut [c32],
    ldt: i32,
) -> i32 {
    ffi::LAPACKE_clarft_work(
        layout.into(),
        direct as c_char,
        storev as c_char,
        n,
        k,
        v.as_ptr() as *const _,
        ldv,
        tau.as_ptr() as *const _,
        t.as_mut_ptr() as *mut _,
        ldt,
    )
}

#[inline]
pub unsafe fn zlarft_work(
    layout: Layout,
    direct: u8,
    storev: u8,
    n: i32,
    k: i32,
    v: &[c64],
    ldv: i32,
    tau: &[c64],
    t: &mut [c64],
    ldt: i32,
) -> i32 {
    ffi::LAPACKE_zlarft_work(
        layout.into(),
        direct as c_char,
        storev as c_char,
        n,
        k,
        v.as_ptr() as *const _,
        ldv,
        tau.as_ptr() as *const _,
        t.as_mut_ptr() as *mut _,
        ldt,
    )
}

#[inline]
pub unsafe fn slarfx_work(
    layout: Layout,
    side: u8,
    m: i32,
    n: i32,
    v: &[f32],
    tau: f32,
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_slarfx_work(
        layout.into(),
        side as c_char,
        m,
        n,
        v.as_ptr(),
        tau,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlarfx_work(
    layout: Layout,
    side: u8,
    m: i32,
    n: i32,
    v: &[f64],
    tau: f64,
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dlarfx_work(
        layout.into(),
        side as c_char,
        m,
        n,
        v.as_ptr(),
        tau,
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clarfx_work(
    layout: Layout,
    side: u8,
    m: i32,
    n: i32,
    v: &[c32],
    tau: c32,
    c: &mut [c32],
    ldc: i32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_clarfx_work(
        layout.into(),
        side as c_char,
        m,
        n,
        v.as_ptr() as *const _,
        transmute(tau),
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zlarfx_work(
    layout: Layout,
    side: u8,
    m: i32,
    n: i32,
    v: &[c64],
    tau: c64,
    c: &mut [c64],
    ldc: i32,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zlarfx_work(
        layout.into(),
        side as c_char,
        m,
        n,
        v.as_ptr() as *const _,
        transmute(tau),
        c.as_mut_ptr() as *mut _,
        ldc,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn slarnv_work(idist: i32, iseed: &mut [i32], n: i32, x: &mut [f32]) -> i32 {
    ffi::LAPACKE_slarnv_work(idist, iseed.as_mut_ptr(), n, x.as_mut_ptr())
}

#[inline]
pub unsafe fn dlarnv_work(idist: i32, iseed: &mut [i32], n: i32, x: &mut [f64]) -> i32 {
    ffi::LAPACKE_dlarnv_work(idist, iseed.as_mut_ptr(), n, x.as_mut_ptr())
}

#[inline]
pub unsafe fn clarnv_work(idist: i32, iseed: &mut [i32], n: i32, x: &mut [c32]) -> i32 {
    ffi::LAPACKE_clarnv_work(idist, iseed.as_mut_ptr(), n, x.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn zlarnv_work(idist: i32, iseed: &mut [i32], n: i32, x: &mut [c64]) -> i32 {
    ffi::LAPACKE_zlarnv_work(idist, iseed.as_mut_ptr(), n, x.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn slascl_work(
    layout: Layout,
    _type: u8,
    kl: i32,
    ku: i32,
    cfrom: f32,
    cto: f32,
    m: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_slascl_work(
        layout.into(),
        _type as c_char,
        kl,
        ku,
        cfrom,
        cto,
        m,
        n,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dlascl_work(
    layout: Layout,
    _type: u8,
    kl: i32,
    ku: i32,
    cfrom: f64,
    cto: f64,
    m: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_dlascl_work(
        layout.into(),
        _type as c_char,
        kl,
        ku,
        cfrom,
        cto,
        m,
        n,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn clascl_work(
    layout: Layout,
    _type: u8,
    kl: i32,
    ku: i32,
    cfrom: f32,
    cto: f32,
    m: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_clascl_work(
        layout.into(),
        _type as c_char,
        kl,
        ku,
        cfrom,
        cto,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zlascl_work(
    layout: Layout,
    _type: u8,
    kl: i32,
    ku: i32,
    cfrom: f64,
    cto: f64,
    m: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_zlascl_work(
        layout.into(),
        _type as c_char,
        kl,
        ku,
        cfrom,
        cto,
        m,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn slaset_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    alpha: f32,
    beta: f32,
    a: &mut [f32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_slaset_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        alpha,
        beta,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dlaset_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    alpha: f64,
    beta: f64,
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_dlaset_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        alpha,
        beta,
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn claset_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    alpha: c32,
    beta: c32,
    a: &mut [c32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_claset_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        transmute(alpha),
        transmute(beta),
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zlaset_work(
    layout: Layout,
    uplo: u8,
    m: i32,
    n: i32,
    alpha: c64,
    beta: c64,
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_zlaset_work(
        layout.into(),
        uplo as c_char,
        m,
        n,
        transmute(alpha),
        transmute(beta),
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn slasrt_work(id: u8, n: i32, d: &mut [f32]) -> i32 {
    ffi::LAPACKE_slasrt_work(id as c_char, n, d.as_mut_ptr())
}

#[inline]
pub unsafe fn dlasrt_work(id: u8, n: i32, d: &mut [f64]) -> i32 {
    ffi::LAPACKE_dlasrt_work(id as c_char, n, d.as_mut_ptr())
}

#[inline]
pub unsafe fn slaswp_work(
    layout: Layout,
    n: i32,
    a: &mut [f32],
    lda: i32,
    k1: i32,
    k2: i32,
    ipiv: &[i32],
    incx: i32,
) -> i32 {
    ffi::LAPACKE_slaswp_work(
        layout.into(),
        n,
        a.as_mut_ptr(),
        lda,
        k1,
        k2,
        ipiv.as_ptr(),
        incx,
    )
}

#[inline]
pub unsafe fn dlaswp_work(
    layout: Layout,
    n: i32,
    a: &mut [f64],
    lda: i32,
    k1: i32,
    k2: i32,
    ipiv: &[i32],
    incx: i32,
) -> i32 {
    ffi::LAPACKE_dlaswp_work(
        layout.into(),
        n,
        a.as_mut_ptr(),
        lda,
        k1,
        k2,
        ipiv.as_ptr(),
        incx,
    )
}

#[inline]
pub unsafe fn claswp_work(
    layout: Layout,
    n: i32,
    a: &mut [c32],
    lda: i32,
    k1: i32,
    k2: i32,
    ipiv: &[i32],
    incx: i32,
) -> i32 {
    ffi::LAPACKE_claswp_work(
        layout.into(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        k1,
        k2,
        ipiv.as_ptr(),
        incx,
    )
}

#[inline]
pub unsafe fn zlaswp_work(
    layout: Layout,
    n: i32,
    a: &mut [c64],
    lda: i32,
    k1: i32,
    k2: i32,
    ipiv: &[i32],
    incx: i32,
) -> i32 {
    ffi::LAPACKE_zlaswp_work(
        layout.into(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        k1,
        k2,
        ipiv.as_ptr(),
        incx,
    )
}

#[inline]
pub unsafe fn slatms_work(
    layout: Layout,
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f32],
    mode: i32,
    cond: f32,
    dmax: f32,
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [f32],
    lda: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_slatms_work(
        layout.into(),
        m,
        n,
        dist as c_char,
        iseed.as_mut_ptr(),
        sym as c_char,
        d.as_mut_ptr(),
        mode,
        cond,
        dmax,
        kl,
        ku,
        pack as c_char,
        a.as_mut_ptr(),
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dlatms_work(
    layout: Layout,
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f64],
    mode: i32,
    cond: f64,
    dmax: f64,
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [f64],
    lda: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dlatms_work(
        layout.into(),
        m,
        n,
        dist as c_char,
        iseed.as_mut_ptr(),
        sym as c_char,
        d.as_mut_ptr(),
        mode,
        cond,
        dmax,
        kl,
        ku,
        pack as c_char,
        a.as_mut_ptr(),
        lda,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn clatms_work(
    layout: Layout,
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f32],
    mode: i32,
    cond: f32,
    dmax: f32,
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [c32],
    lda: i32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_clatms_work(
        layout.into(),
        m,
        n,
        dist as c_char,
        iseed.as_mut_ptr(),
        sym as c_char,
        d.as_mut_ptr(),
        mode,
        cond,
        dmax,
        kl,
        ku,
        pack as c_char,
        a.as_mut_ptr() as *mut _,
        lda,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zlatms_work(
    layout: Layout,
    m: i32,
    n: i32,
    dist: u8,
    iseed: &mut [i32],
    sym: u8,
    d: &mut [f64],
    mode: i32,
    cond: f64,
    dmax: f64,
    kl: i32,
    ku: i32,
    pack: u8,
    a: &mut [c64],
    lda: i32,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zlatms_work(
        layout.into(),
        m,
        n,
        dist as c_char,
        iseed.as_mut_ptr(),
        sym as c_char,
        d.as_mut_ptr(),
        mode,
        cond,
        dmax,
        kl,
        ku,
        pack as c_char,
        a.as_mut_ptr() as *mut _,
        lda,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn slauum_work(layout: Layout, uplo: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_slauum_work(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dlauum_work(layout: Layout, uplo: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dlauum_work(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn clauum_work(layout: Layout, uplo: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_clauum_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zlauum_work(layout: Layout, uplo: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_zlauum_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn sopgtr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    tau: &[f32],
    q: &mut f32,
    ldq: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sopgtr_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        tau.as_ptr(),
        q,
        ldq,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dopgtr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    tau: &[f64],
    q: &mut f64,
    ldq: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dopgtr_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        tau.as_ptr(),
        q,
        ldq,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sopmtr_work(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[f32],
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sopmtr_work(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        ap.as_ptr(),
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dopmtr_work(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    ap: &[f64],
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dopmtr_work(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        ap.as_ptr(),
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sorgbr_work(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sorgbr_work(
        layout.into(),
        vect as c_char,
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dorgbr_work(
    layout: Layout,
    vect: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dorgbr_work(
        layout.into(),
        vect as c_char,
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sorghr_work(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sorghr_work(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dorghr_work(
    layout: Layout,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dorghr_work(
        layout.into(),
        n,
        ilo,
        ihi,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sorglq_work(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sorglq_work(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dorglq_work(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dorglq_work(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sorgql_work(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sorgql_work(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dorgql_work(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dorgql_work(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sorgqr_work(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sorgqr_work(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dorgqr_work(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dorgqr_work(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sorgrq_work(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sorgrq_work(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dorgrq_work(
    layout: Layout,
    m: i32,
    n: i32,
    k: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dorgrq_work(
        layout.into(),
        m,
        n,
        k,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sorgtr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    tau: &[f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sorgtr_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dorgtr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    tau: &[f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dorgtr_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        tau.as_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sormbr_work(
    layout: Layout,
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sormbr_work(
        layout.into(),
        vect as c_char,
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dormbr_work(
    layout: Layout,
    vect: u8,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dormbr_work(
        layout.into(),
        vect as c_char,
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sormhr_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sormhr_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        ilo,
        ihi,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dormhr_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    ilo: i32,
    ihi: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dormhr_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        ilo,
        ihi,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sormlq_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sormlq_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dormlq_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dormlq_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sormql_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sormql_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dormql_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dormql_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sormqr_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sormqr_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dormqr_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dormqr_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sormrq_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sormrq_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dormrq_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dormrq_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sormrz_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sormrz_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        l,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dormrz_work(
    layout: Layout,
    side: u8,
    trans: u8,
    m: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dormrz_work(
        layout.into(),
        side as c_char,
        trans as c_char,
        m,
        n,
        k,
        l,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn sormtr_work(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    tau: &[f32],
    c: &mut [f32],
    ldc: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_sormtr_work(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dormtr_work(
    layout: Layout,
    side: u8,
    uplo: u8,
    trans: u8,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    tau: &[f64],
    c: &mut [f64],
    ldc: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dormtr_work(
        layout.into(),
        side as c_char,
        uplo as c_char,
        trans as c_char,
        m,
        n,
        a.as_ptr(),
        lda,
        tau.as_ptr(),
        c.as_mut_ptr(),
        ldc,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn spbcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_spbcon_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpbcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dpbcon_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpbcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpbcon_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpbcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpbcon_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spbequ_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_spbequ_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dpbequ_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dpbequ_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cpbequ_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cpbequ_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zpbequ_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zpbequ_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn spbrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    afb: &[f32],
    ldafb: i32,
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_spbrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpbrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    afb: &[f64],
    ldafb: i32,
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dpbrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        afb.as_ptr(),
        ldafb,
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpbrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    afb: &[c32],
    ldafb: i32,
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpbrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpbrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    afb: &[c64],
    ldafb: i32,
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpbrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        afb.as_ptr() as *const _,
        ldafb,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spbstf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kb: i32,
    bb: &mut [f32],
    ldbb: i32,
) -> i32 {
    ffi::LAPACKE_spbstf_work(layout.into(), uplo as c_char, n, kb, bb.as_mut_ptr(), ldbb)
}

#[inline]
pub unsafe fn dpbstf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kb: i32,
    bb: &mut [f64],
    ldbb: i32,
) -> i32 {
    ffi::LAPACKE_dpbstf_work(layout.into(), uplo as c_char, n, kb, bb.as_mut_ptr(), ldbb)
}

#[inline]
pub unsafe fn cpbstf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kb: i32,
    bb: &mut [c32],
    ldbb: i32,
) -> i32 {
    ffi::LAPACKE_cpbstf_work(
        layout.into(),
        uplo as c_char,
        n,
        kb,
        bb.as_mut_ptr() as *mut _,
        ldbb,
    )
}

#[inline]
pub unsafe fn zpbstf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kb: i32,
    bb: &mut [c64],
    ldbb: i32,
) -> i32 {
    ffi::LAPACKE_zpbstf_work(
        layout.into(),
        uplo as c_char,
        n,
        kb,
        bb.as_mut_ptr() as *mut _,
        ldbb,
    )
}

#[inline]
pub unsafe fn spbsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spbsv_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpbsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpbsv_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpbsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpbsv_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpbsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpbsv_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn spbsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f32],
    ldab: i32,
    afb: &mut [f32],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_spbsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpbsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [f64],
    ldab: i32,
    afb: &mut [f64],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dpbsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr(),
        ldab,
        afb.as_mut_ptr(),
        ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpbsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c32],
    ldab: i32,
    afb: &mut [c32],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpbsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpbsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &mut [c64],
    ldab: i32,
    afb: &mut [c64],
    ldafb: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpbsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_mut_ptr() as *mut _,
        ldab,
        afb.as_mut_ptr() as *mut _,
        ldafb,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spbtrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
) -> i32 {
    ffi::LAPACKE_spbtrf_work(layout.into(), uplo as c_char, n, kd, ab.as_mut_ptr(), ldab)
}

#[inline]
pub unsafe fn dpbtrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
) -> i32 {
    ffi::LAPACKE_dpbtrf_work(layout.into(), uplo as c_char, n, kd, ab.as_mut_ptr(), ldab)
}

#[inline]
pub unsafe fn cpbtrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c32],
    ldab: i32,
) -> i32 {
    ffi::LAPACKE_cpbtrf_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
    )
}

#[inline]
pub unsafe fn zpbtrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [c64],
    ldab: i32,
) -> i32 {
    ffi::LAPACKE_zpbtrf_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr() as *mut _,
        ldab,
    )
}

#[inline]
pub unsafe fn spbtrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spbtrs_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpbtrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpbtrs_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpbtrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpbtrs_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpbtrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpbtrs_work(
        layout.into(),
        uplo as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn spftrf_work(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [f32]) -> i32 {
    ffi::LAPACKE_spftrf_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpftrf_work(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpftrf_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpftrf_work(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpftrf_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zpftrf_work(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpftrf_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn spftri_work(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [f32]) -> i32 {
    ffi::LAPACKE_spftri_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpftri_work(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpftri_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpftri_work(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpftri_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zpftri_work(layout: Layout, transr: u8, uplo: u8, n: i32, a: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpftri_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn spftrs_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spftrs_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpftrs_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpftrs_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpftrs_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpftrs_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpftrs_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpftrs_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn spocon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_spocon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpocon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dpocon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpocon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpocon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpocon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpocon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spoequ_work(
    layout: Layout,
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_spoequ_work(
        layout.into(),
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dpoequ_work(
    layout: Layout,
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dpoequ_work(
        layout.into(),
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cpoequ_work(
    layout: Layout,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cpoequ_work(
        layout.into(),
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zpoequ_work(
    layout: Layout,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zpoequ_work(
        layout.into(),
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn spoequb_work(
    layout: Layout,
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_spoequb_work(
        layout.into(),
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dpoequb_work(
    layout: Layout,
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dpoequb_work(
        layout.into(),
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cpoequb_work(
    layout: Layout,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cpoequb_work(
        layout.into(),
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zpoequb_work(
    layout: Layout,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zpoequb_work(
        layout.into(),
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn sporfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sporfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dporfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dporfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cporfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cporfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zporfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zporfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sporfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    s: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sporfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        s.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dporfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    s: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dporfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        s.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cporfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    s: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cporfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zporfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    s: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zporfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sposv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sposv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dposv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dposv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cposv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cposv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zposv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zposv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn dsposv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    work: &mut [f64],
    swork: &mut [f32],
    iter: &mut i32,
) -> i32 {
    ffi::LAPACKE_dsposv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        work.as_mut_ptr(),
        swork.as_mut_ptr(),
        iter,
    )
}

#[inline]
pub unsafe fn zcposv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    work: &mut [c64],
    swork: &mut [c32],
    rwork: &mut [f64],
    iter: &mut i32,
) -> i32 {
    ffi::LAPACKE_zcposv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        work.as_mut_ptr() as *mut _,
        swork.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
        iter,
    )
}

#[inline]
pub unsafe fn sposvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sposvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dposvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dposvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cposvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cposvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zposvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zposvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sposvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sposvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dposvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dposvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cposvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cposvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zposvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zposvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spotrf2_work(layout: Layout, uplo: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_spotrf2_work(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dpotrf2_work(layout: Layout, uplo: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dpotrf2_work(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn cpotrf2_work(layout: Layout, uplo: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_cpotrf2_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zpotrf2_work(layout: Layout, uplo: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_zpotrf2_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn spotrf_work(layout: Layout, uplo: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_spotrf_work(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dpotrf_work(layout: Layout, uplo: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dpotrf_work(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn cpotrf_work(layout: Layout, uplo: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_cpotrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zpotrf_work(layout: Layout, uplo: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_zpotrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn spotri_work(layout: Layout, uplo: u8, n: i32, a: &mut [f32], lda: i32) -> i32 {
    ffi::LAPACKE_spotri_work(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn dpotri_work(layout: Layout, uplo: u8, n: i32, a: &mut [f64], lda: i32) -> i32 {
    ffi::LAPACKE_dpotri_work(layout.into(), uplo as c_char, n, a.as_mut_ptr(), lda)
}

#[inline]
pub unsafe fn cpotri_work(layout: Layout, uplo: u8, n: i32, a: &mut [c32], lda: i32) -> i32 {
    ffi::LAPACKE_cpotri_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn zpotri_work(layout: Layout, uplo: u8, n: i32, a: &mut [c64], lda: i32) -> i32 {
    ffi::LAPACKE_zpotri_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn spotrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spotrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpotrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpotrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpotrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpotrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpotrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpotrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sppcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sppcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dppcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dppcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cppcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cppcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zppcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zppcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sppequ_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_sppequ_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn dppequ_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_dppequ_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn cppequ_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
) -> i32 {
    ffi::LAPACKE_cppequ_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn zppequ_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
) -> i32 {
    ffi::LAPACKE_zppequ_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
    )
}

#[inline]
pub unsafe fn spprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_spprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dpprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sppsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sppsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dppsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dppsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cppsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cppsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zppsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zppsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sppsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    afp: &mut [f32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sppsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        afp.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dppsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    afp: &mut [f64],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dppsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        afp.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cppsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    afp: &mut [c32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cppsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        afp.as_mut_ptr() as *mut _,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zppsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    afp: &mut [c64],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zppsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        afp.as_mut_ptr() as *mut _,
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spptrf_work(layout: Layout, uplo: u8, n: i32, ap: &mut [f32]) -> i32 {
    ffi::LAPACKE_spptrf_work(layout.into(), uplo as c_char, n, ap.as_mut_ptr())
}

#[inline]
pub unsafe fn dpptrf_work(layout: Layout, uplo: u8, n: i32, ap: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpptrf_work(layout.into(), uplo as c_char, n, ap.as_mut_ptr())
}

#[inline]
pub unsafe fn cpptrf_work(layout: Layout, uplo: u8, n: i32, ap: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpptrf_work(layout.into(), uplo as c_char, n, ap.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn zpptrf_work(layout: Layout, uplo: u8, n: i32, ap: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpptrf_work(layout.into(), uplo as c_char, n, ap.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn spptri_work(layout: Layout, uplo: u8, n: i32, ap: &mut [f32]) -> i32 {
    ffi::LAPACKE_spptri_work(layout.into(), uplo as c_char, n, ap.as_mut_ptr())
}

#[inline]
pub unsafe fn dpptri_work(layout: Layout, uplo: u8, n: i32, ap: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpptri_work(layout.into(), uplo as c_char, n, ap.as_mut_ptr())
}

#[inline]
pub unsafe fn cpptri_work(layout: Layout, uplo: u8, n: i32, ap: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpptri_work(layout.into(), uplo as c_char, n, ap.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn zpptri_work(layout: Layout, uplo: u8, n: i32, ap: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpptri_work(layout.into(), uplo as c_char, n, ap.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn spptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn spstrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_spstrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        piv.as_mut_ptr(),
        rank,
        tol,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpstrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f64,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dpstrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        piv.as_mut_ptr(),
        rank,
        tol,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpstrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpstrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        piv.as_mut_ptr(),
        rank,
        tol,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpstrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    piv: &mut [i32],
    rank: &mut i32,
    tol: f64,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpstrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        piv.as_mut_ptr(),
        rank,
        tol,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sptcon_work(
    n: i32,
    d: &[f32],
    e: &[f32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sptcon_work(n, d.as_ptr(), e.as_ptr(), anorm, rcond, work.as_mut_ptr())
}

#[inline]
pub unsafe fn dptcon_work(
    n: i32,
    d: &[f64],
    e: &[f64],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dptcon_work(n, d.as_ptr(), e.as_ptr(), anorm, rcond, work.as_mut_ptr())
}

#[inline]
pub unsafe fn cptcon_work(
    n: i32,
    d: &[f32],
    e: &[c32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cptcon_work(
        n,
        d.as_ptr(),
        e.as_ptr() as *const _,
        anorm,
        rcond,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zptcon_work(
    n: i32,
    d: &[f64],
    e: &[c64],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zptcon_work(
        n,
        d.as_ptr(),
        e.as_ptr() as *const _,
        anorm,
        rcond,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spteqr_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_spteqr_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dpteqr_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dpteqr_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cpteqr_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cpteqr_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zpteqr_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zpteqr_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sptrfs_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    df: &[f32],
    ef: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sptrfs_work(
        layout.into(),
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_ptr(),
        ef.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dptrfs_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    df: &[f64],
    ef: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dptrfs_work(
        layout.into(),
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_ptr(),
        ef.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cptrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    df: &[f32],
    ef: &[c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cptrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_ptr(),
        ef.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zptrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    df: &[f64],
    ef: &[c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zptrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_ptr(),
        ef.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sptsv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &mut [f32],
    e: &mut [f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sptsv_work(
        layout.into(),
        n,
        nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dptsv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &mut [f64],
    e: &mut [f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dptsv_work(
        layout.into(),
        n,
        nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cptsv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &mut [f32],
    e: &mut [c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cptsv_work(
        layout.into(),
        n,
        nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zptsv_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &mut [f64],
    e: &mut [c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zptsv_work(
        layout.into(),
        n,
        nrhs,
        d.as_mut_ptr(),
        e.as_mut_ptr() as *mut _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sptsvx_work(
    layout: Layout,
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    df: &mut [f32],
    ef: &mut [f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sptsvx_work(
        layout.into(),
        fact as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_mut_ptr(),
        ef.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dptsvx_work(
    layout: Layout,
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    df: &mut [f64],
    ef: &mut [f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dptsvx_work(
        layout.into(),
        fact as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        df.as_mut_ptr(),
        ef.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cptsvx_work(
    layout: Layout,
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    df: &mut [f32],
    ef: &mut [c32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cptsvx_work(
        layout.into(),
        fact as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_mut_ptr(),
        ef.as_mut_ptr() as *mut _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zptsvx_work(
    layout: Layout,
    fact: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    df: &mut [f64],
    ef: &mut [c64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zptsvx_work(
        layout.into(),
        fact as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        df.as_mut_ptr(),
        ef.as_mut_ptr() as *mut _,
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn spttrf_work(n: i32, d: &mut [f32], e: &mut [f32]) -> i32 {
    ffi::LAPACKE_spttrf_work(n, d.as_mut_ptr(), e.as_mut_ptr())
}

#[inline]
pub unsafe fn dpttrf_work(n: i32, d: &mut [f64], e: &mut [f64]) -> i32 {
    ffi::LAPACKE_dpttrf_work(n, d.as_mut_ptr(), e.as_mut_ptr())
}

#[inline]
pub unsafe fn cpttrf_work(n: i32, d: &mut [f32], e: &mut [c32]) -> i32 {
    ffi::LAPACKE_cpttrf_work(n, d.as_mut_ptr(), e.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn zpttrf_work(n: i32, d: &mut [f64], e: &mut [c64]) -> i32 {
    ffi::LAPACKE_zpttrf_work(n, d.as_mut_ptr(), e.as_mut_ptr() as *mut _)
}

#[inline]
pub unsafe fn spttrs_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_spttrs_work(
        layout.into(),
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dpttrs_work(
    layout: Layout,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dpttrs_work(
        layout.into(),
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cpttrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f32],
    e: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cpttrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zpttrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    d: &[f64],
    e: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zpttrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        d.as_ptr(),
        e.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ssbev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssbev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsbev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsbev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssbevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_ssbevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dsbevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dsbevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn ssbevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    q: &mut f32,
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssbevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        q,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsbevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    q: &mut f64,
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsbevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        q,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssbgst_work(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &[f32],
    ldbb: i32,
    x: &mut [f32],
    ldx: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssbgst_work(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_ptr(),
        ldbb,
        x.as_mut_ptr(),
        ldx,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsbgst_work(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &[f64],
    ldbb: i32,
    x: &mut [f64],
    ldx: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsbgst_work(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_ptr(),
        ldbb,
        x.as_mut_ptr(),
        ldx,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssbgv_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssbgv_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsbgv_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsbgv_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssbgvd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_ssbgvd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dsbgvd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dsbgvd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn ssbgvx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f32],
    ldab: i32,
    bb: &mut [f32],
    ldbb: i32,
    q: &mut f32,
    ldq: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssbgvx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        q,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsbgvx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ka: i32,
    kb: i32,
    ab: &mut [f64],
    ldab: i32,
    bb: &mut [f64],
    ldbb: i32,
    q: &mut f64,
    ldq: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsbgvx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ka,
        kb,
        ab.as_mut_ptr(),
        ldab,
        bb.as_mut_ptr(),
        ldbb,
        q,
        ldq,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssbtrd_work(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f32],
    ldab: i32,
    d: &mut [f32],
    e: &mut [f32],
    q: &mut f32,
    ldq: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssbtrd_work(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q,
        ldq,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsbtrd_work(
    layout: Layout,
    vect: u8,
    uplo: u8,
    n: i32,
    kd: i32,
    ab: &mut [f64],
    ldab: i32,
    d: &mut [f64],
    e: &mut [f64],
    q: &mut f64,
    ldq: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsbtrd_work(
        layout.into(),
        vect as c_char,
        uplo as c_char,
        n,
        kd,
        ab.as_mut_ptr(),
        ldab,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        q,
        ldq,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssfrk_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    beta: f32,
    c: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssfrk_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        trans as c_char,
        n,
        k,
        alpha,
        a.as_ptr(),
        lda,
        beta,
        c.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsfrk_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    trans: u8,
    n: i32,
    k: i32,
    alpha: f64,
    a: &[f64],
    lda: i32,
    beta: f64,
    c: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsfrk_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        trans as c_char,
        n,
        k,
        alpha,
        a.as_ptr(),
        lda,
        beta,
        c.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sspcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sspcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dspcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cspcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_cspcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zspcon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zspcon_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn sspev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sspev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dspev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sspevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_sspevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dspevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dspevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn sspevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sspevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dspevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sspgst_work(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &[f32],
) -> i32 {
    ffi::LAPACKE_sspgst_work(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_ptr(),
    )
}

#[inline]
pub unsafe fn dspgst_work(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &[f64],
) -> i32 {
    ffi::LAPACKE_dspgst_work(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_ptr(),
    )
}

#[inline]
pub unsafe fn sspgv_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sspgv_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspgv_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dspgv_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sspgvd_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_sspgvd_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dspgvd_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dspgvd_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn sspgvx_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    bp: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sspgvx_work(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspgvx_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    bp: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dspgvx_work(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        bp.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &[f32],
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &[f64],
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_ptr(),
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &[c32],
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsprfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &[c64],
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsprfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sspsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f32],
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_sspsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dspsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [f64],
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dspsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn cspsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c32],
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_cspsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zspsv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &mut [c64],
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zspsv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sspsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    afp: &mut [f32],
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sspsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dspsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    afp: &mut [f64],
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dspsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        afp.as_mut_ptr(),
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cspsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    afp: &mut [c32],
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_cspsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zspsvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    afp: &mut [c64],
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zspsvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        afp.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssptrd_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssptrd_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsptrd_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsptrd_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssptrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssptrf_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsptrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsptrf_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csptrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_csptrf_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsptrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    ipiv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zsptrf_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssptri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [f32],
    ipiv: &[i32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssptri_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        ipiv.as_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsptri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [f64],
    ipiv: &[i32],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsptri_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr(),
        ipiv.as_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csptri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c32],
    ipiv: &[i32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_csptri_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zsptri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &mut [c64],
    ipiv: &[i32],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zsptri_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ssptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ssptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dsptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dsptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn csptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_csptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zsptrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zsptrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn sstebz_work(
    range: u8,
    order: u8,
    n: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    d: &[f32],
    e: &[f32],
    m: &mut i32,
    nsplit: &mut [i32],
    w: &mut [f32],
    iblock: &mut [i32],
    isplit: &mut [i32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sstebz_work(
        range as c_char,
        order as c_char,
        n,
        vl,
        vu,
        il,
        iu,
        abstol,
        d.as_ptr(),
        e.as_ptr(),
        m,
        nsplit.as_mut_ptr(),
        w.as_mut_ptr(),
        iblock.as_mut_ptr(),
        isplit.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstebz_work(
    range: u8,
    order: u8,
    n: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    d: &[f64],
    e: &[f64],
    m: &mut i32,
    nsplit: &mut [i32],
    w: &mut [f64],
    iblock: &mut [i32],
    isplit: &mut [i32],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dstebz_work(
        range as c_char,
        order as c_char,
        n,
        vl,
        vu,
        il,
        iu,
        abstol,
        d.as_ptr(),
        e.as_ptr(),
        m,
        nsplit.as_mut_ptr(),
        w.as_mut_ptr(),
        iblock.as_mut_ptr(),
        isplit.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sstedc_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_sstedc_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dstedc_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dstedc_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn cstedc_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_cstedc_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zstedc_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
    lrwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zstedc_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
        lrwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn sstegr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_sstegr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dstegr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dstegr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn cstegr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_cstegr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zstegr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zstegr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn sstein_work(
    layout: Layout,
    n: i32,
    d: &[f32],
    e: &[f32],
    m: i32,
    w: &[f32],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifailv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sstein_work(
        layout.into(),
        n,
        d.as_ptr(),
        e.as_ptr(),
        m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifailv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstein_work(
    layout: Layout,
    n: i32,
    d: &[f64],
    e: &[f64],
    m: i32,
    w: &[f64],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifailv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dstein_work(
        layout.into(),
        n,
        d.as_ptr(),
        e.as_ptr(),
        m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifailv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn cstein_work(
    layout: Layout,
    n: i32,
    d: &[f32],
    e: &[f32],
    m: i32,
    w: &[f32],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifailv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_cstein_work(
        layout.into(),
        n,
        d.as_ptr(),
        e.as_ptr(),
        m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifailv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zstein_work(
    layout: Layout,
    n: i32,
    d: &[f64],
    e: &[f64],
    m: i32,
    w: &[f64],
    iblock: &[i32],
    isplit: &[i32],
    z: &mut [c64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifailv: &mut [i32],
) -> i32 {
    ffi::LAPACKE_zstein_work(
        layout.into(),
        n,
        d.as_ptr(),
        e.as_ptr(),
        m,
        w.as_ptr(),
        iblock.as_ptr(),
        isplit.as_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifailv.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sstemr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    nzc: i32,
    isuppz: &mut [i32],
    tryrac: &mut i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_sstemr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        nzc,
        isuppz.as_mut_ptr(),
        tryrac,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dstemr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    nzc: i32,
    isuppz: &mut [i32],
    tryrac: &mut i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dstemr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        nzc,
        isuppz.as_mut_ptr(),
        tryrac,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn cstemr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    nzc: i32,
    isuppz: &mut [i32],
    tryrac: &mut i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_cstemr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        nzc,
        isuppz.as_mut_ptr(),
        tryrac,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn zstemr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    nzc: i32,
    isuppz: &mut [i32],
    tryrac: &mut i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_zstemr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        nzc,
        isuppz.as_mut_ptr(),
        tryrac,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn ssteqr_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssteqr_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsteqr_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsteqr_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csteqr_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [c32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csteqr_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsteqr_work(
    layout: Layout,
    compz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [c64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsteqr_work(
        layout.into(),
        compz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr() as *mut _,
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssterf_work(n: i32, d: &mut [f32], e: &mut [f32]) -> i32 {
    ffi::LAPACKE_ssterf_work(n, d.as_mut_ptr(), e.as_mut_ptr())
}

#[inline]
pub unsafe fn dsterf_work(n: i32, d: &mut [f64], e: &mut [f64]) -> i32 {
    ffi::LAPACKE_dsterf_work(n, d.as_mut_ptr(), e.as_mut_ptr())
}

#[inline]
pub unsafe fn sstev_work(
    layout: Layout,
    jobz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_sstev_work(
        layout.into(),
        jobz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstev_work(
    layout: Layout,
    jobz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dstev_work(
        layout.into(),
        jobz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn sstevd_work(
    layout: Layout,
    jobz: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_sstevd_work(
        layout.into(),
        jobz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dstevd_work(
    layout: Layout,
    jobz: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dstevd_work(
        layout.into(),
        jobz as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn sstevr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_sstevr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dstevr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dstevr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn sstevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f32],
    e: &mut [f32],
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_sstevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dstevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    n: i32,
    d: &mut [f64],
    e: &mut [f64],
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dstevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        n,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssycon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssycon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsycon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsycon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csycon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    anorm: f32,
    rcond: &mut f32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_csycon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zsycon_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    anorm: f64,
    rcond: &mut f64,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zsycon_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        anorm,
        rcond,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ssyequb_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssyequb_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyequb_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsyequb_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr(),
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csyequb_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    s: &mut [f32],
    scond: &mut [f32],
    amax: &mut f32,
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_csyequb_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zsyequb_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    s: &mut [f64],
    scond: &mut [f64],
    amax: &mut f64,
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zsyequb_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        s.as_mut_ptr(),
        scond.as_mut_ptr(),
        amax,
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ssyev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_ssyev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dsyev_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dsyev_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn ssyevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_ssyevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dsyevd_work(
    layout: Layout,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dsyevd_work(
        layout.into(),
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn ssyevr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_ssyevr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dsyevr_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    isuppz: &mut [i32],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dsyevr_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        isuppz.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn ssyevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssyevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyevx_work(
    layout: Layout,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsyevx_work(
        layout.into(),
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssygst_work(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ssygst_work(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dsygst_work(
    layout: Layout,
    itype: i32,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dsygst_work(
        layout.into(),
        itype,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ssygv_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_ssygv_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dsygv_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dsygv_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn ssygvd_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    w: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_ssygvd_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dsygvd_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    w: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dsygvd_work(
        layout.into(),
        itype,
        jobz as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        w.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn ssygvx_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    vl: f32,
    vu: f32,
    il: i32,
    iu: i32,
    abstol: f32,
    m: &mut i32,
    w: &mut [f32],
    z: &mut [f32],
    ldz: i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssygvx_work(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsygvx_work(
    layout: Layout,
    itype: i32,
    jobz: u8,
    range: u8,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    vl: f64,
    vu: f64,
    il: i32,
    iu: i32,
    abstol: f64,
    m: &mut i32,
    w: &mut [f64],
    z: &mut [f64],
    ldz: i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    ifail: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsygvx_work(
        layout.into(),
        itype,
        jobz as c_char,
        range as c_char,
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        vl,
        vu,
        il,
        iu,
        abstol,
        m,
        w.as_mut_ptr(),
        z.as_mut_ptr(),
        ldz,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        ifail.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssyrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssyrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsyrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csyrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csyrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsyrfs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsyrfs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssyrfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &[f32],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssyrfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsyrfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &[f64],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f64],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsyrfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_ptr(),
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csyrfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &[c32],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csyrfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsyrfsx_work(
    layout: Layout,
    uplo: u8,
    equed: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &[c64],
    ldaf: i32,
    ipiv: &[i32],
    s: &[f64],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsyrfsx_work(
        layout.into(),
        uplo as c_char,
        equed as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_ptr() as *const _,
        ldaf,
        ipiv.as_ptr(),
        s.as_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssysv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f32],
    ldb: i32,
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_ssysv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dsysv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [f64],
    ldb: i32,
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dsysv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn csysv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c32],
    ldb: i32,
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_csysv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zsysv_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    b: &mut [c64],
    ldb: i32,
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zsysv_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn ssysvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssysvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsysvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsysvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csysvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    lwork: i32,
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csysvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsysvx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    b: &[c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    lwork: i32,
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsysvx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        b.as_ptr() as *const _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssysvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f32],
    lda: i32,
    af: &mut [f32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [f32],
    ldb: i32,
    x: &mut [f32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ssysvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsysvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [f64],
    lda: i32,
    af: &mut [f64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [f64],
    ldb: i32,
    x: &mut [f64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dsysvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr(),
        lda,
        af.as_mut_ptr(),
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr(),
        ldb,
        x.as_mut_ptr(),
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csysvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c32],
    lda: i32,
    af: &mut [c32],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f32],
    b: &mut [c32],
    ldb: i32,
    x: &mut [c32],
    ldx: i32,
    rcond: &mut f32,
    rpvgrw: &mut f32,
    berr: &mut [f32],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f32],
    err_bnds_comp: &mut [f32],
    nparams: i32,
    params: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_csysvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn zsysvxx_work(
    layout: Layout,
    fact: u8,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &mut [c64],
    lda: i32,
    af: &mut [c64],
    ldaf: i32,
    ipiv: &mut [i32],
    equed: &mut u8,
    s: &mut [f64],
    b: &mut [c64],
    ldb: i32,
    x: &mut [c64],
    ldx: i32,
    rcond: &mut f64,
    rpvgrw: &mut f64,
    berr: &mut [f64],
    n_err_bnds: i32,
    err_bnds_norm: &mut [f64],
    err_bnds_comp: &mut [f64],
    nparams: i32,
    params: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_zsysvxx_work(
        layout.into(),
        fact as c_char,
        uplo as c_char,
        n,
        nrhs,
        a.as_mut_ptr() as *mut _,
        lda,
        af.as_mut_ptr() as *mut _,
        ldaf,
        ipiv.as_mut_ptr(),
        equed as *mut _ as *mut _,
        s.as_mut_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
        x.as_mut_ptr() as *mut _,
        ldx,
        rcond,
        rpvgrw,
        berr.as_mut_ptr(),
        n_err_bnds,
        err_bnds_norm.as_mut_ptr(),
        err_bnds_comp.as_mut_ptr(),
        nparams,
        params.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ssytrd_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    d: &mut [f32],
    e: &mut [f32],
    tau: &mut [f32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_ssytrd_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dsytrd_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    d: &mut [f64],
    e: &mut [f64],
    tau: &mut [f64],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dsytrd_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        d.as_mut_ptr(),
        e.as_mut_ptr(),
        tau.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn ssytrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_ssytrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dsytrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dsytrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn csytrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_csytrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn zsytrf_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &mut [i32],
    work: &mut [c64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_zsytrf_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        lwork,
    )
}

#[inline]
pub unsafe fn ssytri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ssytri_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dsytri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [f64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dsytri_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr(),
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn csytri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c32],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c32],
) -> i32 {
    ffi::LAPACKE_csytri_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn zsytri_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    a: &mut [c64],
    lda: i32,
    ipiv: &[i32],
    work: &mut [c64],
) -> i32 {
    ffi::LAPACKE_zsytri_work(
        layout.into(),
        uplo as c_char,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        ipiv.as_ptr(),
        work.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ssytrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ssytrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dsytrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[f64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dsytrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr(),
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn csytrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c32],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_csytrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn zsytrs_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    nrhs: i32,
    a: &[c64],
    lda: i32,
    ipiv: &[i32],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_zsytrs_work(
        layout.into(),
        uplo as c_char,
        n,
        nrhs,
        a.as_ptr() as *const _,
        lda,
        ipiv.as_ptr(),
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn stbcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[f32],
    ldab: i32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stbcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtbcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[f64],
    ldab: i32,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtbcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        kd,
        ab.as_ptr(),
        ldab,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctbcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[c32],
    ldab: i32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctbcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztbcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    kd: i32,
    ab: &[c64],
    ldab: i32,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztbcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        kd,
        ab.as_ptr() as *const _,
        ldab,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stbrfs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &[f32],
    ldb: i32,
    x: &[f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stbrfs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtbrfs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &[f64],
    ldb: i32,
    x: &[f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtbrfs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctbrfs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &[c32],
    ldb: i32,
    x: &[c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctbrfs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztbrfs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &[c64],
    ldb: i32,
    x: &[c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztbrfs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stbtrs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f32],
    ldab: i32,
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_stbtrs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dtbtrs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[f64],
    ldab: i32,
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dtbtrs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr(),
        ldab,
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ctbtrs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c32],
    ldab: i32,
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ctbtrs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ztbtrs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    kd: i32,
    nrhs: i32,
    ab: &[c64],
    ldab: i32,
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ztbtrs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        kd,
        nrhs,
        ab.as_ptr() as *const _,
        ldab,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn stfsm_work(
    layout: Layout,
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: f32,
    a: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_stfsm_work(
        layout.into(),
        transr as c_char,
        side as c_char,
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        m,
        n,
        alpha,
        a.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dtfsm_work(
    layout: Layout,
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: f64,
    a: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dtfsm_work(
        layout.into(),
        transr as c_char,
        side as c_char,
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        m,
        n,
        alpha,
        a.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ctfsm_work(
    layout: Layout,
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: c32,
    a: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ctfsm_work(
        layout.into(),
        transr as c_char,
        side as c_char,
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        m,
        n,
        transmute(alpha),
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ztfsm_work(
    layout: Layout,
    transr: u8,
    side: u8,
    uplo: u8,
    trans: u8,
    diag: u8,
    m: i32,
    n: i32,
    alpha: c64,
    a: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ztfsm_work(
        layout.into(),
        transr as c_char,
        side as c_char,
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        m,
        n,
        transmute(alpha),
        a.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn stftri_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stftri_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtftri_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtftri_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctftri_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &mut [c32],
) -> i32 {
    ffi::LAPACKE_ctftri_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztftri_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &mut [c64],
) -> i32 {
    ffi::LAPACKE_ztftri_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stfttp_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f32],
    ap: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stfttp_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr(),
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtfttp_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f64],
    ap: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtfttp_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr(),
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctfttp_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c32],
    ap: &mut [c32],
) -> i32 {
    ffi::LAPACKE_ctfttp_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr() as *const _,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztfttp_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c64],
    ap: &mut [c64],
) -> i32 {
    ffi::LAPACKE_ztfttp_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr() as *const _,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stfttr_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f32],
    a: &mut [f32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_stfttr_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr(),
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dtfttr_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[f64],
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_dtfttr_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr(),
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn ctfttr_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c32],
    a: &mut [c32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_ctfttr_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn ztfttr_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    arf: &[c64],
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_ztfttr_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        arf.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn stgevc_work(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[f32],
    lds: i32,
    p: &[f32],
    ldp: i32,
    vl: &mut f32,
    ldvl: i32,
    vr: &mut f32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stgevc_work(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        s.as_ptr(),
        lds,
        p.as_ptr(),
        ldp,
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtgevc_work(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[f64],
    lds: i32,
    p: &[f64],
    ldp: i32,
    vl: &mut f64,
    ldvl: i32,
    vr: &mut f64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtgevc_work(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        s.as_ptr(),
        lds,
        p.as_ptr(),
        ldp,
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctgevc_work(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[c32],
    lds: i32,
    p: &[c32],
    ldp: i32,
    vl: &mut c32,
    ldvl: i32,
    vr: &mut c32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctgevc_work(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        s.as_ptr() as *const _,
        lds,
        p.as_ptr() as *const _,
        ldp,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztgevc_work(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    s: &[c64],
    lds: i32,
    p: &[c64],
    ldp: i32,
    vl: &mut c64,
    ldvl: i32,
    vr: &mut c64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztgevc_work(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        s.as_ptr() as *const _,
        lds,
        p.as_ptr() as *const _,
        ldp,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stgexc_work(
    layout: Layout,
    wantq: i32,
    wantz: i32,
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
    work: &mut [f32],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_stgexc_work(
        layout.into(),
        wantq,
        wantz,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn dtgexc_work(
    layout: Layout,
    wantq: i32,
    wantz: i32,
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    ifst: &mut [i32],
    ilst: &mut [i32],
    work: &mut [f64],
    lwork: i32,
) -> i32 {
    ffi::LAPACKE_dtgexc_work(
        layout.into(),
        wantq,
        wantz,
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        ifst.as_mut_ptr(),
        ilst.as_mut_ptr(),
        work.as_mut_ptr(),
        lwork,
    )
}

#[inline]
pub unsafe fn ctgexc_work(
    layout: Layout,
    wantq: i32,
    wantz: i32,
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    ifst: i32,
    ilst: i32,
) -> i32 {
    ffi::LAPACKE_ctgexc_work(
        layout.into(),
        wantq,
        wantz,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        ifst,
        ilst,
    )
}

#[inline]
pub unsafe fn ztgexc_work(
    layout: Layout,
    wantq: i32,
    wantz: i32,
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    ifst: i32,
    ilst: i32,
) -> i32 {
    ffi::LAPACKE_ztgexc_work(
        layout.into(),
        wantq,
        wantz,
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        ifst,
        ilst,
    )
}

#[inline]
pub unsafe fn stgsen_work(
    layout: Layout,
    ijob: i32,
    wantq: i32,
    wantz: i32,
    select: &[i32],
    n: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    alphar: &mut [f32],
    alphai: &mut [f32],
    beta: &mut [f32],
    q: &mut f32,
    ldq: i32,
    z: &mut [f32],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f32],
    pr: &mut [f32],
    dif: &mut f32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_stgsen_work(
        layout.into(),
        ijob,
        wantq,
        wantz,
        select.as_ptr(),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn dtgsen_work(
    layout: Layout,
    ijob: i32,
    wantq: i32,
    wantz: i32,
    select: &[i32],
    n: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    alphar: &mut [f64],
    alphai: &mut [f64],
    beta: &mut [f64],
    q: &mut f64,
    ldq: i32,
    z: &mut [f64],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f64],
    pr: &mut [f64],
    dif: &mut f64,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_dtgsen_work(
        layout.into(),
        ijob,
        wantq,
        wantz,
        select.as_ptr(),
        n,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        alphar.as_mut_ptr(),
        alphai.as_mut_ptr(),
        beta.as_mut_ptr(),
        q,
        ldq,
        z.as_mut_ptr(),
        ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn ctgsen_work(
    layout: Layout,
    ijob: i32,
    wantq: i32,
    wantz: i32,
    select: &[i32],
    n: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    alpha: &mut [c32],
    beta: &mut [c32],
    q: &mut c32,
    ldq: i32,
    z: &mut [c32],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f32],
    pr: &mut [f32],
    dif: &mut f32,
    work: &mut [c32],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_ctgsen_work(
        layout.into(),
        ijob,
        wantq,
        wantz,
        select.as_ptr(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif,
        work.as_mut_ptr() as *mut _,
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn ztgsen_work(
    layout: Layout,
    ijob: i32,
    wantq: i32,
    wantz: i32,
    select: &[i32],
    n: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    alpha: &mut [c64],
    beta: &mut [c64],
    q: &mut c64,
    ldq: i32,
    z: &mut [c64],
    ldz: i32,
    m: &mut i32,
    pl: &mut [f64],
    pr: &mut [f64],
    dif: &mut f64,
    work: &mut [c64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
) -> i32 {
    ffi::LAPACKE_ztgsen_work(
        layout.into(),
        ijob,
        wantq,
        wantz,
        select.as_ptr(),
        n,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        alpha.as_mut_ptr() as *mut _,
        beta.as_mut_ptr() as *mut _,
        q as *mut _ as *mut _,
        ldq,
        z.as_mut_ptr() as *mut _,
        ldz,
        m,
        pl.as_mut_ptr(),
        pr.as_mut_ptr(),
        dif,
        work.as_mut_ptr() as *mut _,
        lwork,
        iwork.as_mut_ptr(),
        liwork,
    )
}

#[inline]
pub unsafe fn stgsja_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [f32],
    ldu: i32,
    v: &mut [f32],
    ldv: i32,
    q: &mut f32,
    ldq: i32,
    work: &mut [f32],
    ncycle: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stgsja_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        work.as_mut_ptr(),
        ncycle.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtgsja_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [f64],
    lda: i32,
    b: &mut [f64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [f64],
    ldu: i32,
    v: &mut [f64],
    ldv: i32,
    q: &mut f64,
    ldq: i32,
    work: &mut [f64],
    ncycle: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtgsja_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        k,
        l,
        a.as_mut_ptr(),
        lda,
        b.as_mut_ptr(),
        ldb,
        tola,
        tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr(),
        ldu,
        v.as_mut_ptr(),
        ldv,
        q,
        ldq,
        work.as_mut_ptr(),
        ncycle.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctgsja_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [c32],
    lda: i32,
    b: &mut [c32],
    ldb: i32,
    tola: f32,
    tolb: f32,
    alpha: &mut [f32],
    beta: &mut [f32],
    u: &mut [c32],
    ldu: i32,
    v: &mut [c32],
    ldv: i32,
    q: &mut c32,
    ldq: i32,
    work: &mut [c32],
    ncycle: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ctgsja_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
        ncycle.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztgsja_work(
    layout: Layout,
    jobu: u8,
    jobv: u8,
    jobq: u8,
    m: i32,
    p: i32,
    n: i32,
    k: i32,
    l: i32,
    a: &mut [c64],
    lda: i32,
    b: &mut [c64],
    ldb: i32,
    tola: f64,
    tolb: f64,
    alpha: &mut [f64],
    beta: &mut [f64],
    u: &mut [c64],
    ldu: i32,
    v: &mut [c64],
    ldv: i32,
    q: &mut c64,
    ldq: i32,
    work: &mut [c64],
    ncycle: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ztgsja_work(
        layout.into(),
        jobu as c_char,
        jobv as c_char,
        jobq as c_char,
        m,
        p,
        n,
        k,
        l,
        a.as_mut_ptr() as *mut _,
        lda,
        b.as_mut_ptr() as *mut _,
        ldb,
        tola,
        tolb,
        alpha.as_mut_ptr(),
        beta.as_mut_ptr(),
        u.as_mut_ptr() as *mut _,
        ldu,
        v.as_mut_ptr() as *mut _,
        ldv,
        q as *mut _ as *mut _,
        ldq,
        work.as_mut_ptr() as *mut _,
        ncycle.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stgsna_work(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    vl: &[f32],
    ldvl: i32,
    vr: &[f32],
    ldvr: i32,
    s: &mut [f32],
    dif: &mut f32,
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stgsna_work(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        vl.as_ptr(),
        ldvl,
        vr.as_ptr(),
        ldvr,
        s.as_mut_ptr(),
        dif,
        mm,
        m,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtgsna_work(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    vl: &[f64],
    ldvl: i32,
    vr: &[f64],
    ldvr: i32,
    s: &mut [f64],
    dif: &mut f64,
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtgsna_work(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        vl.as_ptr(),
        ldvl,
        vr.as_ptr(),
        ldvr,
        s.as_mut_ptr(),
        dif,
        mm,
        m,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctgsna_work(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    vl: &[c32],
    ldvl: i32,
    vr: &[c32],
    ldvr: i32,
    s: &mut [f32],
    dif: &mut f32,
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ctgsna_work(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        vl.as_ptr() as *const _,
        ldvl,
        vr.as_ptr() as *const _,
        ldvr,
        s.as_mut_ptr(),
        dif,
        mm,
        m,
        work.as_mut_ptr() as *mut _,
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztgsna_work(
    layout: Layout,
    job: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    vl: &[c64],
    ldvl: i32,
    vr: &[c64],
    ldvr: i32,
    s: &mut [f64],
    dif: &mut f64,
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ztgsna_work(
        layout.into(),
        job as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        vl.as_ptr() as *const _,
        ldvl,
        vr.as_ptr() as *const _,
        ldvr,
        s.as_mut_ptr(),
        dif,
        mm,
        m,
        work.as_mut_ptr() as *mut _,
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stgsyl_work(
    layout: Layout,
    trans: u8,
    ijob: i32,
    m: i32,
    n: i32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    c: &mut [f32],
    ldc: i32,
    d: &[f32],
    ldd: i32,
    e: &[f32],
    lde: i32,
    f: &mut [f32],
    ldf: i32,
    scale: &mut [f32],
    dif: &mut f32,
    work: &mut [f32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stgsyl_work(
        layout.into(),
        trans as c_char,
        ijob,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        c.as_mut_ptr(),
        ldc,
        d.as_ptr(),
        ldd,
        e.as_ptr(),
        lde,
        f.as_mut_ptr(),
        ldf,
        scale.as_mut_ptr(),
        dif,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtgsyl_work(
    layout: Layout,
    trans: u8,
    ijob: i32,
    m: i32,
    n: i32,
    a: &[f64],
    lda: i32,
    b: &[f64],
    ldb: i32,
    c: &mut [f64],
    ldc: i32,
    d: &[f64],
    ldd: i32,
    e: &[f64],
    lde: i32,
    f: &mut [f64],
    ldf: i32,
    scale: &mut [f64],
    dif: &mut f64,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtgsyl_work(
        layout.into(),
        trans as c_char,
        ijob,
        m,
        n,
        a.as_ptr(),
        lda,
        b.as_ptr(),
        ldb,
        c.as_mut_ptr(),
        ldc,
        d.as_ptr(),
        ldd,
        e.as_ptr(),
        lde,
        f.as_mut_ptr(),
        ldf,
        scale.as_mut_ptr(),
        dif,
        work.as_mut_ptr(),
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctgsyl_work(
    layout: Layout,
    trans: u8,
    ijob: i32,
    m: i32,
    n: i32,
    a: &[c32],
    lda: i32,
    b: &[c32],
    ldb: i32,
    c: &mut [c32],
    ldc: i32,
    d: &[c32],
    ldd: i32,
    e: &[c32],
    lde: i32,
    f: &mut [c32],
    ldf: i32,
    scale: &mut [f32],
    dif: &mut f32,
    work: &mut [c32],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ctgsyl_work(
        layout.into(),
        trans as c_char,
        ijob,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        c.as_mut_ptr() as *mut _,
        ldc,
        d.as_ptr() as *const _,
        ldd,
        e.as_ptr() as *const _,
        lde,
        f.as_mut_ptr() as *mut _,
        ldf,
        scale.as_mut_ptr(),
        dif,
        work.as_mut_ptr() as *mut _,
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztgsyl_work(
    layout: Layout,
    trans: u8,
    ijob: i32,
    m: i32,
    n: i32,
    a: &[c64],
    lda: i32,
    b: &[c64],
    ldb: i32,
    c: &mut [c64],
    ldc: i32,
    d: &[c64],
    ldd: i32,
    e: &[c64],
    lde: i32,
    f: &mut [c64],
    ldf: i32,
    scale: &mut [f64],
    dif: &mut f64,
    work: &mut [c64],
    lwork: i32,
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_ztgsyl_work(
        layout.into(),
        trans as c_char,
        ijob,
        m,
        n,
        a.as_ptr() as *const _,
        lda,
        b.as_ptr() as *const _,
        ldb,
        c.as_mut_ptr() as *mut _,
        ldc,
        d.as_ptr() as *const _,
        ldd,
        e.as_ptr() as *const _,
        lde,
        f.as_mut_ptr() as *mut _,
        ldf,
        scale.as_mut_ptr(),
        dif,
        work.as_mut_ptr() as *mut _,
        lwork,
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stpcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[f32],
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stpcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_ptr(),
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtpcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[f64],
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtpcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_ptr(),
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctpcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[c32],
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctpcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_ptr() as *const _,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztpcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    ap: &[c64],
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztpcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_ptr() as *const _,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stprfs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &[f32],
    ldb: i32,
    x: &[f32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_stprfs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtprfs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &[f64],
    ldb: i32,
    x: &[f64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtprfs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_ptr(),
        ldb,
        x.as_ptr(),
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctprfs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &[c32],
    ldb: i32,
    x: &[c32],
    ldx: i32,
    ferr: &mut [f32],
    berr: &mut [f32],
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctprfs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztprfs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &[c64],
    ldb: i32,
    x: &[c64],
    ldx: i32,
    ferr: &mut [f64],
    berr: &mut [f64],
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztprfs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_ptr() as *const _,
        ldb,
        x.as_ptr() as *const _,
        ldx,
        ferr.as_mut_ptr(),
        berr.as_mut_ptr(),
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn stptri_work(layout: Layout, uplo: u8, diag: u8, n: i32, ap: &mut [f32]) -> i32 {
    ffi::LAPACKE_stptri_work(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtptri_work(layout: Layout, uplo: u8, diag: u8, n: i32, ap: &mut [f64]) -> i32 {
    ffi::LAPACKE_dtptri_work(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctptri_work(layout: Layout, uplo: u8, diag: u8, n: i32, ap: &mut [c32]) -> i32 {
    ffi::LAPACKE_ctptri_work(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztptri_work(layout: Layout, uplo: u8, diag: u8, n: i32, ap: &mut [c64]) -> i32 {
    ffi::LAPACKE_ztptri_work(
        layout.into(),
        uplo as c_char,
        diag as c_char,
        n,
        ap.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stptrs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f32],
    b: &mut [f32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_stptrs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn dtptrs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[f64],
    b: &mut [f64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_dtptrs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr(),
        b.as_mut_ptr(),
        ldb,
    )
}

#[inline]
pub unsafe fn ctptrs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c32],
    b: &mut [c32],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ctptrs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn ztptrs_work(
    layout: Layout,
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    nrhs: i32,
    ap: &[c64],
    b: &mut [c64],
    ldb: i32,
) -> i32 {
    ffi::LAPACKE_ztptrs_work(
        layout.into(),
        uplo as c_char,
        trans as c_char,
        diag as c_char,
        n,
        nrhs,
        ap.as_ptr() as *const _,
        b.as_mut_ptr() as *mut _,
        ldb,
    )
}

#[inline]
pub unsafe fn stpttf_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    ap: &[f32],
    arf: &mut [f32],
) -> i32 {
    ffi::LAPACKE_stpttf_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        ap.as_ptr(),
        arf.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtpttf_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    ap: &[f64],
    arf: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtpttf_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        ap.as_ptr(),
        arf.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctpttf_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    ap: &[c32],
    arf: &mut [c32],
) -> i32 {
    ffi::LAPACKE_ctpttf_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        arf.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn ztpttf_work(
    layout: Layout,
    transr: u8,
    uplo: u8,
    n: i32,
    ap: &[c64],
    arf: &mut [c64],
) -> i32 {
    ffi::LAPACKE_ztpttf_work(
        layout.into(),
        transr as c_char,
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        arf.as_mut_ptr() as *mut _,
    )
}

#[inline]
pub unsafe fn stpttr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f32],
    a: &mut [f32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_stpttr_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn dtpttr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[f64],
    a: &mut [f64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_dtpttr_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr(),
        a.as_mut_ptr(),
        lda,
    )
}

#[inline]
pub unsafe fn ctpttr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c32],
    a: &mut [c32],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_ctpttr_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn ztpttr_work(
    layout: Layout,
    uplo: u8,
    n: i32,
    ap: &[c64],
    a: &mut [c64],
    lda: i32,
) -> i32 {
    ffi::LAPACKE_ztpttr_work(
        layout.into(),
        uplo as c_char,
        n,
        ap.as_ptr() as *const _,
        a.as_mut_ptr() as *mut _,
        lda,
    )
}

#[inline]
pub unsafe fn strcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[f32],
    lda: i32,
    rcond: &mut f32,
    work: &mut [f32],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_strcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_ptr(),
        lda,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtrcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[f64],
    lda: i32,
    rcond: &mut f64,
    work: &mut [f64],
    iwork: &mut [i32],
) -> i32 {
    ffi::LAPACKE_dtrcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_ptr(),
        lda,
        rcond,
        work.as_mut_ptr(),
        iwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctrcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[c32],
    lda: i32,
    rcond: &mut f32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctrcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztrcon_work(
    layout: Layout,
    norm: u8,
    uplo: u8,
    diag: u8,
    n: i32,
    a: &[c64],
    lda: i32,
    rcond: &mut f64,
    work: &mut [c64],
    rwork: &mut [f64],
) -> i32 {
    ffi::LAPACKE_ztrcon_work(
        layout.into(),
        norm as c_char,
        uplo as c_char,
        diag as c_char,
        n,
        a.as_ptr() as *const _,
        lda,
        rcond,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn strevc_work(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &mut [i32],
    n: i32,
    t: &[f32],
    ldt: i32,
    vl: &mut f32,
    ldvl: i32,
    vr: &mut f32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f32],
) -> i32 {
    ffi::LAPACKE_strevc_work(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_mut_ptr(),
        n,
        t.as_ptr(),
        ldt,
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn dtrevc_work(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &mut [i32],
    n: i32,
    t: &[f64],
    ldt: i32,
    vl: &mut f64,
    ldvl: i32,
    vr: &mut f64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [f64],
) -> i32 {
    ffi::LAPACKE_dtrevc_work(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_mut_ptr(),
        n,
        t.as_ptr(),
        ldt,
        vl,
        ldvl,
        vr,
        ldvr,
        mm,
        m,
        work.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ctrevc_work(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &mut [c32],
    ldt: i32,
    vl: &mut c32,
    ldvl: i32,
    vr: &mut c32,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c32],
    rwork: &mut [f32],
) -> i32 {
    ffi::LAPACKE_ctrevc_work(
        layout.into(),
        side as c_char,
        howmny as c_char,
        select.as_ptr(),
        n,
        t.as_mut_ptr() as *mut _,
        ldt,
        vl as *mut _ as *mut _,
        ldvl,
        vr as *mut _ as *mut _,
        ldvr,
        mm,
        m,
        work.as_mut_ptr() as *mut _,
        rwork.as_mut_ptr(),
    )
}

#[inline]
pub unsafe fn ztrevc_work(
    layout: Layout,
    side: u8,
    howmny: u8,
    select: &[i32],
    n: i32,
    t: &mut [c64],
    ldt: i32,
    vl: &mut c64,
    ldvl: i32,
    vr: &mut c64,
    ldvr: i32,
    mm: i32,
    m: &mut i32,
    work: &mut [c64],
    rwork: &mut [f64],