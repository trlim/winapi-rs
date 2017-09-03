use shared::basetsd::{UINT32};
use um::mfobjects::IMFAttributes;
use um::winnt::{HRESULT};
extern "system" {
    pub fn MFCreateAttributes(
        ppMFAttributes: *mut *mut IMFAttributes,
        cInitialSize: UINT32,
    ) -> HRESULT;
}
