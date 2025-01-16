use core::alloc::Layout;
use crate::{chunk::ChunkMetadata, error::AllocError};

const SIZES: [usize; 3] = [32, 64, 128];
const ALIGNMENTS: [usize; 3] = [8, 16, 32];
const CHUNKS_PER_POOL: usize = 21;
const TOTAL_MEMORY: usize = 4096;
const TOTAL_CHUNKS: usize = CHUNKS_PER_POOL * 3;

// Vérifications statiques via const generics
const _: () = assert!(TOTAL_CHUNKS <= 64);
const _: () = assert!(SIZES[0] >= ALIGNMENTS[0]);
const _: () = assert!(SIZES[1] >= ALIGNMENTS[1]);
const _: () = assert!(SIZES[2] >= ALIGNMENTS[2]);

#[repr(C, align(32))]
pub struct SlabAllocator {
    memory: [u8; TOTAL_MEMORY],
    chunks: [ChunkMetadata; 64],
    free_lists: [Option<usize>; 3],
    free_counts: [usize; 3],
}

impl SlabAllocator {
    #[inline]
    pub const fn new() -> Self {
        const INIT_CHUNK: ChunkMetadata = ChunkMetadata::new(0, 0, None);
        SlabAllocator {
            memory: [0; TOTAL_MEMORY],
            chunks: [INIT_CHUNK; 64],
            free_lists: [None; 3],
            free_counts: [0; 3],
        }
    }

    #[inline]
    pub fn init(&mut self) {
        let mut offset = align_up(0, ALIGNMENTS[2]);
        
        for pool_idx in 0..3 {
            self.free_counts[pool_idx] = CHUNKS_PER_POOL;
            let mut prev = None;
            
            for chunk_idx in (pool_idx * CHUNKS_PER_POOL)..((pool_idx + 1) * CHUNKS_PER_POOL) {
                self.chunks[chunk_idx] = ChunkMetadata::new(
                    offset,
                    SIZES[pool_idx],
                    prev
                );
                offset = align_up(offset + SIZES[pool_idx], ALIGNMENTS[pool_idx]);
                prev = Some(chunk_idx);
            }
            
            self.free_lists[pool_idx] = prev;
        }
    }

    #[inline]
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocError> {
        if layout.size() > SIZES[2] {
            return Err(AllocError::SizeTooLarge);
        }

        let pool_idx = self.get_pool_index(layout)?;
        
        if let Some(chunk_idx) = self.free_lists[pool_idx].take() {
            let chunk = &mut self.chunks[chunk_idx];
            self.free_lists[pool_idx] = chunk.next();
            chunk.mark_used();
            self.free_counts[pool_idx] = self.free_counts[pool_idx].saturating_sub(1);
            
            Ok(unsafe { self.memory.as_mut_ptr().add(chunk.offset()) })
        } else {
            Err(AllocError::NoSpace)
        }
    }

    #[inline]
    pub fn free(&mut self, ptr: *mut u8) -> Result<(), AllocError> {
        if !self.is_ptr_valid(ptr) {
            return Err(AllocError::InvalidPointer);
        }

        let offset = unsafe { ptr.offset_from(self.memory.as_ptr()) as usize };
        let chunk_idx = self.find_chunk_by_offset(offset)?;
        let size = self.chunks[chunk_idx].size();
        let pool_idx = self.get_chunk_pool_index(size);
        
        let chunk = &mut self.chunks[chunk_idx];
        chunk.mark_free(self.free_lists[pool_idx]);
        self.free_lists[pool_idx] = Some(chunk_idx);
        self.free_counts[pool_idx] += 1;
        
        Ok(())
    }

    #[inline]
    fn find_chunk_by_offset(&self, offset: usize) -> Result<usize, AllocError> {
        let chunk_estimate = offset / SIZES[0];
        let search_range = (chunk_estimate.saturating_sub(1))..=(chunk_estimate + 1);
        
        for idx in search_range {
            if idx < self.chunks.len() && self.chunks[idx].offset() == offset {
                if !self.chunks[idx].is_used() {
                    return Err(AllocError::InvalidFree);
                }
                return Ok(idx);
            }
        }
        
        Err(AllocError::InvalidPointer)
    }

    #[inline]
    pub fn get_stats(&self) -> Stats {
        Stats {
            free_small: self.free_counts[0],
            free_medium: self.free_counts[1],
            free_large: self.free_counts[2],
            total_chunks: TOTAL_CHUNKS,
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

#[derive(Debug)]
pub struct Stats {
    pub free_small: usize,
    pub free_medium: usize,
    pub free_large: usize,
    pub total_chunks: usize,
}

#[inline]
const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}