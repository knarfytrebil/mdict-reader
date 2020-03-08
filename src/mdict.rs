use std::fs::File;
use std::io::{Read, Cursor, SeekFrom, Seek};
use byteorder::{BigEndian, ReadBytesExt, LittleEndian};
use adler32::adler32;
use std::convert::TryInto;
use failure::Error;

pub struct MdictHeader {
    offset: u64
}

pub fn read_header(file_name: &str) -> Result<MdictHeader, Error>{
    let offset: u64;
    {
        let mut file = File::open(file_name)?;
        let size = file.read_uint::<BigEndian>(4)?;
        let mut header: Vec<u8> = vec![0 ; size as usize];
        for i in 0..size {
            header[i as usize] = file.read_u8()?;
        }
        let actual_checksum = adler32(&header[..])?;
        let expected_checksum = file.read_uint::<LittleEndian>(4)? as u32;
        assert_eq!(actual_checksum, expected_checksum);
        offset = file.seek(SeekFrom::Current(0))?;
    }

    Ok(MdictHeader{
        offset
    })
}