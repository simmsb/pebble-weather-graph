// This whole module could perhaps use a pure-Rust rewrite. It's too weird.

use core::{ffi::CStr, marker::PhantomData};

use crate::SysResult;
use gravel_sys::foundation::dictionary as sys;
use sys::DictionaryIterator as SysIter;

type Result<T = (), E = Error> = core::result::Result<T, E>;

pub fn calc_buffer_size(tuple_count: u8, value_lengths: &[u32]) -> u32 {
    1 + (tuple_count as u32 * 7) + value_lengths.iter().sum::<u32>()
}

pub fn write<'a>(
    buffer: &'a mut [u8],
    func: impl FnOnce(&mut DictWriter<'a>) -> Result,
) -> Result<u32> {
    let mut writer = DictWriter::default();
    unsafe { sys::dict_write_begin(&mut writer.iter, buffer.as_mut_ptr(), buffer.len() as u16) }
        .into_nice()?;

    func(&mut writer)?;

    Ok(unsafe { sys::dict_write_end(&mut writer.iter) })
}

pub unsafe fn read<'a>(buffer: &'a [u8]) -> (DictReader<'a>, Option<(u32, DictValue<'a>)>) {
    unsafe {
        let mut reader = DictReader::default();
        let first = sys::dict_read_begin_from_buffer(
            &mut reader.iter,
            buffer.as_ptr(),
            buffer.len() as u16,
        );

        let pair = first.as_ref().map(|v| DictValue::pair_from_tuple(v));
        (reader, pair)
    }
}

#[repr(transparent)]
pub struct DictWriter<'a> {
    iter: SysIter,
    marker: PhantomData<&'a mut [u8]>,
}

impl DictWriter<'_> {
    pub(crate) fn default() -> Self {
        Self {
            iter: Default::default(),
            marker: PhantomData,
        }
    }

    pub fn len(&mut self) -> u32 {
        unsafe { sys::dict_size(&mut self.iter) }
    }

    pub fn byte_array(&mut self, key: u32, value: &[u8]) -> Result<&mut Self> {
        unsafe { sys::dict_write_data(&mut self.iter, key, value.as_ptr(), value.len() as u16) }
            .into_nice()?;
        Ok(self)
    }
    pub fn cstring(&mut self, key: u32, value: &CStr) -> Result<&mut Self> {
        unsafe { sys::dict_write_cstring(&mut self.iter, key, value.as_ptr()) }.into_nice()?;
        Ok(self)
    }
    pub fn u8(&mut self, key: u32, value: u8) -> Result<&mut Self> {
        unsafe { sys::dict_write_uint8(&mut self.iter, key, value) }.into_nice()?;
        Ok(self)
    }
    pub fn u16(&mut self, key: u32, value: u16) -> Result<&mut Self> {
        unsafe { sys::dict_write_uint16(&mut self.iter, key, value) }.into_nice()?;
        Ok(self)
    }
    pub fn u32(&mut self, key: u32, value: u32) -> Result<&mut Self> {
        unsafe { sys::dict_write_uint32(&mut self.iter, key, value) }.into_nice()?;
        Ok(self)
    }
    pub fn i8(&mut self, key: u32, value: i8) -> Result<&mut Self> {
        unsafe { sys::dict_write_int8(&mut self.iter, key, value) }.into_nice()?;
        Ok(self)
    }
    pub fn i16(&mut self, key: u32, value: i16) -> Result<&mut Self> {
        unsafe { sys::dict_write_int16(&mut self.iter, key, value) }.into_nice()?;
        Ok(self)
    }
    pub fn i32(&mut self, key: u32, value: i32) -> Result<&mut Self> {
        unsafe { sys::dict_write_int32(&mut self.iter, key, value) }.into_nice()?;
        Ok(self)
    }
}

pub struct DictIterator<'a> {
    next: Option<(u32, DictValue<'a>)>,
    iter: DictReader<'a>,
}

impl<'a> Iterator for DictIterator<'a> {
    type Item = (u32, DictValue<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.next.take() {
            self.next = unsafe { self.iter.next() };
            return Some(v);
        }

        None
    }
}

impl<'a> DictIterator<'a> {
    fn new(mut iter: DictReader<'a>) -> Self {
        let next = unsafe { iter.first() };

        Self { next, iter }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct DictReader<'a> {
    iter: SysIter,
    marker: PhantomData<&'a [u8]>,
}

impl<'a> IntoIterator for DictReader<'a> {
    type Item = <DictIterator<'a> as Iterator>::Item;

    type IntoIter = DictIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DictIterator::new(self)
    }
}

impl<'a> DictReader<'a> {
    pub(crate) fn default() -> Self {
        Self {
            iter: Default::default(),
            marker: PhantomData,
        }
    }

    pub fn len(&mut self) -> u32 {
        unsafe { sys::dict_size(&mut self.iter) }
    }

    pub unsafe fn first(&mut self) -> Option<(u32, DictValue<'a>)> {
        unsafe {
            sys::dict_read_first(&mut self.iter)
                .as_ref()
                .map(|v| DictValue::pair_from_tuple(v))
        }
    }

    pub unsafe fn next(&mut self) -> Option<(u32, DictValue<'a>)> {
        unsafe {
            sys::dict_read_next(&mut self.iter)
                .as_ref()
                .map(|v| DictValue::pair_from_tuple(v))
        }
    }

    pub unsafe fn find(&mut self, key: u32) -> Option<DictValue<'a>> {
        unsafe {
            sys::dict_find(&mut self.iter, key)
                .as_ref()
                .map(|v| DictValue::from_tuple(v))
        }
    }
}

error_enum! {
    pub enum Error: sys::DictionaryResult;
    NotEnoughStorage = NOT_ENOUGH_STORAGE;
    InvalidArgs = INVALID_ARGS;
    InternalInconsistency = INTERNAL_INCONSISTENCY;
    MallocFailed = MALLOC_FAILED;
}

#[derive(Debug)]
pub enum DictValue<'a> {
    ByteArray(&'a [u8]),
    CString(&'a CStr),
    U8(u8),
    U16(u16),
    U32(u32),
    I8(i8),
    I16(i16),
    I32(i32),
    Invalid,
}

impl<'a> DictValue<'a> {
    pub unsafe fn from_tuple(tuple: &'a sys::Tuple) -> Self {
        unsafe {
            if tuple.value_type == sys::TupleType::BYTE_ARRAY {
                Self::ByteArray(core::slice::from_raw_parts(
                    &raw const tuple.value.data[0],
                    tuple.length as usize,
                ))
            } else if tuple.value_type == sys::TupleType::CSTRING {
                Self::CString(CStr::from_ptr(&raw const tuple.value.cstring[0]))
            } else if tuple.value_type == sys::TupleType::UINT {
                match tuple.length {
                    1 => Self::U8(tuple.value.uint8),
                    2 => Self::U16(tuple.value.uint16),
                    4 => Self::U32(tuple.value.uint32),
                    _ => Self::Invalid,
                }
            } else if tuple.value_type == sys::TupleType::INT {
                match tuple.length {
                    1 => Self::I8(tuple.value.int8),
                    2 => Self::I16(tuple.value.int16),
                    4 => Self::I32(tuple.value.int32),
                    _ => Self::Invalid,
                }
            } else {
                Self::Invalid
            }
        }
    }

    pub unsafe fn pair_from_tuple(tuple: &'a sys::Tuple) -> (u32, Self) {
        (tuple.key, unsafe { Self::from_tuple(tuple) })
    }
}
