use crate::align::align_up;
use byteorder::{BigEndian, ByteOrder};
use std::io::Write;

pub trait OscType {
    fn width(&self) -> usize
    where
        Self: Sized,
    {
        std::mem::size_of::<Self>()
    }
    fn tag_width(&self) -> usize {
        std::mem::size_of::<u8>()
    }
    fn tag(&self) -> u8;
    fn encode(&self, buf: &mut [u8]);
}

impl OscType for i32 {
    fn tag(&self) -> u8 {
        b'i'
    }
    fn encode(&self, buf: &mut [u8]) {
        BigEndian::write_i32(buf, *self);
    }
}

impl OscType for f32 {
    fn tag(&self) -> u8 {
        b'f'
    }
    fn encode(&self, buf: &mut [u8]) {
        BigEndian::write_f32(buf, *self);
    }
}

impl OscType for bool {
    fn width(&self) -> usize {
        0
    }
    fn tag(&self) -> u8 {
        if *self {
            b'T'
        } else {
            b'F'
        }
    }
    fn encode(&self, _buf: &mut [u8]) {
        // do nothing
    }
}

impl OscType for &[u8] {
    fn width(&self) -> usize {
        align_up(std::mem::size_of::<i32>() + self.len(), 4)
    }
    fn tag(&self) -> u8 {
        b'b'
    }
    fn encode(&self, buf: &mut [u8]) {
        BigEndian::write_i32(&mut buf[..4], self.len() as i32);
        buf[4..4 + self.len()].copy_from_slice(self);
    }
}

impl OscType for &str {
    fn width(&self) -> usize {
        align_up(self.len() + 1, 4)
    }
    fn tag(&self) -> u8 {
        b's'
    }
    fn encode(&self, mut buf: &mut [u8]) {
        buf.write_all(self.as_bytes()).unwrap();
        buf[self.len()] = 0;
    }
}
