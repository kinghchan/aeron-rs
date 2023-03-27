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

pub const DEFAULT_RETRY_ATTEMPTS: i32 = 3;

pub struct ArchiveProxy<'a> {
    connect_timeout_ns: i64,
    retry_attempts: i32,
    // IC: Yielding only for now
    retry_idle_strategy: YieldingIdleStrategy,
    // Box<dyn IdleStrategy>,
    nano_clock: NanoClock,
    // credentials_supplier: Box<dyn CredentialsSupplier>,
    // IC: ExpandableArrayBuffer just a container for a Vec<u8>
    buffer: ExpandableArrayBuffer,
    publication: Publication,
    // IC: Not sure if this works...
    message_header: MessageHeaderEncoder<WriteBuf<'a>>,
    // Add the rest of the encoders for each message type here
    start_recording_request: Option<StartRecordingRequestEncoder<'a>>,
    start_recording_request2: Option<StartRecordingRequest2Encoder<'a>>,
    stop_recording_request: Option<StopRecordingRequestEncoder<'a>>,
    stop_recording_subscription_request: Option<StopRecordingSubscriptionRequestEncoder<'a>>,
    stop_recording_by_identity_request: Option<StopRecordingByIdentityRequestEncoder<'a>>,
    replay_request: Option<ReplayRequestEncoder<'a>>,
    stop_replay_request: Option<StopReplayRequestEncoder<'a>>,
    list_recordings_request: Option<ListRecordingsRequestEncoder<'a>>,
    list_recordings_for_uri_request: Option<ListRecordingsForUriRequestEncoder<'a>>,
    list_recording_request: Option<ListRecordingRequestEncoder<'a>>,
    extend_recording_request: Option<ExtendRecordingRequestEncoder<'a>>,
    extend_recording_request2: Option<ExtendRecordingRequest2Encoder<'a>>,
    recording_position_request: Option<RecordingPositionRequestEncoder<'a>>,
    truncate_recording_request: Option<TruncateRecordingRequestEncoder<'a>>,
    purge_recording_request: Option<PurgeRecordingRequestEncoder<'a>>,
    stop_position_request: Option<StopPositionRequestEncoder<'a>>,
    find_last_matching_recording_request: Option<FindLastMatchingRecordingRequestEncoder<'a>>,
    list_recording_subscriptions_request: Option<ListRecordingSubscriptionsRequestEncoder<'a>>,
    bounded_replay_request: Option<BoundedReplayRequestEncoder<'a>>,
    stop_all_replays_request: Option<StopAllReplaysRequestEncoder<'a>>,
    replicate_request: Option<ReplicateRequest2Encoder<'a>>,
    stop_replication_request: Option<StopReplicationRequestEncoder<'a>>,
    start_position_request: Option<StartPositionRequestEncoder<'a>>,
    detach_segments_request: Option<DetachSegmentsRequestEncoder<'a>>,
    delete_detached_segments_request: Option<DeleteDetachedSegmentsRequestEncoder<'a>>,
    purge_segments_request: Option<PurgeSegmentsRequestEncoder<'a>>,
    attach_segments_request: Option<AttachSegmentsRequestEncoder<'a>>,
    migrate_segments_request: Option<MigrateSegmentsRequestEncoder<'a>>,
}

impl ArchiveProxy<'_> {
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
        let mut buffer: Vec<u8> = Vec::with_capacity(256);

        Self {
            connect_timeout_ns: MESSAGE_TIMEOUT_DEFAULT_NS,
            retry_attempts: DEFAULT_RETRY_ATTEMPTS,
            retry_idle_strategy: YieldingIdleStrategy {},
            nano_clock: system_nano_clock::NanoClock,
            // credentials_supplier: Box::new(NullCredentialsSupplier::default()),
            buffer: buffer,
            publication,
            message_header: MessageHeaderEncoder::default(),
            start_recording_request: None,
            start_recording_request2: None,
            stop_recording_request: None,
            stop_recording_subscription_request: None,
            stop_recording_by_identity_request: None,
            replay_request: None,
            stop_replay_request: None,
            list_recordings_request: None,
            list_recordings_for_uri_request: None,
            list_recording_request: None,
            extend_recording_request: None,
            extend_recording_request2: None,
            recording_position_request: None,
            truncate_recording_request: None,
            purge_recording_request: None,
            stop_position_request: None,
            find_last_matching_recording_request: None,
            list_recording_subscriptions_request: None,
            bounded_replay_request: None,
            stop_all_replays_request: None,
            replicate_request: None,
            stop_replication_request: None,
            start_position_request: None,
            detach_segments_request: None,
            delete_detached_segments_request: None,
            purge_segments_request: None,
            attach_segments_request: None,
            migrate_segments_request: None,
        }
    }
}