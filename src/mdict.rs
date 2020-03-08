use std::fs::File;
use std::io::{Read, Cursor, SeekFrom, Seek};
use byteorder::{BigEndian, ReadBytesExt, LittleEndian};
use adler32::adler32;
use std::convert::TryInto;
use failure::Error;

pub struct MdictHeader {

}

pub fn read_header(file_name: &str) -> Result<MdictHeader, Error>{
    let mut file = File::open(file_name)?;
    let size = file.read_uint::<BigEndian>(4)?;
    let mut header: Vec<u8> = vec![0 ; size as usize];
    for i in 0..size {
        header[i as usize] = file.read_u8()?;
    }
    let actual_checksum = adler32(&header[..]).unwrap();
    let checksum = file.read_uint::<LittleEndian>(4)?;
    println!("Actual check sum {}", actual_checksum);
    println!("Checksum {}", checksum);
    Ok(MdictHeader{})
}