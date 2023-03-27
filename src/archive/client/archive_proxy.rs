use std::time::Duration;
use std::sync::Arc;
use crate::agrona::concurrent::system_nano_clock;
// use crate::archive::client::aeron_archive::AeronArchive;
use crate::concurrent::strategies::YieldingIdleStrategy;
use crate::publication::Publication;
use crate::agrona::concurrent::system_nano_clock::NanoClock;
use io_aeron_archive_codecs::*;
use crate::agrona::expandable_array_buffer::ExpandableArrayBuffer;
use crate::concurrent::atomic_buffer::{AlignedBuffer, AtomicBuffer};
use crate::concurrent::atomic_counter::AtomicCounter;
use crate::archive::client::aeron_archive::MESSAGE_TIMEOUT_DEFAULT_NS;
use crate::archive::client::configuration::PROTOCOL_SEMANTIC_VERSION;

pub const DEFAULT_RETRY_ATTEMPTS: i32 = 3;

pub struct ArchiveProxy<'a> {
    connect_timeout_ns: i64,
    retry_attempts: i32,
    // IC: Yielding only for now
    retry_idle_strategy: YieldingIdleStrategy,
    // Box<dyn IdleStrategy,
    nano_clock: NanoClock,
    // credentials_supplier: Box<dyn CredentialsSupplier,
    // IC: ExpandableArrayBuffer just a container for a Vec<u8>
    buffer: ExpandableArrayBuffer,
    publication: Publication,
    // IC: Not sure if this works...
    message_header: MessageHeaderEncoder<WriteBuf<'a>>,
    // Add the rest of the encoders for each message type here
    start_recording_request: StartRecordingRequestEncoder<'a>,
    start_recording_request2: StartRecordingRequest2Encoder<'a>,
    stop_recording_request: StopRecordingRequestEncoder<'a>,
    stop_recording_subscription_request: StopRecordingSubscriptionRequestEncoder<'a>,
    stop_recording_by_identity_request: StopRecordingByIdentityRequestEncoder<'a>,
    replay_request: ReplayRequestEncoder<'a>,
    stop_replay_request: StopReplayRequestEncoder<'a>,
    list_recordings_request: ListRecordingsRequestEncoder<'a>,
    list_recordings_for_uri_request: ListRecordingsForUriRequestEncoder<'a>,
    list_recording_request: ListRecordingRequestEncoder<'a>,
    extend_recording_request: ExtendRecordingRequestEncoder<'a>,
    extend_recording_request2: ExtendRecordingRequest2Encoder<'a>,
    recording_position_request: RecordingPositionRequestEncoder<'a>,
    truncate_recording_request: TruncateRecordingRequestEncoder<'a>,
    purge_recording_request: PurgeRecordingRequestEncoder<'a>,
    stop_position_request: StopPositionRequestEncoder<'a>,
    find_last_matching_recording_request: FindLastMatchingRecordingRequestEncoder<'a>,
    list_recording_subscriptions_request: ListRecordingSubscriptionsRequestEncoder<'a>,
    bounded_replay_request: BoundedReplayRequestEncoder<'a>,
    stop_all_replays_request: StopAllReplaysRequestEncoder<'a>,
    replicate_request: ReplicateRequest2Encoder<'a>,
    stop_replication_request: StopReplicationRequestEncoder<'a>,
    start_position_request: StartPositionRequestEncoder<'a>,
    detach_segments_request: DetachSegmentsRequestEncoder<'a>,
    delete_detached_segments_request: DeleteDetachedSegmentsRequestEncoder<'a>,
    purge_segments_request: PurgeSegmentsRequestEncoder<'a>,
    attach_segments_request: AttachSegmentsRequestEncoder<'a>,
    migrate_segments_request: MigrateSegmentsRequestEncoder<'a>,
}

impl ArchiveProxy<'_> {
    // In Rust, you can't create a struct in 1 go where a field references another field.
    // Which means I have to first create the buffer, and then set the encoders. 
    // https://stackoverflow.com/questions/27092273/how-to-make-a-struct-where-one-of-the-fields-refers-to-another-field
    pub fn set_encoders(&mut self) {
        // Initialize the rest of the encoders for each message type here
        // let mut write_buf = WriteBuf::new(self.buffer.clone().as_mutable_slice());
        // self.start_recording_request = Some(StartRecordingRequestEncoder::default().wrap(write_buf, 0));
        // write_bufself.start_recording_request2 = Some(StartRecordingRequest2Encoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.stop_recording_request = Some(StopRecordingRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.stop_recording_subscription_request = Some(StopRecordingSubscriptionRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.stop_recording_by_identity_request = Some(StopRecordingByIdentityRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.replay_request = Some(ReplayRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.stop_replay_request = Some(StopReplayRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.list_recordings_request = Some(ListRecordingsRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.list_recordings_for_uri_request = Some(ListRecordingsForUriRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.list_recording_request = Some(ListRecordingRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.extend_recording_request = Some(ExtendRecordingRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.extend_recording_request2 = Some(ExtendRecordingRequest2Encoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.recording_position_request = Some(RecordingPositionRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.truncate_recording_request = Some(TruncateRecordingRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.purge_recording_request = Some(PurgeRecordingRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.stop_position_request = Some(StopPositionRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.find_last_matching_recording_request = Some(FindLastMatchingRecordingRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.list_recording_subscriptions_request = Some(ListRecordingSubscriptionsRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.bounded_replay_request = Some(BoundedReplayRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.stop_all_replays_request = Some(StopAllReplaysRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.replicate_request = Some(ReplicateRequest2Encoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.stop_replication_request = Some(StopReplicationRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.start_position_request = Some(StartPositionRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.detach_segments_request = Some(DetachSegmentsRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.delete_detached_segments_request = Some(DeleteDetachedSegmentsRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.purge_segments_request = Some(PurgeSegmentsRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.attach_segments_request = Some(AttachSegmentsRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
        // self.migrate_segments_request = Some(MigrateSegmentsRequestEncoder::default().wrap(WriteBuf::new(self.buffer.clone().as_mutable_slice()), 0));
    }

    pub fn new(publication: Publication) -> Self {
        let mut buffer = ExpandableArrayBuffer::with_capacity(256);

        Self {
            connect_timeout_ns: MESSAGE_TIMEOUT_DEFAULT_NS,
            retry_attempts: DEFAULT_RETRY_ATTEMPTS,
            retry_idle_strategy: YieldingIdleStrategy {},
            nano_clock: system_nano_clock::NanoClock,
            // credentials_supplier: Box::new(NullCredentialsSupplier::default()),
            buffer: buffer,
            publication,
            message_header: MessageHeaderEncoder::default(),
            start_recording_request: StartRecordingRequestEncoder::default(),
            start_recording_request2: StartRecordingRequest2Encoder::default(),
            stop_recording_request: StopRecordingRequestEncoder::default(),
            stop_recording_subscription_request: StopRecordingSubscriptionRequestEncoder::default(),
            stop_recording_by_identity_request: StopRecordingByIdentityRequestEncoder::default(),
            replay_request: ReplayRequestEncoder::default(),
            stop_replay_request: StopReplayRequestEncoder::default(),
            list_recordings_request: ListRecordingsRequestEncoder::default(),
            list_recordings_for_uri_request: ListRecordingsForUriRequestEncoder::default(),
            list_recording_request: ListRecordingRequestEncoder::default(),
            extend_recording_request: ExtendRecordingRequestEncoder::default(),
            extend_recording_request2: ExtendRecordingRequest2Encoder::default(),
            recording_position_request: RecordingPositionRequestEncoder::default(),
            truncate_recording_request: TruncateRecordingRequestEncoder::default(),
            purge_recording_request: PurgeRecordingRequestEncoder::default(),
            stop_position_request: StopPositionRequestEncoder::default(),
            find_last_matching_recording_request: FindLastMatchingRecordingRequestEncoder::default(),
            list_recording_subscriptions_request: ListRecordingSubscriptionsRequestEncoder::default(),
            bounded_replay_request: BoundedReplayRequestEncoder::default(),
            stop_all_replays_request: StopAllReplaysRequestEncoder::default(),
            replicate_request: ReplicateRequest2Encoder::default(),
            stop_replication_request: StopReplicationRequestEncoder::default(),
            start_position_request: StartPositionRequestEncoder::default(),
            detach_segments_request: DetachSegmentsRequestEncoder::default(),
            delete_detached_segments_request: DeleteDetachedSegmentsRequestEncoder::default(),
            purge_segments_request: PurgeSegmentsRequestEncoder::default(),
            attach_segments_request: AttachSegmentsRequestEncoder::default(),
            migrate_segments_request: MigrateSegmentsRequestEncoder::default(),
        }
    }

    pub fn connect(&mut self, response_channel: String, response_stream_id: i32, correlation_id: i64) -> bool {
        let mut connect_request_encoder: AuthConnectRequestEncoder = AuthConnectRequestEncoder::default();
        connect_request_encoder = connect_request_encoder.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        connect_request_encoder = connect_request_encoder.header(0).parent().unwrap(); // don't use unwrap
        connect_request_encoder.correlation_id(correlation_id);
        connect_request_encoder.response_stream_id(response_stream_id);
        connect_request_encoder.version(PROTOCOL_SEMANTIC_VERSION);
        connect_request_encoder.response_channel(response_channel.as_bytes());
        offer_with_timeout(connect_request_encoder.encoded_length(), null)
    }

    pub fn try_connect(&mut self, response_channel: String, response_stream_id: i32, correlation_id: i64) -> bool {
        let encoded_credentials = self.credentials_supplier.encoded_credentials();

        let mut connect_request_encoder: AuthConnectRequestEncoder = AuthConnectRequestEncoder::default();
        connect_request_encoder = connect_request_encoder.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        connect_request_encoder = connect_request_encoder.header(0).parent().unwrap(); // don't use unwrap
        connect_request_encoder.correlation_id(correlation_id);
        connect_request_encoder.response_stream_id(response_stream_id);
        connect_request_encoder.version(PROTOCOL_SEMANTIC_VERSION);
        connect_request_encoder.response_channel(response_channel.as_bytes());
        connect_request_encoder.put_encoded_credentials(encoded_credentials.as_ref(), 0, encoded_credentials.len());

        let length = connect_request_encoder.encoded_length();

        publication.offer(buffer, 0, length) > 0
    }

    pub fn connect_with_client_invoker(
        &mut self,
        response_channel: String,
        response_stream_id: i32,
        correlation_id: i64,
        aeron_client_invoker: AgentInvoker,
    ) -> bool {
        let encoded_credentials = self.credentials_supplier.encoded_credentials();

        let mut connect_request_encoder: AuthConnectRequestEncoder = AuthConnectRequestEncoder::default();
        connect_request_encoder = connect_request_encoder.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        connect_request_encoder = connect_request_encoder.header(0).parent().unwrap(); // don't use unwrap
        connect_request_encoder.correlation_id(correlation_id);
        connect_request_encoder.response_stream_id(response_stream_id);
        connect_request_encoder.version(PROTOCOL_SEMANTIC_VERSION);
        connect_request_encoder.response_channel(response_channel.as_bytes());
        connect_request_encoder.put_encoded_credentials(encoded_credentials.as_ref(), 0, encoded_credentials.len());

        offer_with_timeout(connect_request_encoder.encoded_length(), aeron_client_invoker)
    }

    pub fn keep_alive(&mut self, control_session_id: i64, correlation_id: i64) -> bool {
        let mut keep_alive_request_encoder: KeepAliveRequestEncoder = KeepAliveRequestEncoder::default();
        keep_alive_request_encoder = keep_alive_request_encoder.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        keep_alive_request_encoder = keep_alive_request_encoder.header(0).parent().unwrap(); // don't use unwrap
        keep_alive_request_encoder.control_session_id(control_session_id);
        keep_alive_request_encoder.correlation_id(correlation_id);

        offer(keep_alive_request_encoder.encoded_length())
    }

    pub fn close_session(&mut self, control_session_id: i64) -> bool {
        let mut close_session_request_encoder: CloseSessionRequestEncoder = CloseSessionRequestEncoder::default();
        close_session_request_encoder = close_session_request_encoder.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        close_session_request_encoder = close_session_request_encoder.header(0).parent().unwrap(); // don't use unwrap
        close_session_request_encoder.control_session_id(control_session_id);

        offer(close_session_request_encoder.encoded_length())
    }

    pub fn try_challenge_response(&mut self, encoded_credentials: &[u8], correlation_id: i64, control_session_id: i64) -> bool {
        let mut challenge_response_encoder: ChallengeResponseEncoder = ChallengeResponseEncoder::default();
        challenge_response_encoder = challenge_response_encoder.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        challenge_response_encoder = challenge_response_encoder.header(0).parent().unwrap(); // don't use unwrap
        challenge_response_encoder.control_session_id(control_session_id);
        challenge_response_encoder.correlation_id(correlation_id);
        challenge_response_encoder.put_encoded_credentials(encoded_credentials);

        let length = message_header_codec::ENCODED_LENGTH + challenge_response_encoder.encoded_length();

        self.publication.offer(self.buffer.byte_array.as_slice(), 0, length) > 0
    }

    pub fn start_recording(&mut self, channel: String, stream_id: i32, source_location: SourceLocation, correlation_id: i64, control_session_id: i64) -> bool {
        let mut start_recording_request_encoder: StartRecordingRequestEncoder = StartRecordingRequestEncoder::default();
        start_recording_request_encoder = start_recording_request_encoder.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        start_recording_request_encoder = start_recording_request_encoder.header(0).parent().unwrap(); // don't use unwrap
        start_recording_request_encoder.control_session_id(control_session_id);
        start_recording_request_encoder.correlation_id(correlation_id);
        start_recording_request_encoder.stream_id(stream_id);
        start_recording_request_encoder.source_location(source_location);
        start_recording_request_encoder.channel(channel);
        offer(start_recording_request_encoder.encoded_length())
    }

    pub fn start_recording_2(
        &mut self,
        channel: String,
        stream_id: i32,
        source_location: SourceLocation,
        auto_stop: bool,
        correlation_id: i64,
        control_session_id: i64
    ) -> bool {
        let mut start_recording_request2: StartRecordingRequest2Encoder = StartRecordingRequest2Encoder::default();
        start_recording_request2 = start_recording_request2.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        start_recording_request2 = start_recording_request2.header(0).parent().unwrap();
        start_recording_request2.control_session_id(control_session_id);
        start_recording_request2.correlation_id(correlation_id);
        start_recording_request2.stream_id(stream_id);
        start_recording_request2.source_location(source_location);
        start_recording_request2.auto_stop(if auto_stop { BooleanType::TRUE } else { BooleanType::FALSE });
        start_recording_request2.channel(channel.as_bytes());
        offer(start_recording_request2.encoded_length())
    }

    pub fn stop_recording(&mut self, channel: String, stream_id: i32, correlation_id: i64, control_session_id: i64, ) -> bool {
        let mut stop_recording_request: StopRecordingRequestEncoder = StopRecordingRequestEncoder::default();
        stop_recording_request = stop_recording_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        stop_recording_request = stop_recording_request.header(0).parent().unwrap(); // don't use unwrap
        stop_recording_request.control_session_id(control_session_id);
        stop_recording_request.correlation_id(correlation_id);
        stop_recording_request.stream_id(stream_id);
        stop_recording_request.channel(channel.as_bytes());
        offer(stop_recording_request.encoded_length())
    }

    pub fn stop_recording_subscription(
        &mut self,
        subscription_id: i64,
        correlation_id: i64,
        control_session_id: i64
    ) -> bool {
        let mut stop_recording_subscription_request = StopRecordingSubscriptionRequestEncoder::default();
        stop_recording_subscription_request = stop_recording_subscription_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        stop_recording_subscription_request = stop_recording_subscription_request.header(0).parent().unwrap();
        stop_recording_subscription_request.control_session_id(control_session_id);
        stop_recording_subscription_request.correlation_id(correlation_id);
        stop_recording_subscription_request.subscription_id(subscription_id);
        offer(stop_recording_subscription_request.encoded_length())
    }

    pub fn stop_recording_by_identity(
        &mut self,
        recording_id: i64,
        correlation_id: i64,
        control_session_id: i64
    ) -> bool {
        let mut stop_recording_by_identity_request = StopRecordingByIdentityRequestEncoder::default();
        stop_recording_by_identity_request = stop_recording_by_identity_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        stop_recording_by_identity_request = stop_recording_by_identity_request.header(0).parent().unwrap();
        stop_recording_by_identity_request.control_session_id(control_session_id);
        stop_recording_by_identity_request.correlation_id(correlation_id);
        stop_recording_by_identity_request.recording_id(recording_id);
        offer(stop_recording_by_identity_request.encoded_length())
    }

    pub fn replay(
        recording_id: i64,
        replay_channel: String,
        replay_stream_id: i32,
        replay_params: ReplayParams,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        if replay_params.is_bounded() {
            bounded_replay(
                recording_id,
                replay_params.position(),
                replay_params.length(),
                replay_params.bounding_limit_counter_id(),
                replay_channel,
                replay_stream_id,
                correlation_id,
                control_session_id,
                replay_params.file_io_max_length(),
            )
        } else {
            replay(
                recording_id,
                replay_params.position(),
                replay_params.length(),
                replay_channel,
                replay_stream_id,
                correlation_id,
                control_session_id,
                replay_params.file_io_max_length(),
            )
        }
    }

    pub fn replay(
        &mut self,
        recording_id: i64,
        position: i64,
        length: i64,
        replay_channel: String,
        replay_stream_id: i32,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        self.replay(
            recording_id,
            position,
            length,
            replay_channel,
            replay_stream_id,
            correlation_id,
            control_session_id,
            Aeron::NULL_VALUE,
        )
    }

    pub fn stop_replay(
        &mut self,
        replay_session_id: i64,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut stop_replay_request = StopReplayRequestEncoder::default();
        stop_replay_request = stop_replay_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        stop_replay_request = stop_replay_request.header(0).parent().unwrap(); // don't use unwrap
        stop_replay_request.correlation_id(correlation_id);
        stop_replay_request.control_session_id(control_session_id);
        stop_replay_request.replay_session_id(replay_session_id);
        offer(stop_replay_request.encoded_length())
    }

    pub fn stop_all_replays(&mut self, recording_id: i64, correlation_id: i64, control_session_id: i64) -> bool {
        let mut stop_all_replays_request: StopAllReplaysRequestEncoder = StopAllReplaysRequestEncoder::default();
        stop_all_replays_request = stop_all_replays_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        stop_all_replays_request = stop_all_replays_request.header(0).parent().unwrap();
        stop_all_replays_request.control_session_id(control_session_id);
        stop_all_replays_request.correlation_id(correlation_id);
        stop_all_replays_request.recording_id(recording_id);
        offer(stop_all_replays_request.encoded_length())
    }

    pub fn list_recordings(
        &mut self,
        from_recording_id: i64,
        record_count: i32,
        correlation_id: i64,
        control_session_id: i64
    ) -> bool {
        let mut list_recordings_request: ListRecordingsRequestEncoder = ListRecordingsRequestEncoder::default();
        list_recordings_request = list_recordings_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        list_recordings_request = list_recordings_request.header(0).parent().unwrap(); // don't use unwrap
        list_recordings_request.control_session_id(control_session_id);
        list_recordings_request.correlation_id(correlation_id);
        list_recordings_request.from_recording_id(from_recording_id);
        list_recordings_request.record_count(record_count);
        offer(list_recordings_request.encoded_length())
    }

    pub fn list_recordings_for_uri(
        &mut self,
        from_recording_id: i64,
        record_count: i32,
        channel_fragment: String,
        stream_id: i32,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut list_recordings_for_uri_request = ListRecordingsForUriRequestEncoder::default();
        list_recordings_for_uri_request = list_recordings_for_uri_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        list_recordings_for_uri_request = list_recordings_for_uri_request.header(0).parent().unwrap(); // don't use unwrap
        list_recordings_for_uri_request.control_session_id(control_session_id);
        list_recordings_for_uri_request.correlation_id(correlation_id);
        list_recordings_for_uri_request.from_recording_id(from_recording_id);
        list_recordings_for_uri_request.record_count(record_count);
        list_recordings_for_uri_request.stream_id(stream_id);
        list_recordings_for_uri_request.channel(channel_fragment.as_bytes());
        offer(list_recordings_for_uri_request.encoded_length())
    }

    pub fn list_recording(&mut self, recording_id: i64, correlation_id: i64, control_session_id: i64) -> bool {
        let mut list_recording_request: ListRecordingRequestEncoder = ListRecordingRequestEncoder::default();
        list_recording_request = list_recording_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        list_recording_request = list_recording_request.header(0).parent().unwrap(); // don't use unwrap
        list_recording_request.control_session_id(control_session_id);
        list_recording_request.correlation_id(correlation_id);
        list_recording_request.recording_id(recording_id);
        offer(list_recording_request.encoded_length())
    }

    pub fn extend_recording(&mut self, channel: String, stream_id: i32, source_location: SourceLocation, recording_id: i64, correlation_id: i64, control_session_id: i64) -> bool {
        let mut extend_recording_request = ExtendRecordingRequestEncoder::default();
        extend_recording_request = extend_recording_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        extend_recording_request = extend_recording_request.header(0).parent().unwrap();
        extend_recording_request.correlation_id(correlation_id);
        extend_recording_request.recording_id(recording_id);
        extend_recording_request.stream_id(stream_id);
        extend_recording_request.source_location(source_location);
        extend_recording_request.channel(channel.as_bytes());
        offer(extend_recording_request.encoded_length())
    }

    pub fn extend_recording_2(
        &mut self,
        channel: String,
        stream_id: i32,
        source_location: SourceLocation,
        auto_stop: bool,
        recording_id: i64,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut extend_recording_request_2: ExtendRecordingRequest2Encoder = ExtendRecordingRequest2Encoder::default();
        extend_recording_request_2 = extend_recording_request_2.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        extend_recording_request_2 = extend_recording_request_2.header(0).parent().unwrap(); // don't use unwrap
        extend_recording_request_2.control_session_id(control_session_id);
        extend_recording_request_2.correlation_id(correlation_id);
        extend_recording_request_2.recording_id(recording_id);
        extend_recording_request_2.stream_id(stream_id);
        extend_recording_request_2.source_location(source_location);
        extend_recording_request_2.auto_stop(if auto_stop { BooleanType::TRUE } else { BooleanType::FALSE });
        extend_recording_request_2.channel(channel.as_bytes());
        offer(extend_recording_request_2.encoded_length())
    }

    pub fn get_recording_position(&mut self, recording_id: i64, correlation_id: i64, control_session_id: i64) -> bool {
        let mut recording_position_request = RecordingPositionRequestEncoder::default();
        recording_position_request = recording_position_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        recording_position_request = recording_position_request.header(0).parent().unwrap();
        recording_position_request.control_session_id(control_session_id);
        recording_position_request.correlation_id(correlation_id);
        recording_position_request.recording_id(recording_id);
        offer(recording_position_request.encoded_length())
    }

    pub fn truncate_recording(&mut self, recording_id: i64, position: i64, correlation_id: i64, control_session_id: i64) -> bool {
        let mut truncate_recording_request = TruncateRecordingRequestEncoder::default();
        truncate_recording_request = truncate_recording_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        truncate_recording_request = truncate_recording_request.header(0).parent().unwrap(); // don't use unwrap
        truncate_recording_request.control_session_id(control_session_id);
        truncate_recording_request.correlation_id(correlation_id);
        truncate_recording_request.recording_id(recording_id);
        truncate_recording_request.position(position);
        offer(truncate_recording_request.encoded_length())
    }

    pub fn purge_recording(
        &mut self,
        recording_id: i64,
        correlation_id: i64,
        control_session_id: i64
    ) -> bool {
        let mut purge_recording_request: PurgeRecordingRequestEncoder = PurgeRecordingRequestEncoder::default();
        purge_recording_request = purge_recording_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        purge_recording_request = purge_recording_request.header(0).parent().unwrap(); // don't use unwrap
        purge_recording_request.control_session_id(control_session_id);
        purge_recording_request.correlation_id(correlation_id);
        purge_recording_request.recording_id(recording_id);
        offer(purge_recording_request.encoded_length())
    }

    pub fn get_start_position(&mut self, recording_id: i64, correlation_id: i64, control_session_id: i64) -> bool {
        let mut start_position_request = StartPositionRequestEncoder::default();

        start_position_request = start_position_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH)
        start_position_request = start_position_request.header(0).parent().unwrap();
        start_position_request.control_session_id(control_session_id);
        start_position_request.correlation_id(correlation_id);
        start_position_request.recording_id(recording_id);

        offer(start_position_request.encoded_length())
    }

    pub fn get_stop_position(&mut self, recording_id: i64, correlation_id: i64, control_session_id: i64) -> bool {
        let mut stop_position_request = StopPositionRequestEncoder::default();

        stop_position_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH)
        stop_position_request = stop_position_request.header(0).parent().unwrap()
        stop_position_request.control_session_id(control_session_id);
        stop_position_request.correlation_id(correlation_id);
        stop_position_request.recording_id(recording_id);
        offer(stop_position_request.encoded_length())
    }

    pub fn find_last_matching_recording(
        &mut self,
        min_recording_id: i64,
        channel_fragment: String,
        stream_id: i32,
        session_id: i32,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut find_last_matching_recording_request: FindLastMatchingRecordingRequestEncoder = FindLastMatchingRecordingRequestEncoder::default();
        find_last_matching_recording_request = find_last_matching_recording_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        find_last_matching_recording_request = find_last_matching_recording_request.header(0).parent().unwrap();
        find_last_matching_recording_request.control_session_id(control_session_id);
        find_last_matching_recording_request.correlation_id(correlation_id);
        find_last_matching_recording_request.min_recording_id(min_recording_id);
        find_last_matching_recording_request.session_id(session_id);
        find_last_matching_recording_request.stream_id(stream_id);
        find_last_matching_recording_request.channel(channel_fragment.as_bytes());

        offer(find_last_matching_recording_request.encoded_length(), None)
    }

    pub fn list_recording_subscriptions(
        &mut self,
        pseudo_index: i32,
        subscription_count: i32,
        channel_fragment: String,
        stream_id: i32,
        apply_stream_id: bool,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut list_recording_subscriptions_request = ListRecordingSubscriptionsRequestEncoder::default();
        list_recording_subscriptions_request = list_recording_subscriptions_request.wrap(
            WriteBuf::new(self.buffer.byte_array.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );
        list_recording_subscriptions_request = list_recording_subscriptions_request
            .header(0)
            .parent()
            .unwrap(); // don't use unwrap
        list_recording_subscriptions_request.control_session_id(control_session_id);
        list_recording_subscriptions_request.correlation_id(correlation_id);
        list_recording_subscriptions_request.pseudo_index(pseudo_index);
        list_recording_subscriptions_request.subscription_count(subscription_count);
        list_recording_subscriptions_request.apply_stream_id(
            if apply_stream_id {
                BooleanType::TRUE
            } else {
                BooleanType::FALSE
            },
        );
        list_recording_subscriptions_request.stream_id(stream_id);
        list_recording_subscriptions_request.channel(channel_fragment.as_bytes());
        offer(list_recording_subscriptions_request.encoded_length(), None)
    }

    // LOOK ABOVE LINE 1206
    pub fn stop_replication(
        &mut self,
        replication_id: i64,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut stop_replication_request = StopReplicationRequestEncoder::default();

         stop_replication_request
            .wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH)
            .header(0)
            .parent()
            .unwrap(); // don't use unwrap

        stop_replication_request.control_session_id(control_session_id);
        stop_replication_request.correlation_id(correlation_id);
        stop_replication_request.replication_id(replication_id);

        offer(stop_replication_request.encoded_length())
    }

    pub fn detach_segments(
        &mut self,
        recording_id: i64,
        new_start_position: i64,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut detach_segments_request = DetachSegmentsRequestEncoder::default();
        detach_segments_request = detach_segments_request.wrap(
            WriteBuf::new(self.buffer.byte_array.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );
        detach_segments_request = detach_segments_request.header(0).parent().unwrap(); // don't use unwrap
        detach_segments_request.control_session_id(control_session_id);
        detach_segments_request.correlation_id(correlation_id);
        detach_segments_request.recording_id(recording_id);
        detach_segments_request.new_start_position(new_start_position);
        offer(detach_segments_request.encoded_length())
    }

    pub fn delete_detached_segments(&mut self, recording_id: i64, correlation_id: i64, control_session_id: i64) -> bool {
        let mut delete_detached_segments_request = DeleteDetachedSegmentsRequestEncoder::default();
        delete_detached_segments_request = delete_detached_segments_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        delete_detached_segments_request = delete_detached_segments_request.header(0).parent().unwrap(); // don't use unwrap
        delete_detached_segments_request.control_session_id(control_session_id);
        delete_detached_segments_request.correlation_id(correlation_id);
        delete_detached_segments_request.recording_id(recording_id);
        offer(delete_detached_segments_request.encoded_length())
    }

    pub fn purge_segments(
        &mut self,
        recording_id: i64,
        new_start_position: i64,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut purge_segments_request = PurgeSegmentsRequestEncoder::default();

        purge_segments_request
            .wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH)
            .header(0)
            .parent()
            .unwrap();
        purge_segments_request.control_session_id(control_session_id);
        purge_segments_request.correlation_id(correlation_id);
        purge_segments_request.recording_id(recording_id);
        purge_segments_request.new_start_position(new_start_position);

        offer(purge_segments_request.encoded_length())
    }

    pub fn attach_segments(
        &mut self,
        recording_id: i64,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut attach_segments_request = AttachSegmentsRequestEncoder::default();

        attach_segments_request
            .wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH)
            .header(0)
            .parent()
            .unwrap();

        attach_segments_request
            .control_session_id(control_session_id)
            .correlation_id(correlation_id)
            .recording_id(recording_id);

        offer(attach_segments_request.encoded_length())
    }

    pub fn migrate_segments(
        &mut self,
        src_recording_id: i64,
        dst_recording_id: i64,
        correlation_id: i64,
        control_session_id: i64,
    ) -> bool {
        let mut migrate_segments_request: MigrateSegmentsRequestEncoder = MigrateSegmentsRequestEncoder::default();
        migrate_segments_request = migrate_segments_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        migrate_segments_request = migrate_segments_request.header(0).parent().unwrap(); // don't use unwrap
        migrate_segments_request.control_session_id(control_session_id);
        migrate_segments_request.correlation_id(correlation_id);
        migrate_segments_request.src_recording_id(src_recording_id);
        migrate_segments_request.dst_recording_id(dst_recording_id);

        offer(migrate_segments_request.encoded_length())
    }

    pub fn replay(
        &mut self,
        recording_id: i64,
        position: i64,
        length: i64,
        replay_channel: String,
        replay_stream_id: i32,
        correlation_id: i64,
        control_session_id: i64,
        file_io_max_length: i32,
    ) -> bool {
        let mut replay_request = ReplayRequestEncoder::default();
        replay_request = replay_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        replay_request = replay_request.header(0).parent().unwrap(); // don't use unwrap
        replay_request.correlation_id(correlation_id);
        replay_request.control_session_id(control_session_id);
        replay_request.recording_id(recording_id);
        replay_request.position(position);
        replay_request.length(length);
        replay_request.replay_stream_id(replay_stream_id);
        replay_request.file_io_max_length(file_io_max_length);
        replay_request.replay_channel(replay_channel.as_bytes());
        offer(replay_request.encoded_length(), &self.retry_idle_strategy, self.retry_attempts, &self.publication)
    }

    fn bounded_replay(
        &mut self,
        recording_id: i64,
        position: i64,
        length: i64,
        limit_counter_id: i32,
        replay_channel: String,
        replay_stream_id: i32,
        correlation_id: i64,
        control_session_id: i64,
        file_io_max_length: i32,
    ) -> bool {
        let mut bounded_replay_request = BoundedReplayRequestEncoder::default();

        bounded_replay_request
            .wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH)
            .header(0)
            .unwrap();
        bounded_replay_request.control_session_id(control_session_id);
        bounded_replay_request.correlation_id(correlation_id);
        bounded_replay_request.recording_id(recording_id);
        bounded_replay_request.position(position);
        bounded_replay_request.length(length);
        bounded_replay_request.limit_counter_id(limit_counter_id);
        bounded_replay_request.replay_stream_id(replay_stream_id);
        bounded_replay_request.file_io_max_length(file_io_max_length);
        bounded_replay_request.replay_channel(replay_channel.as_bytes());

        self.offer(bounded_replay_request.encoded_length())
    }

    fn replicate(
        &mut self,
        src_recording_id: i64,
        dst_recording_id: i64,
        stop_position: i64,
        channel_tag_id: i64,
        subscription_tag_id: i64,
        src_control_stream_id: i32,
        src_control_channel: String,
        live_destination: String,
        replication_channel: String,
        correlation_id: i64,
        control_session_id: i64,
        file_io_max_length: i32,
    ) -> bool {
        let mut replicate_request = ReplicateRequest2Encoder::default();
        replicate_request = replicate_request.wrap(WriteBuf::new(self.buffer.byte_array.as_mut_slice()), message_header_codec::ENCODED_LENGTH);
        replicate_request = replicate_request.header(0).parent().unwrap(); // don't use unwrap
        replicate_request.control_session_id(control_session_id);
        replicate_request.correlation_id(correlation_id);
        replicate_request.src_recording_id(src_recording_id);
        replicate_request.dst_recording_id(dst_recording_id);
        replicate_request.stop_position(stop_position);
        replicate_request.channel_tag_id(channel_tag_id);
        replicate_request.subscription_tag_id(subscription_tag_id);
        replicate_request.src_control_stream_id(src_control_stream_id);
        replicate_request.file_io_max_length(file_io_max_length);
        replicate_request.src_control_channel(src_control_channel.as_bytes());
        replicate_request.live_destination(live_destination.as_bytes());
        replicate_request.replication_channel(replication_channel.as_bytes());
        offer(replicate_request.encoded_length(), null)
    }


}

