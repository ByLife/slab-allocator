#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ChunkMetadata {
    offset: usize,
    size: usize,
    next: Option<usize>,
    is_used: bool,
}

impl ChunkMetadata {
    #[inline]
    pub const fn new(offset: usize, size: usize, next: Option<usize>) -> Self {
        Self {
            offset,
            size,
            next,
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
    pub fn next(&self) -> Option<usize> {
        self.next
    }

    #[inline]
    pub fn is_used(&self) -> bool {
        self.is_used
    }

    #[inline]
    pub fn mark_used(&mut self) {
        self.is_used = true;
        self.next = None;
    }

    #[inline]
    pub fn mark_free(&mut self, next: Option<usize>) {
        self.is_used = false;
        self.next = next;
    }
}