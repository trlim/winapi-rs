// Copyright © 2016, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
use shared::minwindef::{WORD};
use shared::wtypes::{VARTYPE};
use um::winnt::HRESULT;

STRUCT!{struct PROPVARIANT {
    vt: VARTYPE,
    wReserved1: WORD,
    wReserved2: WORD,
    wReserved3: WORD,
    data: [u8; 16],
}}
pub type REFPROPVARIANT = *const PROPVARIANT;

EXTERN!{stdcall fn PropVariantCopy(
    pvarDest: *mut PROPVARIANT,
    pvarSrc: *const PROPVARIANT
) -> HRESULT}
EXTERN!{stdcall fn PropVariantClear(
    pvar: *mut PROPVARIANT
) -> HRESULT}
#[inline]
pub fn PropVariantInit(pvar: *mut PROPVARIANT) {
    // unsafe { libc::memset(); }
    // *pvar = unsafe { mem::zeroed() };
}
