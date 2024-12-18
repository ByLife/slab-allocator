use core::alloc::Layout;
use crate::{chunk::ChunkMetadata, error::AllocError};

const ALIGNMENTS: [usize; 3] = [8, 16, 32];
const SIZES: [usize; 3] = [32, 64, 128];
const CHUNKS_PER_POOL: usize = 21; 

pub struct SlabAllocator {
    memory: [u8; 4096],
    chunks: [ChunkMetadata; 64],
    free_counts: [usize; 3],
}

impl SlabAllocator {
    pub const fn new() -> Self {
        const INIT_CHUNK: ChunkMetadata = ChunkMetadata::new(0, 0);
        
        SlabAllocator {
            memory: [0; 4096],
            chunks: [INIT_CHUNK; 64],
            free_counts: [0; 3],
        }
    }

    pub fn init(&mut self) {
        let mut offset = 0;
        let mut chunk_idx = 0;
        
        for (pool_idx, &size) in SIZES.iter().enumerate() {
            let pool_offset = pool_idx * CHUNKS_PER_POOL;
            self.free_counts[pool_idx] = CHUNKS_PER_POOL;
            
            for i in 0..CHUNKS_PER_POOL {
                if chunk_idx < self.chunks.len() {
                    self.chunks[chunk_idx] = ChunkMetadata::new(offset, size);
                    offset += size;
                    chunk_idx += 1;
                }
            }
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocError> {
        let size = layout.size();
        let align = layout.align();
        
        let pool_idx = SIZES.iter()
            .position(|&s| size <= s)
            .ok_or(AllocError::SizeTooLarge)?;
            
        if align > ALIGNMENTS[pool_idx] {
            return Err(AllocError::InvalidAlign);
        }
        
        let start_idx = pool_idx * CHUNKS_PER_POOL;
        let end_idx = start_idx + CHUNKS_PER_POOL;
        
        for i in start_idx..end_idx {
            if !self.chunks[i].is_used {
                self.chunks[i].is_used = true;
                self.free_counts[pool_idx] -= 1;
                return Ok(unsafe { self.memory.as_mut_ptr().add(self.chunks[i].offset) });
            }
        }
        
        Err(AllocError::NoSpace)
    }

    pub fn free(&mut self, ptr: *mut u8) -> Result<(), AllocError> {
        let offset = unsafe {
            ptr.offset_from(self.memory.as_ptr()) as usize
        };
        
        for (i, chunk) in self.chunks.iter_mut().enumerate() {
            if chunk.offset == offset {
                if !chunk.is_used {
                    return Err(AllocError::InvalidFree);
                }
                
                chunk.is_used = false;
                let pool_idx = SIZES.iter()
                    .position(|&s| chunk.size <= s)
                    .unwrap();
                self.free_counts[pool_idx] += 1;
                return Ok(());
            }
        }
        
        Err(AllocError::InvalidPointer)
    }

    pub fn get_stats(&self) -> Stats {
        Stats {
            free_small: self.free_counts[0],
            free_medium: self.free_counts[1],
            free_large: self.free_counts[2],
            total_chunks: self.chunks.len(),
        }
    }
}

pub struct Stats {
    pub free_small: usize,
    pub free_medium: usize,
    pub free_large: usize,
    pub total_chunks: usize,
}