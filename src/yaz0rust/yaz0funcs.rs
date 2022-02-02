use std::str;
use std::io::*;
use yaz0::deflate::*;

pub fn yaz0decomp(data: &Vec<u8>) -> Vec<u8> {
    let magic = &data[0..4];
    let mut output = vec![0];
    let test = str::from_utf8(magic).unwrap();
    if test != "Yaz0" {
        return output;
    }
    let fullsize = ((data[4] as i32) << 24) | ((data[5] as i32) << 16) | ((data[6] as i32) << 8) | data[7] as i32;
    output = vec![0; fullsize as usize];
    let mut inpos = 16;
    let mut outpos = 0;
    while outpos < fullsize {
        let mut block = data[inpos];
        inpos += 1;
        for _i in 0..8 {
            if ((block as i32) & 0x80) != 0 {
                output[outpos as usize] = data[inpos as usize];
                outpos += 1;
                inpos += 1;
            } else {
                let b1 = data[inpos as usize];
                inpos += 1;
                let b2 = data[inpos as usize];
                inpos += 1;
                let dist = (((b1 as i32) & 0xF) << 8) | (b2 as i32);
                let mut copysrc = outpos - (dist + 1);
                let mut nbytes = (b1 as i32) >> 4;
                if nbytes == 0 {
                    nbytes = (data[inpos as usize] as i32) + 0x12;
                    inpos += 1;
                } else {
                    nbytes += 2;
                }
                for _j in 0..nbytes {
                    output[outpos as usize] = output[copysrc as usize];
                    outpos += 1;
                    copysrc += 1;
                }
            }
            block <<= 1;
            if outpos >= fullsize || inpos >= data.len() {
                break;
            }
        }
    }
    output
}

pub fn yaz0comp(data: &[u8]) -> Vec<u8> {
    let mut mem = Cursor::new(Vec::new());
    let writer = Yaz0Writer::new(&mut mem);
    let quality = CompressionLevel::Lookahead {quality: 10};
    writer.compress_and_write(data, quality).unwrap();
    mem.seek(SeekFrom::Start(0)).unwrap();
    let mut res = Vec::new();
    mem.read_to_end(&mut res).unwrap();
    return res;
}