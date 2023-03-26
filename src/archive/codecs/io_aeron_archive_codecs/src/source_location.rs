#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum SourceLocation {
    LOCAL = 0_i32, 
    REMOTE = 1_i32, 
    NullVal = -2147483648_i32, 
}
impl Default for SourceLocation {
    #[inline]
    fn default() -> Self { SourceLocation::NullVal }
}
impl From<i32> for SourceLocation {
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0_i32 => Self::LOCAL, 
            1_i32 => Self::REMOTE, 
            _ => Self::NullVal,
        }
    }
}
