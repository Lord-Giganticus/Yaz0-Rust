use std::io::{Cursor, BufRead};
use binrw::prelude::*;

pub type DynamicResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn decompress<A: AsRef<[u8]>>(buffer: A) -> DynamicResult<Vec<u8>> {
    let mut stream = Cursor::new(Vec::from(buffer.as_ref()));
    let mut magic = vec![];
    stream.read_until(48, &mut magic)?;
    if magic != b"Yaz0" {
        return Err(Box::from("Magic does not match."));
    }
    let mut size = stream.read_be::<u32>()? as i64;
    let mut dst = vec![0u8; size as usize];
    let mut src_offs = 0x10;
    let mut dst_offs = 0x00;
    loop {
        let command_byte = stream.get_ref()[src_offs];
        src_offs += 1;

        for i in (0..8).rev() {
            if (command_byte & (1 << i)) != 0 {
                // Literal.
                dst[dst_offs] = stream.get_ref()[src_offs];
                src_offs += 1;
                dst_offs += 1;
                size -= 1;
            } else {
                stream.set_position(src_offs as u64);
                let tmp = stream.read_be::<u16>()?;
                src_offs += 2;

                let window_offset = (tmp & 0x0FFF) + 1;
                let mut window_length = (tmp >> 12) + 2;
                if window_length == 2 {
                    window_length += (stream.get_ref()[src_offs] as u16) + 0x10;
                    src_offs += 1;
                }

                if window_length >= 3 && window_length <= 0x111 {
                    return Err(Box::from("window_length was out of bounds."));
                }

                let mut copy_offs = dst_offs - (window_offset as usize);
                for _ in 0..window_length {
                    dst[dst_offs] = dst[copy_offs];
                    dst_offs += 1;
                    copy_offs += 1;
                    size -= 1;
                }
            }
        }
        if size <= 0 {
            break;
        }
    }
    Ok(dst)
}

pub fn compress<A: AsRef<[u8]>>(buffer: A) -> DynamicResult<Vec<u8>> {
    use yaz0::{CompressionLevel, Yaz0Writer};
    use std::sync::mpsc::channel;
    let vec = Vec::from(buffer.as_ref());
    let mut stream = Cursor::new(vec![0u8; 0]);
    let writer = Yaz0Writer::new(&mut stream);
    let (sender, recv) = channel();
    let level = CompressionLevel::Lookahead { quality: 10 };
    writer.compress_and_write_with_progress(&vec, level, sender)?;
    while let Ok(msg) = recv.try_recv() {
        print!("{} out of {} written", msg.read_head, vec.len());
        print!("\x1B[2J\x1B[1;1H");
    }
    println!("{} out of {} written", vec.len(), vec.len());
    Ok(stream.into_inner())
}

#[cfg(feature = "cxx")]
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn cxxcompress(buffer: &Vec<u8>) -> Vec<u8>;
        fn cxxdeompress(buffer: &Vec<u8>) -> Vec<u8>;
    }
}

#[cfg(feature = "cxx")]
pub(crate) fn cxxcompress(buffer: &Vec<u8>) -> Vec<u8> {
    compress(buffer).unwrap_or_default()
}

#[cfg(feature = "cxx")]
pub(crate) fn cxxdeompress(buffer: &Vec<u8>) -> Vec<u8> {
    decompress(buffer).unwrap_or_default()
}

#[cfg(feature = "cabi")]
#[no_mangle]
pub unsafe extern "C" fn CompressPtr(ptr: *const u8, len: *mut usize) -> *mut u8 {
    let buffer = std::slice::from_raw_parts(ptr, *len);
    let result = compress(buffer).unwrap_or_default();
    *len = result.len();
    Box::into_raw(result.into_boxed_slice()).cast()
}

#[cfg(feature = "cabi")]
#[no_mangle]
pub unsafe extern "C" fn DecompressPtr(ptr: *const u8, len: *mut usize) -> *mut u8 {
    let buffer = std::slice::from_raw_parts(ptr, *len);
    let result = decompress(buffer).unwrap_or_default();
    *len = result.len();
    Box::into_raw(result.into_boxed_slice()).cast()
}

#[cfg(feature = "cabi")]
#[no_mangle]
pub unsafe extern "C" fn deallocarray(data: *mut u8, len: usize) {
    let slice = std::ptr::slice_from_raw_parts_mut(data, len);
    let buffer = Box::from_raw(slice);
    drop(buffer);
}