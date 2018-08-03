// Copyright © 2015-2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
DEFINE_GUID!{WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56}
DEFINE_PROPERTYKEY!{WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 11}
DEFINE_PROPERTYKEY!{WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 12}
DEFINE_PROPERTYKEY!{WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 13}
DEFINE_PROPERTYKEY!{WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 14}
DEFINE_PROPERTYKEY!{WPD_COMMAND_MTP_EXT_READ_DATA,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 15}
DEFINE_PROPERTYKEY!{WPD_COMMAND_MTP_EXT_WRITE_DATA,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 16}
DEFINE_PROPERTYKEY!{WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 17}
DEFINE_PROPERTYKEY!{WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 18}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_OPERATION_CODE,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1001}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1002}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_RESPONSE_CODE,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1003}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1004}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1005}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1006}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1007}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1008}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1009}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1010}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1011}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_TRANSFER_DATA,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1012}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1013}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION,
    0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56, 1014}
DEFINE_GUID!{WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS,
    0x4d545058, 0x4fce, 0x4578, 0x95, 0xc8, 0x86, 0x98, 0xa9, 0xbc, 0xf, 0x49}
DEFINE_GUID!{WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS,
    0x4d545058, 0x8900, 0x40b3, 0x8f, 0x1d, 0xdc, 0x24, 0x6e, 0x1e, 0x83, 0x70}
DEFINE_GUID!{WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS,
    0x00000000, 0x5738, 0x4ff2, 0x84, 0x45, 0xbe, 0x31, 0x26, 0x69, 0x10, 0x59}
DEFINE_PROPERTYKEY!{WPD_PROPERTY_MTP_EXT_EVENT_PARAMS,
    0x4d545058, 0xef88, 0x4e4d, 0x95, 0xc3, 0x4f, 0x32, 0x7f, 0x72, 0x8a, 0x96, 1011}
