#![allow(non_camel_case_types)]
extern crate libc;
use self::libc::{c_char, size_t, uint8_t, uint16_t, uint32_t, uint64_t};

pub type SkBool = ::libc::c_uint;

/* ATA SMART test type (ATA8 7.52.5.2) */
#[repr(C)]
#[derive(Debug)]
pub enum SkSmartSelfTest {
    SK_SMART_SELF_TEST_SHORT = 1,
    SK_SMART_SELF_TEST_EXTENDED = 2,
    SK_SMART_SELF_TEST_CONVEYANCE = 3,
    SK_SMART_SELF_TEST_ABORT = 127
}

#[repr(C)]
pub struct SkIdentifyParsedData {
    pub serial: [c_char; 21],
    pub firmware: [c_char; 9],
    pub model: [c_char; 41],
}

#[repr(C)]
#[derive(Debug)]
pub enum SkSmartOfflineDataCollectionStatus {
    SK_SMART_OFFLINE_DATA_COLLECTION_STATUS_NEVER,
    SK_SMART_OFFLINE_DATA_COLLECTION_STATUS_SUCCESS,
    SK_SMART_OFFLINE_DATA_COLLECTION_STATUS_INPROGRESS,
    SK_SMART_OFFLINE_DATA_COLLECTION_STATUS_SUSPENDED,
    SK_SMART_OFFLINE_DATA_COLLECTION_STATUS_ABORTED,
    SK_SMART_OFFLINE_DATA_COLLECTION_STATUS_FATAL,
    SK_SMART_OFFLINE_DATA_COLLECTION_STATUS_UNKNOWN,
    _SK_SMART_OFFLINE_DATA_COLLECTION_STATUS_MAX
}



#[repr(C)]
#[derive(Debug)]
pub enum SkSmartSelfTestExecutionStatus {
    SK_SMART_SELF_TEST_EXECUTION_STATUS_SUCCESS_OR_NEVER = 0,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_ABORTED = 1,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_INTERRUPTED = 2,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_FATAL = 3,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_ERROR_UNKNOWN = 4,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_ERROR_ELECTRICAL = 5,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_ERROR_SERVO = 6,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_ERROR_READ = 7,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_ERROR_HANDLING = 8,
    SK_SMART_SELF_TEST_EXECUTION_STATUS_INPROGRESS = 15,
    _SK_SMART_SELF_TEST_EXECUTION_STATUS_MAX
}


#[repr(C)]
#[derive(Debug)]
pub struct SkSmartParsedData {
    /* Volatile data */
    pub offline_data_collection_status: SkSmartOfflineDataCollectionStatus ,
    pub total_offline_data_collection_seconds: uint32_t,
    pub self_test_execution_status: SkSmartSelfTestExecutionStatus,
    pub self_test_execution_percent_remaining: uint32_t,

    /* Fixed data */
    pub short_and_extended_test_available: SkBool,
    pub conveyance_test_available: SkBool,
    pub start_test_available: SkBool,
    pub abort_test_available: SkBool,

    pub short_test_polling_minutes: uint32_t,
    pub extended_test_polling_minutes: uint32_t,
    pub conveyance_test_polling_minutes: uint32_t,
}

#[repr(C)]
#[derive(Debug)]
pub enum SkSmartAttributeUnit {
    SK_SMART_ATTRIBUTE_UNIT_UNKNOWN,
    SK_SMART_ATTRIBUTE_UNIT_NONE,
    SK_SMART_ATTRIBUTE_UNIT_MSECONDS,      /* milliseconds */
    SK_SMART_ATTRIBUTE_UNIT_SECTORS,
    SK_SMART_ATTRIBUTE_UNIT_MKELVIN,       /* millikelvin */
    SK_SMART_ATTRIBUTE_UNIT_SMALL_PERCENT, /* percentage with 3 decimal points */
    SK_SMART_ATTRIBUTE_UNIT_PERCENT,       /* integer percentage */
    SK_SMART_ATTRIBUTE_UNIT_MB,
    _SK_SMART_ATTRIBUTE_UNIT_MAX

}


#[repr(C)]
#[derive(Debug)]
pub struct SkSmartAttributeParsedData {
    pub id: uint8_t,
    pub name: *const ::libc::c_char,
    pub pretty_unit: SkSmartAttributeUnit,
    pub flags: uint16_t ,
    pub threshold: uint8_t,
    pub threshold_valid: SkBool,
    pub online: SkBool,
    pub prefailure: SkBool,
    pub good_now: SkBool, 
    pub good_now_valid: SkBool,
    pub good_in_the_past: SkBool,
    pub good_in_the_past_valid: SkBool,
    pub current_value_valid: SkBool,
    pub worst_value_valid: SkBool,
    pub warn: SkBool,
    pub current_value: uint8_t, 
    pub worst_value: uint8_t,
    pub pretty_value: uint64_t,
    pub raw: [uint8_t; 6],
}

pub enum SkDisk{

}

#[repr(C)]
#[derive(Debug)]
pub enum SkSmartOverall  {
    SK_SMART_OVERALL_GOOD,
    SK_SMART_OVERALL_BAD_ATTRIBUTE_IN_THE_PAST,  /* At least one pre-fail attribute exceeded its threshold in the past */
    SK_SMART_OVERALL_BAD_SECTOR,                 /* At least one bad sector */
    SK_SMART_OVERALL_BAD_ATTRIBUTE_NOW,          /* At least one pre-fail attribute is exceeding its threshold now */
    SK_SMART_OVERALL_BAD_SECTOR_MANY,            /* Many bad sectors */
    SK_SMART_OVERALL_BAD_STATUS,                 /* Smart Self Assessment negative */
    SK_SMART_OVERALL_MAX
}

#[link(name = "atasmart")]
extern "C" {
    pub fn sk_smart_self_test_execution_status_to_string(status: SkSmartSelfTestExecutionStatus) -> *const ::libc::c_char;
    pub fn sk_smart_offline_data_collection_status_to_string(status: SkSmartOfflineDataCollectionStatus) -> *const ::libc::c_char;
    pub fn sk_smart_self_test_to_string(test: SkSmartSelfTest) -> *const ::libc::c_char;
    pub fn sk_smart_self_test_available(d: *const SkSmartParsedData, test: SkSmartSelfTest) -> SkBool;
    pub fn sk_smart_self_test_polling_minutes(d: *const SkSmartParsedData, test: SkSmartSelfTest) -> uint32_t;
    pub fn sk_smart_attribute_unit_to_string(unit: SkSmartAttributeUnit) -> *const ::libc::c_char;
    pub fn sk_smart_overall_to_string(overall: SkSmartOverall) -> *const ::libc::c_char;
    pub fn sk_disk_open(name: *const ::libc::c_char, d: *mut *mut SkDisk) -> ::libc::c_int;
    pub fn sk_disk_get_size(d: *mut SkDisk, bytes: *mut uint64_t) -> ::libc::c_int;
    pub fn sk_disk_check_sleep_mode(d: *mut SkDisk, awake: *mut SkBool) -> ::libc::c_int;
    pub fn sk_disk_identify_is_available(d: *mut SkDisk, available: *mut SkBool) -> ::libc::c_int;
    pub fn sk_disk_identify_parse(d: *mut *mut SkDisk, data: *const SkIdentifyParsedData) -> ::libc::c_int;
    pub fn sk_disk_smart_is_available(d: *mut SkDisk, available: *mut SkBool) -> ::libc::c_int;
    pub fn sk_disk_smart_status(d: *mut SkDisk, good: *mut SkBool) -> ::libc::c_int;
    pub fn sk_disk_smart_read_data(d: *mut SkDisk) -> ::libc::c_int;
    pub fn sk_disk_get_blob(d: *mut *mut SkDisk, blob: *const ::libc::c_void, size: *mut size_t) -> ::libc::c_int;
    pub fn sk_disk_set_blob(d: *mut SkDisk, blob: *const ::libc::c_void, size: size_t) -> ::libc::c_int;
    pub fn sk_disk_smart_parse(d: *mut *mut SkDisk, data: *const SkSmartParsedData) -> ::libc::c_int;

    pub fn sk_disk_smart_parse_attributes(d: *mut SkDisk, cb: extern fn(d: *mut SkDisk, data: *const SkSmartAttributeParsedData, userdata: *mut ::libc::c_void), userdata: *mut ::libc::c_void) -> ::libc::c_int;
    
    pub fn sk_disk_smart_self_test(d: *mut SkDisk, test: SkSmartSelfTest) -> ::libc::c_int;
    pub fn sk_disk_smart_get_power_on(d: *mut SkDisk, mseconds: *mut uint64_t) -> ::libc::c_int;
    pub fn sk_disk_smart_get_power_cycle(d: *mut SkDisk, count: *mut uint64_t) -> ::libc::c_int;
    pub fn sk_disk_smart_get_bad(d: *mut SkDisk, sectors: *mut uint64_t) -> ::libc::c_int;
    pub fn sk_disk_smart_get_temperature(d: *mut SkDisk, mkelvin: *mut uint64_t ) -> ::libc::c_int;
    pub fn sk_disk_smart_get_overall(d: *mut SkDisk, overall: *mut SkSmartOverall) -> ::libc::c_int;
    pub fn sk_disk_dump(d: *mut SkDisk) -> ::libc::c_int;
    pub fn sk_disk_free(d: *mut SkDisk) -> ::libc::c_void;
}
