use crate::subscription::Subscription;

pub struct RecordingDescriptorPoller {
    message_header_decoder: MessageHeaderDecoder,
    control_response_decoder: ControlResponseDecoder,
    recording_descriptor_decoder: RecordingDescriptorDecoder,
    recording_signal_event_decoder: RecordingSignalEventDecoder,

    control_session_id: i64,
    fragment_limit: i32,
    subscription: Subscription,
    fragment_assembler: ControlledFragmentAssembler,
    error_handler: Arc<dyn ErrorHandler>,
    recording_signal_consumer: Arc<dyn RecordingSignalConsumer>,

    correlation_id: i64,
    remaining_record_count: i32,
    is_dispatch_complete: bool,
    recording_descriptor_consumer: Option<Box<dyn RecordingDescriptorConsumer>>,
}


impl RecordingDescriptorPoller {
    pub fn new(
        subscription: Subscription,
        error_handler: Arc<dyn ErrorHandler>,
        control_session_id: i64,
        fragment_limit: i32,
        recording_signal_consumer: Option<Arc<dyn RecordingSignalConsumer>>,
    ) -> Self {
        let recording_signal_consumer = recording_signal_consumer.unwrap_or_else(|| {
            Arc::new(NoOpRecordingSignalConsumer::default()) as Arc<dyn RecordingSignalConsumer>
        });

        Self {
            message_header_decoder: MessageHeaderDecoder::default(),
            control_response_decoder: ControlResponseDecoder::default(),
            recording_descriptor_decoder: RecordingDescriptorDecoder::default(),
            recording_signal_event_decoder: RecordingSignalEventDecoder::default(),

            control_session_id,
            fragment_limit,
            subscription,
            fragment_assembler: ControlledFragmentAssembler::default(),
            error_handler,
            recording_signal_consumer,

            correlation_id: 0,
            remaining_record_count: 0,
            is_dispatch_complete: false,
            recording_descriptor_consumer: None,
        }
    }

    pub fn subscription(&self) -> &Subscription {
        &self.subscription
    }

    pub fn poll(&mut self) -> Result<i32, AeronError> {
        if self.is_dispatch_complete {
            return Ok(0);
        }

        let fragments_read = self
            .subscription
            .controlled_poll(&mut self.fragment_assembler, self.fragment_limit)?;

        if self.is_dispatch_complete {
            return Ok(0);
        }

        Ok(fragments_read)
    }

    pub fn control_session_id(&self) -> i64 {
        self.control_session_id
    }

    pub fn is_dispatch_complete(&self) -> bool {
        self.is_dispatch_complete
    }

    pub fn remaining_record_count(&self) -> i32 {
        self.remaining_record_count
    }

    pub fn reset(
        &mut self,
        correlation_id: i64,
        record_count: i32,
        consumer: Box<dyn RecordingDescriptorConsumer>,
    ) {
        self.correlation_id = correlation_id;
        self.recording_descriptor_consumer = Some(consumer);
        self.remaining_record_count = record_count;
        self.is_dispatch_complete = false;
    }
}