#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum RecordingState {
    INVALID = 0_i32, 
    VALID = 1_i32, 
    NullVal = -2147483648_i32, 
}
impl Default for RecordingState {
    #[inline]
    fn default() -> Self { RecordingState::NullVal }
}
impl From<i32> for RecordingState {
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0_i32 => Self::INVALID, 
            1_i32 => Self::VALID, 
            _ => Self::NullVal,
        }
    }
}
