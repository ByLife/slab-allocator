#[derive(Copy, Clone, Debug)]
pub struct ChunkMetadata {
    pub(crate) offset: usize,
    pub(crate) size: usize,
    pub(crate) is_used: bool,
}

impl ChunkMetadata {
    pub const fn new(offset: usize, size: usize) -> Self {
        Self {
            offset,
            size,
            is_used: false,
        }
    }
}