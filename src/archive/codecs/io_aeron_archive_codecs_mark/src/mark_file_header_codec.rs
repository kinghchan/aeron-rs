use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 128;
pub const SBE_TEMPLATE_ID: u16 = 200;
pub const SBE_SCHEMA_ID: u16 = 100;
pub const SBE_SCHEMA_VERSION: u16 = 1;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct MarkFileHeaderEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for MarkFileHeaderEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for MarkFileHeaderEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> MarkFileHeaderEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// primitive field 'version'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 4
        #[inline]
        pub fn version(&mut self, value: i32) {
            let offset = self.offset;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'activityTimestamp'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        #[inline]
        pub fn activity_timestamp(&mut self, value: i64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'startTimestamp'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 16
        /// - encodedLength: 8
        #[inline]
        pub fn start_timestamp(&mut self, value: i64) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'pid'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 24
        /// - encodedLength: 8
        #[inline]
        pub fn pid(&mut self, value: i64) {
            let offset = self.offset + 24;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'controlStreamId'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 32
        /// - encodedLength: 4
        #[inline]
        pub fn control_stream_id(&mut self, value: i32) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'localControlStreamId'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 36
        /// - encodedLength: 4
        #[inline]
        pub fn local_control_stream_id(&mut self, value: i32) {
            let offset = self.offset + 36;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'eventsStreamId'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 40
        /// - encodedLength: 4
        #[inline]
        pub fn events_stream_id(&mut self, value: i32) {
            let offset = self.offset + 40;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'headerLength'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 44
        /// - encodedLength: 4
        #[inline]
        pub fn header_length(&mut self, value: i32) {
            let offset = self.offset + 44;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'errorBufferLength'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: 0
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 48
        /// - encodedLength: 4
        #[inline]
        pub fn error_buffer_length(&mut self, value: i32) {
            let offset = self.offset + 48;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// VAR_DATA ENCODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn control_channel(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 4 + data_length);
            self.get_buf_mut().put_u32_at(limit, data_length as u32);
            self.get_buf_mut().put_slice_at(limit + 4, value);
        }

        /// VAR_DATA ENCODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn local_control_channel(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 4 + data_length);
            self.get_buf_mut().put_u32_at(limit, data_length as u32);
            self.get_buf_mut().put_slice_at(limit + 4, value);
        }

        /// VAR_DATA ENCODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn events_channel(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 4 + data_length);
            self.get_buf_mut().put_u32_at(limit, data_length as u32);
            self.get_buf_mut().put_slice_at(limit + 4, value);
        }

        /// VAR_DATA ENCODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn aeron_directory(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 4 + data_length);
            self.get_buf_mut().put_u32_at(limit, data_length as u32);
            self.get_buf_mut().put_slice_at(limit + 4, value);
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct MarkFileHeaderDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for MarkFileHeaderDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for MarkFileHeaderDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> MarkFileHeaderDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn version(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn activity_timestamp(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 8)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn start_timestamp(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 16)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn pid(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 24)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn control_stream_id(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 32)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn local_control_stream_id(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 36)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn events_stream_id(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 40)
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn header_length(&self) -> Option<i32> {
            if self.acting_version < 1 {
                return None;
            }

            let value = self.get_buf().get_i32_at(self.offset + 44);
            if value == 0_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '0' }
        #[inline]
        pub fn error_buffer_length(&self) -> Option<i32> {
            if self.acting_version < 1 {
                return None;
            }

            let value = self.get_buf().get_i32_at(self.offset + 48);
            if value == 0_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// VAR_DATA DECODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn control_channel_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u32_at(offset) as usize;
            self.set_limit(offset + 4 + data_length);
            (offset + 4, data_length)
        }

        #[inline]
        pub fn control_channel_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn local_control_channel_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u32_at(offset) as usize;
            self.set_limit(offset + 4 + data_length);
            (offset + 4, data_length)
        }

        #[inline]
        pub fn local_control_channel_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn events_channel_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u32_at(offset) as usize;
            self.set_limit(offset + 4 + data_length);
            (offset + 4, data_length)
        }

        #[inline]
        pub fn events_channel_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn aeron_directory_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u32_at(offset) as usize;
            self.set_limit(offset + 4 + data_length);
            (offset + 4, data_length)
        }

        #[inline]
        pub fn aeron_directory_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

    }

} // end decoder

