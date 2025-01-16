use core::alloc::Layout;
use crate::{chunk::ChunkMetadata, error::AllocError};

const SIZES: [usize; 3] = [32, 64, 128];
const ALIGNMENTS: [usize; 3] = [8, 16, 32];
const CHUNKS_PER_POOL: usize = 21;
const TOTAL_MEMORY: usize = 4096;

#[repr(C)]
pub struct SlabAllocator {
    memory: [u8; TOTAL_MEMORY],
    chunks: [ChunkMetadata; 64],
    free_counts: [usize; 3],
}

impl SlabAllocator {
    #[inline]
    pub const fn new() -> Self {
        const INIT_CHUNK: ChunkMetadata = ChunkMetadata::new(0, 0);
        
        SlabAllocator {
            memory: [0; TOTAL_MEMORY],
            chunks: [INIT_CHUNK; 64],
            free_counts: [0; 3],
        }
    }

    #[inline]
    pub fn init(&mut self) {
        let mut offset = 0;
        let mut chunk_idx = 0;
        
        offset = align_up(offset, ALIGNMENTS[2]);
        
        for (pool_idx, &size) in SIZES.iter().enumerate() {
            self.free_counts[pool_idx] = CHUNKS_PER_POOL;
            
            for _ in 0..CHUNKS_PER_POOL {
                if chunk_idx < self.chunks.len() {
                    self.chunks[chunk_idx] = ChunkMetadata::new(offset, size);
                    offset = align_up(offset + size, ALIGNMENTS[pool_idx]);
                    chunk_idx += 1;
                }
            }
        }
    }

    #[inline]
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocError> {
        if layout.size() > SIZES[2] {
            return Err(AllocError::SizeTooLarge);
        }

        let pool_idx = self.get_pool_index(layout)?;
        
        let start_idx = pool_idx * CHUNKS_PER_POOL;
        let end_idx = start_idx + CHUNKS_PER_POOL;
        
        for i in start_idx..end_idx {
            let chunk = &mut self.chunks[i];
            if !chunk.is_used() {
                chunk.mark_used();
                self.free_counts[pool_idx] = self.free_counts[pool_idx].saturating_sub(1);
                return Ok(unsafe { self.memory.as_mut_ptr().add(chunk.offset()) });
            }
        }
        
        Err(AllocError::NoSpace)
    }

    #[inline]
    pub fn free(&mut self, ptr: *mut u8) -> Result<(), AllocError> {
        if !self.is_ptr_valid(ptr) {
            return Err(AllocError::InvalidPointer);
        }

        let offset = unsafe {
            ptr.offset_from(self.memory.as_ptr()) as usize
        };
        
        let mut chunk_idx = None;
        let mut chunk_size = 0;
        
        for (idx, chunk) in self.chunks.iter().enumerate() {
            if chunk.offset() == offset {
                if !chunk.is_used() {
                    return Err(AllocError::InvalidFree);
                }
                chunk_idx = Some(idx);
                chunk_size = chunk.size();
                break;
            }
        }

        if let Some(idx) = chunk_idx {
            let pool_idx = self.get_chunk_pool_index(chunk_size);
            self.chunks[idx].mark_free();
            self.free_counts[pool_idx] += 1;
            Ok(())
        } else {
            Err(AllocError::InvalidPointer)
        }
    }

    #[inline]
    pub fn get_stats(&self) -> Stats {
        Stats {
            free_small: self.free_counts[0],
            free_medium: self.free_counts[1],
            free_large: self.free_counts[2],
            total_chunks: self.chunks.len(),
        }
    }
    
    #[inline]
    fn get_pool_index(&self, layout: Layout) -> Result<usize, AllocError> {
        let size = layout.size();
        let align = layout.align();
        
        for (idx, (&pool_size, &pool_align)) in SIZES.iter().zip(ALIGNMENTS.iter()).enumerate() {
            if size <= pool_size && align <= pool_align {
                return Ok(idx);
            }
        }
        
        Err(AllocError::SizeTooLarge)
    }

    #[inline]
    fn get_chunk_pool_index(&self, size: usize) -> usize {
        SIZES.iter()
            .position(|&s| size <= s)
            .unwrap_or(0)
    }

    #[inline]
    fn is_ptr_valid(&self, ptr: *mut u8) -> bool {
        let start = self.memory.as_ptr() as usize;
        let end = start + TOTAL_MEMORY;
        let ptr_addr = ptr as usize;
        
        ptr_addr >= start && ptr_addr < end
    }
}

#[inline]
const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[derive(Debug)]
pub struct Stats {
    pub free_small: usize,
    pub free_medium: usize,
    pub free_large: usize,
    pub total_chunks: usize,
}