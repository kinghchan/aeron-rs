#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum BooleanType {
    FALSE = 0_i32, 
    TRUE = 1_i32, 
    NullVal = -2147483648_i32, 
}
impl Default for BooleanType {
    #[inline]
    fn default() -> Self { BooleanType::NullVal }
}
impl From<i32> for BooleanType {
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0_i32 => Self::FALSE, 
            1_i32 => Self::TRUE, 
            _ => Self::NullVal,
        }
    }
}
