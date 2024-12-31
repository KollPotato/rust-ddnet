use crate::format::OnlyI32;
use crate::raw::CallbackError;
use crate::raw::CallbackNew;
use crate::raw::CallbackReadData;
use crate::raw::CallbackReadError;
use std::mem;

pub use libtw2_common::relative_size_of;
pub use libtw2_common::relative_size_of_mult;
pub use libtw2_common::slice::transmute as transmute_slice;
pub use libtw2_common::slice::transmute_mut as transmute_mut_slice;

pub fn as_mut_i32_slice<T: OnlyI32>(x: &mut [T]) -> &mut [i32] {
    unsafe { transmute_mut_slice(x) }
}

pub unsafe fn from_little_endian<T>(buffer: &mut [T]) {
    if cfg!(target_endian = "big") {
        swap_endian(buffer);
    }
}

pub unsafe fn to_little_endian<T>(buffer: &mut [T]) {
    if cfg!(target_endian = "big") {
        swap_endian(buffer);
    }
}

// Depending on the target's endianness this function might not be needed.
#[allow(dead_code)]
pub unsafe fn swap_endian<T>(buffer: &mut [T]) {
    let len = buffer.len();
    let buffer_bytes: &mut [u8] = transmute_mut_slice(buffer);
    for i in 0..len {
        let mut start = i * mem::size_of::<T>();
        let mut end = start + mem::size_of::<T>() - 1;
        while start < end {
            buffer_bytes.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}

pub trait CallbackNewExt {
    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), CallbackReadError>;
    unsafe fn read_exact_raw<T>(&mut self, buffer: &mut [T]) -> Result<(), CallbackReadError>;
    fn read_le_i32s<T: OnlyI32>(&mut self, buffer: &mut [T]) -> Result<usize, CallbackError>;
    fn read_exact_le_i32s<T: OnlyI32>(&mut self, buffer: &mut [T])
        -> Result<(), CallbackReadError>;
    fn read_exact_le_i32s_owned<T: OnlyI32>(
        &mut self,
        count: usize,
    ) -> Result<Vec<T>, CallbackReadError>;
}

impl<'a> CallbackNewExt for &'a mut dyn CallbackNew {
    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), CallbackReadError> {
        let read = self.read(buffer)?;
        if read != buffer.len() {
            return Err(CallbackReadError::EndOfFile);
        }
        Ok(())
    }
    unsafe fn read_exact_raw<T>(&mut self, buffer: &mut [T]) -> Result<(), CallbackReadError> {
        self.read_exact(transmute_mut_slice(buffer))
    }
    fn read_le_i32s<T: OnlyI32>(&mut self, buffer: &mut [T]) -> Result<usize, CallbackError> {
        let read = self.read(unsafe { transmute_mut_slice(buffer) })?;
        let read_i32s = read / mem::size_of::<i32>();
        unsafe { from_little_endian(&mut as_mut_i32_slice(buffer)[..read_i32s]) };
        Ok(read)
    }
    fn read_exact_le_i32s<T: OnlyI32>(
        &mut self,
        buffer: &mut [T],
    ) -> Result<(), CallbackReadError> {
        unsafe { self.read_exact_raw(buffer) }?;
        unsafe {
            from_little_endian(as_mut_i32_slice(buffer));
        }
        Ok(())
    }
    fn read_exact_le_i32s_owned<T: OnlyI32>(
        &mut self,
        count: usize,
    ) -> Result<Vec<T>, CallbackReadError> {
        let mut result = Vec::with_capacity(count);
        // Safe because T: OnlyI32 is POD.
        unsafe {
            result.set_len(count);
        }
        self.read_exact_le_i32s(&mut result)?;
        Ok(result)
    }
}

pub trait CallbackReadDataExt {
    fn seek_read_exact(&mut self, offset: u32, buffer: &mut [u8]) -> Result<(), CallbackReadError>;
    fn seek_read_exact_owned(
        &mut self,
        offset: u32,
        count: usize,
    ) -> Result<Vec<u8>, CallbackReadError>;
}

impl<'a> CallbackReadDataExt for &'a mut dyn CallbackReadData {
    fn seek_read_exact(&mut self, offset: u32, buffer: &mut [u8]) -> Result<(), CallbackReadError> {
        let read = self.seek_read(offset, buffer)?;
        if read != buffer.len() {
            return Err(CallbackReadError::EndOfFile);
        }
        Ok(())
    }
    fn seek_read_exact_owned(
        &mut self,
        offset: u32,
        count: usize,
    ) -> Result<Vec<u8>, CallbackReadError> {
        let mut result = Vec::with_capacity(count);
        // Safe because `u8` is POD.
        unsafe {
            result.set_len(count);
        }
        self.seek_read_exact(offset, &mut result)?;
        Ok(result)
    }
}
