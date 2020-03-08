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
    let mut raw_header: Vec<u8>;
    {
        let mut file = File::open(file_name)?;
        let size = file.read_uint::<BigEndian>(4)?;
        raw_header = vec![0 ; size as usize];
        for i in 0..size {
            raw_header[i as usize] = file.read_u8()?;
        }
        let actual_checksum = adler32(&raw_header[..])?;
        let expected_checksum = file.read_uint::<LittleEndian>(4)? as u32;
        assert_eq!(actual_checksum, expected_checksum);
        offset = file.seek(SeekFrom::Current(0))?;
    }

    // The header was read in utf-8 format, while actually it is in utf-16.
    // Convert it into utf-16 and generate header string.
    let header_in_utf16 = raw_header
        .chunks_exact(2 as usize)
        .into_iter()
        .map(|c| u16::from_ne_bytes([c[0], c[1]]))
        .collect::<Vec<u16>>();

    let header = String::from_utf16_lossy(
            // The last two items are '\x00\x00'
            &header_in_utf16[..header_in_utf16.len() - 2]);

    Ok(MdictHeader{
        offset
    })
}