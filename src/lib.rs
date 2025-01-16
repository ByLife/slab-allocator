#![no_std]

mod slab;
mod chunk;
mod error;

pub use slab::SlabAllocator;
pub use error::AllocError;

#[cfg(test)]
mod tests;