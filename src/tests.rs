#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alloc_free_cycle() {
        let mut allocator = SlabAllocator::new();
        allocator.init();
        
        let layout = Layout::from_size_align(24, 8).unwrap();
        
        let ptr = allocator.alloc(layout).unwrap();
        assert!(!ptr.is_null());
        
        allocator.free(ptr).unwrap();
        
        let ptr2 = allocator.alloc(layout).unwrap();
        assert!(!ptr2.is_null());
    }
    
    #[test]
    fn test_invalid_free() {
        let mut allocator = SlabAllocator::new();
        allocator.init();
        
        let ptr = allocator.memory.as_mut_ptr();
        assert!(allocator.free(ptr).is_err());
    }
    
    #[test]
    fn test_double_free() {
        let mut allocator = SlabAllocator::new();
        allocator.init();
        
        let layout = Layout::from_size_align(24, 8).unwrap();
        let ptr = allocator.alloc(layout).unwrap();
        
        allocator.free(ptr).unwrap();
        assert!(allocator.free(ptr).is_err());
    }
}