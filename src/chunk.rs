#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ChunkMetadata {
    offset: usize,
    size: usize,
    is_used: bool,
}

impl ChunkMetadata {
    #[inline]
    pub const fn new(offset: usize, size: usize) -> Self {
        Self {
            offset,
            size,
            is_used: false,
        }
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_used(&self) -> bool {
        self.is_used
    }

    #[inline]
    pub fn mark_used(&mut self) {
        self.is_used = true;
    }

    #[inline]
    pub fn mark_free(&mut self) {
        self.is_used = false;
    }
}