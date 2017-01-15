// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! this ALWAYS GENERATED file contains the definitions for the interfaces
use shared::minwindef::DWORD;
use shared::wtypes::PROPERTYKEY;
use um::propidl::{PROPVARIANT, REFPROPVARIANT};
use um::propkeydef::REFPROPERTYKEY;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;

pub type IPropertyDescriptionList = IUnknown; // TODO

RIDL!(
interface IPropertyStore(IPropertyStoreVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount( 
        &self,
        cProps: *mut DWORD
    ) -> HRESULT,
    fn GetAt( 
        &self,
        iProp: DWORD,
        pkey: *mut PROPERTYKEY
    ) -> HRESULT,
    fn GetValue( 
        &self,
        key: REFPROPERTYKEY,
        pv: *mut PROPVARIANT
    ) -> HRESULT,
    fn SetValue( 
        &self,
        key: REFPROPERTYKEY,
        propvar: REFPROPVARIANT
    ) -> HRESULT,
    fn Commit(
        &self
    ) -> HRESULT
}
);
