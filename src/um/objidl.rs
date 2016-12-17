// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! this ALWAYS GENERATED file contains the definitions for the interfaces
use ctypes::c_void;
use shared::basetsd::UINT64;
use shared::guiddef::{CLSID, REFIID};
use shared::minwindef::{BOOL, DWORD, FILETIME, ULONG};
use shared::wtypesbase::{OLECHAR, LPOLESTR};
use um::objidlbase::{IEnumString, IStream};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, ULARGE_INTEGER, VOID};
//8402
STRUCT!{struct BIND_OPTS {
    cbStruct: DWORD,
    grfFlags: DWORD,
    grfMode: DWORD,
    dwTickCountDeadline: DWORD,
}}
pub type LPBIND_OPTS = *mut BIND_OPTS;
//8479
RIDL!(
interface IBindCtx(IBindCtxVtbl): IUnknown(IUnknownVtbl) {
    fn RegisterObjectBound(
        &mut self,
        punk: *mut IUnknown
    ) -> HRESULT,
    fn RevokeObjectBound(
        &mut self,
        punk: *mut IUnknown
    ) -> HRESULT,
    fn ReleaseBoundObjects(
        &mut self
    ) -> HRESULT,
    fn SetBindOptions(
        &mut self,
        pbindopts: *mut BIND_OPTS
    ) -> HRESULT,
    fn GetBindOptions(
        &mut self,
        pbindopts: *mut BIND_OPTS
    ) -> HRESULT,
    fn GetRunningObjectTable(
        &mut self,
        pprot: *mut *mut IRunningObjectTable
    ) -> HRESULT,
    fn RegisterObjectParam(
        &mut self,
        pszKey: LPOLESTR,
        punk: *mut IUnknown
    ) -> HRESULT,
    fn GetObjectParam(
        &mut self,
        pszKey: LPOLESTR,
        ppunk: *mut *mut IUnknown
    ) -> HRESULT,
    fn EnumObjectParam(
        &mut self,
        ppenum: *mut *mut IEnumString
    ) -> HRESULT,
    fn RevokeObjectParam(
        &mut self,
        pszKey: LPOLESTR
    ) -> HRESULT
}
);
//8681
RIDL!(
interface IEnumMoniker(IEnumMonikerVtbl): IUnknown(IUnknownVtbl) {
    fn Next( 
        &mut self,
        celt: ULONG,
        rgelt: *mut *mut IMoniker,
        pceltFetched: *mut ULONG
    ) -> HRESULT,
    fn Skip( 
        &mut self,
        celt: ULONG
    ) -> HRESULT,
    fn Reset(
        &mut self
    ) -> HRESULT,
    fn Clone( 
        &mut self,
        ppenum: *mut *mut IEnumMoniker
    ) -> HRESULT
}
);
//8958
RIDL!(
interface IRunningObjectTable(IRunningObjectTableVtbl): IUnknown(IUnknownVtbl) {
    fn Register(
        &mut self,
        grfFlags: DWORD,
        punkObject: *mut IUnknown,
        pmkObjectName: *mut IMoniker,
        pdwRegister: *mut DWORD
    ) -> HRESULT,
    fn Revoke(
        &mut self,
        dwRegister: DWORD
    ) -> HRESULT,
    fn IsRunning(
        &mut self,
        pmkObjectName: *mut IMoniker
    ) -> HRESULT,
    fn GetObject(
        &mut self,
        pmkObjectName: *mut IMoniker,
        ppunkObject: *mut *mut IUnknown
    ) -> HRESULT,
    fn NoteChangeTime(
        &mut self,
        dwRegister: DWORD,
        pfiletime: *mut FILETIME
    ) -> HRESULT,
    fn GetTimeOfLastChange(
        &mut self,
        pmkObjectName: *mut IMoniker,
        pfiletime: *mut FILETIME
    ) -> HRESULT,
    fn EnumRunning(
        &mut self,
        ppenumMoniker: *mut *mut IEnumMoniker
    ) -> HRESULT
}
);
//9125
RIDL!(
interface IPersist(IPersistVtbl): IUnknown(IUnknownVtbl) {
    fn GetClassID( 
        &mut self,
        pClassID: *mut CLSID
    ) -> HRESULT
}
);
//9207
RIDL!(
interface IPersistStream(IPersistStreamVtbl): IPersist(IPersistVtbl) {
    fn IsDirty(
        &mut self
    ) -> HRESULT,
    fn Load( 
        &mut self,
        pStm: *mut IStream
    ) -> HRESULT,
    fn Save( 
        &mut self,
        pStm: *mut IStream,
        fClearDirty: BOOL
    ) -> HRESULT,
    fn GetSizeMax( 
        &mut self,
        pcbSize: *mut ULARGE_INTEGER
    ) -> HRESULT
}
);
//9350
RIDL!(
interface IMoniker(IMonikerVtbl): IPersistStream(IPersistStreamVtbl) {
    fn BindToObject( 
        &mut self,
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        riidResult: REFIID,
        ppvResult: *mut *mut c_void
    ) -> HRESULT,
    fn BindToStorage( 
        &mut self,
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        riid: REFIID,
        ppvObj: *mut *mut c_void
    ) -> HRESULT,
    fn Reduce( 
        &mut self,
        pbc: *mut IBindCtx,
        dwReduceHowFar: DWORD,
        ppmkToLeft: *mut *mut IMoniker,
        ppmkReduced: *mut *mut IMoniker
    ) -> HRESULT,
    fn ComposeWith( 
        &mut self,
        pmkRight: *mut IMoniker,
        fOnlyIfNotGeneric: BOOL,
        ppmkComposite: *mut *mut IMoniker
    ) -> HRESULT,
    fn Enum( 
        &mut self,
        fForward: BOOL,
        ppenumMoniker: *mut *mut IEnumMoniker
    ) -> HRESULT,
    fn IsEqual( 
        &mut self,
        pmkOtherMoniker: *mut IMoniker
    ) -> HRESULT,
    fn Hash( 
        &mut self,
        pdwHash: *mut DWORD
    ) -> HRESULT,
    fn IsRunning( 
        &mut self,
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        pmkNewlyRunning: *mut IMoniker
    ) -> HRESULT,
    fn GetTimeOfLastChange( 
        &mut self,
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        pFileTime: *mut FILETIME
    ) -> HRESULT,
    fn Inverse( 
        &mut self,
        ppmk: *mut *mut IMoniker
    ) -> HRESULT,
    fn CommonPrefixWith( 
        &mut self,
        pmkOther: *mut IMoniker,
        ppmkPrefix: *mut *mut IMoniker
    ) -> HRESULT,
    fn RelativePathTo( 
        &mut self,
        pmkOther: *mut IMoniker,
        ppmkRelPath: *mut *mut IMoniker
    ) -> HRESULT,
    fn GetDisplayName( 
        &mut self,
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        ppszDisplayName: *mut LPOLESTR
    ) -> HRESULT,
    fn ParseDisplayName( 
        &mut self,
        pbc: *mut IBindCtx,
        pmkToLeft: *mut IMoniker,
        pszDisplayName: LPOLESTR,
        pchEaten: *mut ULONG,
        ppmkOut: *mut *mut IMoniker
    ) -> HRESULT,
    fn IsSystemMoniker( 
        &mut self,
        pdwMksys: *mut DWORD
    ) -> HRESULT
}
);
ENUM!{enum EOLE_AUTHENTICATION_CAPABILITIES {
    EOAC_NONE = 0,
    EOAC_MUTUAL_AUTH = 0x1,
    EOAC_STATIC_CLOAKING = 0x20,
    EOAC_DYNAMIC_CLOAKING = 0x40,
    EOAC_ANY_AUTHORITY = 0x80,
    EOAC_MAKE_FULLSIC = 0x100,
    EOAC_DEFAULT = 0x800,
    EOAC_SECURE_REFS = 0x2,
    EOAC_ACCESS_CONTROL = 0x4,
    EOAC_APPID = 0x8,
    EOAC_DYNAMIC = 0x10,
    EOAC_REQUIRE_FULLSIC = 0x200,
    EOAC_AUTO_IMPERSONATE = 0x400,
    EOAC_NO_CUSTOM_MARSHAL = 0x2000,
    EOAC_DISABLE_AAA = 0x1000,
}}
STRUCT!{struct SOLE_AUTHENTICATION_SERVICE {
    dwAuthnSvc: DWORD,
    dwAuthzSvc: DWORD,
    pPrincipalName: *mut OLECHAR,
    hr: HRESULT,
}}

RIDL!(
interface IApartmentShutdown(IApartmentShutdownVtbl): IUnknown(IUnknownVtbl) {
    fn OnUninitialize(
        &mut self,
        ui64ApartmentIdentifier: UINT64
    ) -> VOID
}
);

RIDL!(
interface IMarshal(IMarshalVtbl): IUnknown(IUnknownVtbl) {
    fn GetUnmarshalClass(
        &mut self,
        riid: REFIID,
        pv: *mut c_void,
        dwDestContext: DWORD,
        pvDestContext: *mut c_void,
        mshlflags: DWORD,
        pCid: *mut CLSID
    ) -> HRESULT,
    fn GetMarshalSizeMax(
        &mut self,
        riid: REFIID,
        pv: *mut c_void,
        dwDestContext: DWORD,
        pvDestContext: *mut c_void,
        mshlflags: DWORD,
        pSize: *mut DWORD
    ) -> HRESULT,
    fn MarshalInterface(
        &mut self,
        pStm: *mut IStream,
        riid: REFIID,
        pv: *mut c_void,
        dwDestContext: DWORD,
        pvDestContext: *mut c_void,
        mshlflags: DWORD
    ) -> HRESULT,
    fn UnmarshalInterface(
        &mut self,
        pStm: *mut IStream,
        riid: REFIID,
        ppv: *mut *mut c_void
    ) -> HRESULT,
    fn ReleaseMarshalData(
        &mut self,
        pStm: *mut IStream
    ) -> HRESULT,
    fn DisconnectObject(
        &mut self,
        dwReserved: DWORD
    ) -> HRESULT
}
);
