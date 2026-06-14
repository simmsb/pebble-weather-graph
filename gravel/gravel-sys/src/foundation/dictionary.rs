use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn dict_calc_buffer_size(tuple_count: u8, ...) -> u32;
    pub fn dict_size(iter: *mut DictionaryIterator) -> u32;
    pub fn dict_write_begin(
        iter: *mut DictionaryIterator,
        buffer: *mut u8,
        size: u16,
    ) -> DictionaryResult;
    pub fn dict_write_data(
        iter: *mut DictionaryIterator,
        key: u32,
        data: *const u8,
        size: u16,
    ) -> DictionaryResult;
    pub fn dict_write_cstring(
        iter: *mut DictionaryIterator,
        key: u32,
        cstring: *const c_char,
    ) -> DictionaryResult;
    pub fn dict_write_int(iter: *mut DictionaryIterator, key: u32, value: u8) -> DictionaryResult;
    pub fn dict_write_uint8(iter: *mut DictionaryIterator, key: u32, value: u8)
    -> DictionaryResult;
    pub fn dict_write_uint16(
        iter: *mut DictionaryIterator,
        key: u32,
        value: u16,
    ) -> DictionaryResult;
    pub fn dict_write_uint32(
        iter: *mut DictionaryIterator,
        key: u32,
        value: u32,
    ) -> DictionaryResult;
    pub fn dict_write_int8(iter: *mut DictionaryIterator, key: u32, value: i8) -> DictionaryResult;
    pub fn dict_write_int16(
        iter: *mut DictionaryIterator,
        key: u32,
        value: i16,
    ) -> DictionaryResult;
    pub fn dict_write_int32(
        iter: *mut DictionaryIterator,
        key: u32,
        value: i32,
    ) -> DictionaryResult;
    pub fn dict_write_end(iter: *mut DictionaryIterator) -> u32;
    pub fn dict_read_begin_from_buffer(
        iter: *mut DictionaryIterator,
        buffer: *const u8,
        size: u16,
    ) -> *mut Tuple;
    pub fn dict_read_next(iter: *mut DictionaryIterator) -> *mut Tuple;
    pub fn dict_read_first(iter: *mut DictionaryIterator) -> *mut Tuple;
    pub fn dict_serialize_tuplets(
        callback: DictionarySerializeCallback,
        context: *mut c_void,
        tuplets: *const Tuplet,
        tuplets_count: u8,
    ) -> DictionaryResult;
    pub fn dict_serialize_tuplets_to_buffer(
        tuplets: *const Tuplet,
        tuplets_count: u8,
        buffer: *const u8,
        size_in_out: *mut u32,
    ) -> DictionaryResult;
    pub fn dict_serialize_tuplets_to_buffer_with_iter(
        iter: *mut DictionaryIterator,
        tuplets: *const Tuplet,
        tuplets_count: u8,
        buffer: *const u8,
        size_in_out: *mut u32,
    ) -> DictionaryResult;
    pub fn dict_write_tuplet(
        iter: *mut DictionaryIterator,
        tuplet: *const Tuplet,
    ) -> DictionaryResult;
    pub fn dict_calc_buffer_size_from_tuplets(tuplets: *const Tuplet, tuplets_count: u8) -> u32;
    pub fn dict_merge(
        dest: *mut DictionaryIterator,
        dest_max_size_in_out: *mut u32,
        source: *mut DictionaryIterator,
        update_existing_keys_only: bool,
        key_callback: DictionaryKeyUpdatedCallback,
        context: *mut c_void,
    );
    pub fn dict_find(iter: *const DictionaryIterator, key: u32) -> *mut Tuple;
}

#[derive(Default, Clone)]
#[repr(C)]
pub struct DictionaryIterator {
    pub dictionary: *mut Dictionary,
    pub end: *const core::ffi::c_void,
    pub cursor: *mut Tuple,
}

#[repr(C, packed)]
pub struct Tuple {
    pub key: u32,
    pub value_type: TupleType,
    pub length: u16,
    pub value: TupleValue,
}

#[repr(C)]
pub union TupleValue {
    pub data: [u8; 1],
    pub cstring: [c_char; 1],
    pub uint8: u8,
    pub uint16: u16,
    pub uint32: u32,
    pub int8: i8,
    pub int16: i16,
    pub int32: i32,
}

#[repr(C)]
pub struct Tuplet {
    pub value_type: TupleType,
    pub key: u32,
    pub value: TupletValue,
}

#[repr(C)]
pub union TupletValue {
    pub byte_array: TupletByteArray,
    pub cstring: TupletCString,
    pub integer: TupletInteger,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TupletByteArray {
    pub data: *const u8,
    pub length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TupletCString {
    pub data: *const c_char,
    pub length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TupletInteger {
    pub storage: u32,
    pub length: u16,
}

c_enum! {
    pub enum DictionaryResult;
    OK = 0;
    NOT_ENOUGH_STORAGE = 1;
    INVALID_ARGS = 2;
    INTERNAL_INCONSISTENCY = 3;
    MALLOC_FAILED = 4;
}

c_enum! {
    pub enum TupleType : u8;
    BYTE_ARRAY = 0;
    CSTRING = 1;
    UINT = 2;
    INT = 3;
}

#[repr(C)]
pub struct Dictionary(crate::Opaque);

callbacks! {
    pub fn DictionarySerializeCallback(data: *const u8, size: u16, context: *mut c_void);
    pub fn DictionaryKeyUpdatedCallback(key: u32, new_tuple: *const Tuple, old_tuple: *const Tuple, context: *mut c_void);
}
