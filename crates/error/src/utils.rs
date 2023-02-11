use std::ops::Range;

#[allow(clippy::range_plus_one)]
pub const fn ranged(position: usize) -> Range<usize> {
    position..position + 1
}
