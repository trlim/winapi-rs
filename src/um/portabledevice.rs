// Copyright © 2015-2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of PortableDevice.h
DEFINE_GUID!{GUID_DEVINTERFACE_WPD,
    0x6AC27878, 0xA6FA, 0x4155, 0xBA, 0x85, 0xF9, 0x8F, 0x49, 0x1D, 0x4F, 0x33}
DEFINE_GUID!{GUID_DEVINTERFACE_WPD_PRIVATE,
    0xBA0C718F, 0x4DED, 0x49B7, 0xBD, 0xD3, 0xFA, 0xBE, 0x28, 0x66, 0x12, 0x11}
DEFINE_GUID!{GUID_DEVINTERFACE_WPD_SERVICE,
    0x9EF44F80, 0x3D64, 0x4246, 0xA6, 0xAA, 0x20, 0x6F, 0x32, 0x8D, 0x1E, 0xDC}
//51
pub const WPD_DEVICE_OBJECT_ID: &[u16] =
    &['D' as u16, 'E' as u16, 'V' as u16, 'I' as u16, 'C' as u16, 'E' as u16, '\0' as u16];
//101
ENUM!{enum WPD_DEVICE_TYPES {
    WPD_DEVICE_TYPE_GENERIC = 0,
    WPD_DEVICE_TYPE_CAMERA = 1,
    WPD_DEVICE_TYPE_MEDIA_PLAYER = 2,
    WPD_DEVICE_TYPE_PHONE = 3,
    WPD_DEVICE_TYPE_VIDEO = 4,
    WPD_DEVICE_TYPE_PERSONAL_INFORMATION_MANAGER = 5,
    WPD_DEVICE_TYPE_AUDIO_RECORDER = 6,
}}
//133
ENUM!{enum WPD_DEVICE_TRANSPORTS {
    WPD_DEVICE_TRANSPORT_UNSPECIFIED = 0,
    WPD_DEVICE_TRANSPORT_USB = 1,
    WPD_DEVICE_TRANSPORT_IP = 2,
    WPD_DEVICE_TRANSPORT_BLUETOOTH = 3,
}}
//175
ENUM!{enum WPD_POWER_SOURCES {
    WPD_POWER_SOURCE_BATTERY = 0,
    WPD_POWER_SOURCE_EXTERNAL = 1,
}}
//696
DEFINE_GUID!{WPD_FUNCTIONAL_OBJECT_PROPERTIES_V1,
    0x8F052D93, 0xABCA, 0x4FC5, 0xA5, 0xAC, 0xB0, 0x1D, 0xF4, 0xDB, 0xE5, 0x98}
DEFINE_PROPERTYKEY!{WPD_FUNCTIONAL_OBJECT_CATEGORY,
    0x8F052D93, 0xABCA, 0x4FC5, 0xA5, 0xAC, 0xB0, 0x1D, 0xF4, 0xDB, 0xE5, 0x98, 2}
//1140
DEFINE_GUID!{WPD_DEVICE_PROPERTIES_V1,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC}
DEFINE_PROPERTYKEY!{WPD_DEVICE_SYNC_PARTNER,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 2}
DEFINE_PROPERTYKEY!{WPD_DEVICE_FIRMWARE_VERSION,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 3}
DEFINE_PROPERTYKEY!{WPD_DEVICE_POWER_LEVEL,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 4}
DEFINE_PROPERTYKEY!{WPD_DEVICE_POWER_SOURCE,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 5}
DEFINE_PROPERTYKEY!{WPD_DEVICE_PROTOCOL,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 6}
DEFINE_PROPERTYKEY!{WPD_DEVICE_MANUFACTURER,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 7}
DEFINE_PROPERTYKEY!{WPD_DEVICE_MODEL,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 8}
DEFINE_PROPERTYKEY!{WPD_DEVICE_SERIAL_NUMBER,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 9}
DEFINE_PROPERTYKEY!{WPD_DEVICE_SUPPORTS_NON_CONSUMABLE,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 10}
DEFINE_PROPERTYKEY!{WPD_DEVICE_DATETIME,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 11}
DEFINE_PROPERTYKEY!{WPD_DEVICE_FRIENDLY_NAME,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 12}
DEFINE_PROPERTYKEY!{WPD_DEVICE_SUPPORTED_DRM_SCHEMES,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 13}
DEFINE_PROPERTYKEY!{WPD_DEVICE_SUPPORTED_FORMATS_ARE_ORDERED,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 14}
DEFINE_PROPERTYKEY!{WPD_DEVICE_TYPE,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 15}
DEFINE_PROPERTYKEY!{WPD_DEVICE_NETWORK_IDENTIFIER,
    0x26D4979A, 0xE643, 0x4626, 0x9E, 0x2B, 0x73, 0x6D, 0xC0, 0xC9, 0x2F, 0xDC, 16}
DEFINE_GUID!{WPD_DEVICE_PROPERTIES_V2,
    0x463DD662, 0x7FC4, 0x4291, 0x91, 0x1C, 0x7F, 0x4C, 0x9C, 0xCA, 0x97, 0x99}
DEFINE_PROPERTYKEY!{WPD_DEVICE_FUNCTIONAL_UNIQUE_ID,
    0x463DD662, 0x7FC4, 0x4291, 0x91, 0x1C, 0x7F, 0x4C, 0x9C, 0xCA, 0x97, 0x99, 2}
DEFINE_PROPERTYKEY!{WPD_DEVICE_MODEL_UNIQUE_ID,
    0x463DD662, 0x7FC4, 0x4291, 0x91, 0x1C, 0x7F, 0x4C, 0x9C, 0xCA, 0x97, 0x99, 3}
DEFINE_PROPERTYKEY!{WPD_DEVICE_TRANSPORT,
    0x463DD662, 0x7FC4, 0x4291, 0x91, 0x1C, 0x7F, 0x4C, 0x9C, 0xCA, 0x97, 0x99, 4}
DEFINE_PROPERTYKEY!{WPD_DEVICE_USE_DEVICE_STAGE,
    0x463DD662, 0x7FC4, 0x4291, 0x91, 0x1C, 0x7F, 0x4C, 0x9C, 0xCA, 0x97, 0x99, 5}
DEFINE_GUID!{WPD_DEVICE_PROPERTIES_V3,
    0x6C2B878C, 0xC2EC, 0x490D, 0xB4, 0x25, 0xD7, 0xA7, 0x5E, 0x23, 0xE5, 0xED}
DEFINE_PROPERTYKEY!{WPD_DEVICE_EDP_IDENTITY,
    0x6C2B878C, 0xC2EC, 0x490D, 0xB4, 0x25, 0xD7, 0xA7, 0x5E, 0x23, 0xE5, 0xED, 1}
//1488
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_COMMAND_CATEGORY,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1001}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_COMMAND_ID,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1002}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_HRESULT,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1003}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_DRIVER_ERROR_CODE,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1004}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_COMMAND_TARGET,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1006}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_PERSISTENT_UNIQUE_IDS,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1007}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_OBJECT_IDS,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1008}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_CLIENT_INFORMATION,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1009}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_CLIENT_INFORMATION_CONTEXT,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1010}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_COMMON_ACTIVITY_ID,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 1011}
DEFINE_PROPERTYKEY!{WPD_OPTION_VALID_OBJECT_IDS,
    0xF0422A9C, 0x5DC8, 0x4440, 0xB5, 0xBD, 0x5D, 0xF2, 0x88, 0x35, 0x65, 0x8A, 5001}
//3440
DEFINE_PROPERTYKEY!{WPD_OBJECT_ID,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 2}
DEFINE_PROPERTYKEY!{WPD_OBJECT_PARENT_ID,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 3}
DEFINE_PROPERTYKEY!{WPD_OBJECT_NAME,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 4}
DEFINE_PROPERTYKEY!{WPD_OBJECT_PERSISTENT_UNIQUE_ID,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 5}
DEFINE_PROPERTYKEY!{WPD_OBJECT_FORMAT,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 6}
DEFINE_PROPERTYKEY!{WPD_OBJECT_ISHIDDEN,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 9}
DEFINE_PROPERTYKEY!{WPD_OBJECT_ISSYSTEM,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 10}
DEFINE_PROPERTYKEY!{WPD_OBJECT_SIZE,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 11}
DEFINE_PROPERTYKEY!{WPD_OBJECT_ORIGINAL_FILE_NAME,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 12}
DEFINE_PROPERTYKEY!{WPD_OBJECT_NON_CONSUMABLE,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 13}
DEFINE_PROPERTYKEY!{WPD_OBJECT_KEYWORDS,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 15}
DEFINE_PROPERTYKEY!{WPD_OBJECT_SYNC_ID,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 16}
DEFINE_PROPERTYKEY!{WPD_OBJECT_IS_DRM_PROTECTED,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 17}
DEFINE_PROPERTYKEY!{WPD_OBJECT_DATE_CREATED,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 18}
DEFINE_PROPERTYKEY!{WPD_OBJECT_DATE_MODIFIED,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 19}
DEFINE_PROPERTYKEY!{WPD_OBJECT_DATE_AUTHORED,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 20}
DEFINE_PROPERTYKEY!{WPD_OBJECT_BACK_REFERENCES,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 21}
DEFINE_PROPERTYKEY!{WPD_OBJECT_CAN_DELETE,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 26}
DEFINE_PROPERTYKEY!{WPD_OBJECT_LANGUAGE_LOCALE,
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 27}
