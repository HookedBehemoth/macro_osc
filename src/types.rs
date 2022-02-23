use crate::align::align_up;
use byteorder::{BigEndian, ByteOrder};

pub trait OscType {
    #[inline]
    fn width(&self) -> usize
    where
        Self: Sized,
    {
        std::mem::size_of::<Self>()
    }
    #[inline]
    fn tag_width(&self) -> usize {
        std::mem::size_of::<u8>()
    }
    fn tag(&self) -> u8;
    fn encode(&self, buf: &mut [u8]);
}

impl OscType for i32 {
    #[inline]
    fn tag(&self) -> u8 {
        b'i'
    }
    #[inline]
    fn encode(&self, buf: &mut [u8]) {
        BigEndian::write_i32(buf, *self);
    }
}

impl OscType for f32 {
    #[inline]
    fn tag(&self) -> u8 {
        b'f'
    }
    #[inline]
    fn encode(&self, buf: &mut [u8]) {
        BigEndian::write_f32(buf, *self);
    }
}

impl OscType for bool {
    #[inline]
    fn width(&self) -> usize {
        0
    }
    #[inline]
    fn tag(&self) -> u8 {
        if *self {
            b'T'
        } else {
            b'F'
        }
    }
    #[inline]
    fn encode(&self, _buf: &mut [u8]) {
        // do nothing
    }
}

impl OscType for &[u8] {
    #[inline]
    fn width(&self) -> usize {
        align_up(std::mem::size_of::<i32>() + self.len(), 4)
    }
    #[inline]
    fn tag(&self) -> u8 {
        b'b'
    }
    #[inline]
    fn encode(&self, buf: &mut [u8]) {
        BigEndian::write_i32(&mut buf[..4], self.len() as i32);
        buf[4..4 + self.len()].copy_from_slice(self);
    }
}

impl OscType for &str {
    #[inline]
    fn width(&self) -> usize {
        align_up(self.len() + 1, 4)
    }
    #[inline]
    fn tag(&self) -> u8 {
        b's'
    }
    #[inline]
    fn encode(&self, buf: &mut [u8]) {
        buf[..self.len()].copy_from_slice(self.as_bytes());
        buf[self.len()] = 0;
    }
}
