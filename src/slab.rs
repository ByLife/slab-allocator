pub struct SlabAlloc {
    memory: [u8; 1024],
    chunks: [ChunkState; 32],
    chunk_size: usize,
    used_chunks: usize,
}

#[derive(Copy, Clone)]
struct ChunkState {
    offset: usize,
    is_used: bool,
}

#[derive(Debug)]
pub enum AllocError {
    NoSpace,
    InvalidFree,
}

impl SlabAlloc {
    pub const fn new() -> Self {
        const INIT_CHUNK: ChunkState = ChunkState {
            offset: 0,
            is_used: false,
        };
        
        SlabAlloc {
            memory: [0; 1024],
            chunks: [INIT_CHUNK; 32],
            chunk_size: 32,
            used_chunks: 0,
        }
    }

    pub fn alloc(&mut self) -> Result<*mut u8, AllocError> {
        for (i, chunk) in self.chunks.iter_mut().enumerate() {
            if !chunk.is_used {
                chunk.is_used = true;
                chunk.offset = i * self.chunk_size;
                self.used_chunks += 1;
                return Ok(unsafe { 
                    self.memory.as_mut_ptr().add(chunk.offset)
                });
            }
        }
        Err(AllocError::NoSpace)
    }

    pub fn free(&mut self, ptr: *mut u8) -> Result<(), AllocError> {
        let offset = unsafe {
            ptr.offset_from(self.memory.as_ptr()) as usize
        };
        
        let chunk_idx = offset / self.chunk_size;
        
        if chunk_idx >= self.chunks.len() || 
           !self.chunks[chunk_idx].is_used || 
           self.chunks[chunk_idx].offset != offset {
            return Err(AllocError::InvalidFree);
        }

        self.chunks[chunk_idx].is_used = false;
        self.used_chunks -= 1;
        Ok(())
    }

    pub fn get_stats(&self) -> (usize, usize) {
        (self.used_chunks, self.chunks.len())
    }
}