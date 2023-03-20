// use byteorder::{ByteOrder, LittleEndian};
//
// pub const SCHEMA_ID: i32 = 101;
// pub const SCHEMA_VERSION: i32 = 7;
// pub const ENCODED_LENGTH: usize = 8;
//
// pub struct MessageHeaderEncoder<'a> {
//     buffer: &'a mut [u8],
//     offset: usize,
// }
//
// impl<'a> MessageHeaderEncoder<'a> {
//     pub fn wrap(buffer: &'a mut [u8], offset: usize) -> Self {
//         Self { buffer, offset }
//     }
//
//     pub fn buffer(&self) -> &[u8] {
//         &self.buffer
//     }
//
//     pub fn offset(&self) -> usize {
//         self.offset
//     }
//
//     pub fn encoded_length() -> usize {
//         ENCODED_LENGTH
//     }
//
//     pub fn sbe_schema_id() -> i32 {
//         SCHEMA_ID
//     }
//
//     pub fn sbe_schema_version() -> i32 {
//         SCHEMA_VERSION
//     }
//
//     pub fn block_length(&mut self, value: i32) -> &mut Self {
//         LittleEndian::write_i16(&mut self.buffer[self.offset..], value as i16);
//         self
//     }
//
//     pub fn template_id(&mut self, value: i32) -> &mut Self {
//         LittleEndian::write_i16(&mut self.buffer[self.offset + 2..], value as i16);
//         self
//     }
//
//     pub fn schema_id(&mut self, value: i32) -> &mut Self {
//         LittleEndian::write_i16(&mut self.buffer[self.offset + 4..], value as i16);
//         self
//     }
//
//     pub fn version(&mut self, value: i32) -> &mut Self {
//         LittleEndian::write_i16(&mut self.buffer[self.offset + 6..], value as i16);
//         self
//     }
//
//     pub fn block_length_encoding_offset() -> usize {
//         0
//     }
//
//     pub fn block_length_encoding_length() -> usize {
//         2
//     }
//
//     pub fn block_length_null_value() -> i32 {
//         65_535
//     }
//
//     pub fn block_length_min_value() -> i32 {
//         0
//     }
//
//     pub fn block_length_max_value() -> i32 {
//         65_534
//     }
//
//     pub fn template_id_encoding_offset() -> usize {
//         2
//     }
//
//     pub fn template_id_encoding_length() -> usize {
//         2
//     }
//
//     pub fn template_id_null_value() -> i32 {
//         65_535
//     }
//
//     pub fn template_id_min_value() -> i32 {
//         0
//     }
//
//     pub fn template_id_max_value() -> i32 {
//         65_534
//     }
//
//     pub fn schema_id_encoding_offset() -> usize {
//         4
//     }
//
//     pub fn schema_id_encoding_length() -> usize {
//         2
//     }
//
//     pub fn schema_id_null_value() -> i32 {
//         65_535
//     }
//
//     pub fn schema_id_min_value() -> i32 {
//         0
//     }
//
//     pub fn schema_id_max_value() -> i32 {
//         65_534
//     }
//
//     pub fn version_encoding_offset() -> usize {
//         6
//     }
//
//     pub fn version_encoding_length() -> usize {
//         2
//     }
//
//     pub fn version_null_value() -> i32 {
//         65_535
//     }
//
//     pub fn version_min_value() -> i32 {
//         0
//     }
//
//     pub fn version_max_value() -> i32 {
//         65_534
//     }
// }
