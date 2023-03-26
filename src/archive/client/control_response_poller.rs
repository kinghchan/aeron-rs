use crate::aeron::Aeron;
use crate::context::Context;
use crate::subscription::Subscription;
use crate::context::NULL_VALUE;


pub struct ControlResponsePoller {
    pub fragment_limit: i32,
    subscription: Subscription,
    fragment_assembler: ControlledFragmentAssembler,
    message_header_decoder: MessageHeaderDecoder,
    control_response_decoder: ControlResponseDecoder,
    challenge_decoder: ChallengeDecoder,
    recording_signal_event_decoder: RecordingSignalEventDecoder,
    control_session_id: i64,
    correlation_id: i64,
    relevant_id: i64,
    template_id: i32,
    version: i32,
    code: ControlResponseCode,
    error_message: Option<String>,
    recording_id: i64,
    subscription_id: i64,
    position: i64,
    recording_signal: Option<RecordingSignal>,
    encoded_challenge: Option<Vec<u8>>,
    is_poll_complete: bool,
}

impl ControlResponsePoller {
    /**
     * Create a poller for a given subscription to an archive for control response messages with a default
     * fragment limit for polling as {@link #FRAGMENT_LIMIT}.
     *
     * @param subscription to poll for new events.
     */
    pub fn new(subscription: Subscription) -> Self {
        Self::with_fragment_limit(subscription, 10)
    }

    /**
     * Create a poller for a given subscription to an archive for control response messages.
     *
     * @param subscription  to poll for new events.
     * @param fragment_limit to apply when polling.
     */
    pub fn with_fragment_limit(subscription: Subscription, fragment_limit: i32) -> Self {
        Self {
            fragment_limit,
            subscription,
            fragment_assembler: ControlledFragmentAssembler::new(Self::on_fragment),
            message_header_decoder: MessageHeaderDecoder::new(),
            control_response_decoder: ControlResponseDecoder::new(),
            challenge_decoder: ChallengeDecoder::new(),
            recording_signal_event_decoder: RecordingSignalEventDecoder::new(),
            control_session_id: NULL_VALUE,
            correlation_id: NULL_VALUE,
            relevant_id: NULL_VALUE,
            template_id: NULL_VALUE as i32,
            version: 0,
            code: NULL_VALUE,
            error_message: None,
            recording_id: NULL_VALUE,
            subscription_id: NULL_VALUE,
            position: NULL_VALUE,
            recording_signal: None,
            encoded_challenge: None,
            is_poll_complete: false,
        }
    }

    /**
     * Get the {@link Subscription} used for polling responses.
     *
     * @return the {@link Subscription} used for polling responses.
     */
    pub fn subscription(&self) -> &Subscription {
        &self.subscription
    }

    pub fn poll(&mut self) -> i32 {
        if self.is_poll_complete {
            self.is_poll_complete = false;
            self.template_id = NULL_VALUE as i32;
            self.control_session_id = NULL_VALUE;
            self.correlation_id = NULL_VALUE;
            self.relevant_id = NULL_VALUE;
            self.recording_id = NULL_VALUE;
            self.subscription_id = NULL_VALUE;
            self.position = NULL_VALUE;
            self.recording_signal = None;
            self.version = 0;
            self.error_message = None;
            self.encoded_challenge = None;
        }

        self.subscription.controlled_poll(&mut self.fragment_assembler, self.fragment_limit)
    }
}

