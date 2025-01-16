#[derive(Debug)]
pub enum AllocError {
    NoSpace,
    InvalidFree,
    InvalidAlign,
    SizeTooLarge,
    InvalidPointer,
}