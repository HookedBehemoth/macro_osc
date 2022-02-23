#[inline]
pub fn align_up(len: usize, align: usize) -> usize {
    len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1)
}
