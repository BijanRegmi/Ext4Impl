use libc::memcpy;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::os::raw::c_void;

pub trait LoadAble: Sized {
    fn from_file_offset(file: &mut File, offset: u64) -> std::io::Result<Self> {
        let mut bytes = vec![0u8; size_of::<Self>()];

        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(&mut bytes[..])?;

        let size = size_of::<Self>();
        let mut result = unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() };

        unsafe {
            memcpy(
                &mut result as *mut Self as *mut c_void,
                bytes.as_ptr() as *const c_void,
                size,
            );
        }
        Ok(result)
    }

    fn from_vec(buf: Vec<u8>) -> Self {
        let size = size_of::<Self>();
        let mut result = unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() };
        unsafe {
            memcpy(
                &mut result as *mut Self as *mut c_void,
                buf.as_ptr() as *const c_void,
                size,
            );
        }
        result
    }
}
