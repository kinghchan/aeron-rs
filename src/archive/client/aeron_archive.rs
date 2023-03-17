use std::sync::{Arc, Mutex};
use crate::context::NULL_VALUE;
use crate::archive::client::context::Context;
use crate::aeron::Aeron;


const NULL_TIMESTAMP: i64 = NULL_VALUE;
const NULL_POSITION: i64 = NULL_VALUE;
const NULL_LENGTH: i64 = NULL_VALUE;
const NOT_CONNECTED_MSG: &str = "not connected";
const FRAGMENT_LIMIT: i32 = 10;

pub struct AeronArchive {
    is_closed: bool,
    is_in_callback: bool,
    last_correlation_id: i64,
    control_session_id: i64,
    message_timeout_ns: i64,
    context: Context,
    aeron: Arc<Mutex<Aeron>>,
    archive_proxy: ArchiveProxy,
    idle_strategy: Box<dyn IdleStrategy>,
    control_response_poller: ControlResponsePoller,
    lock: Mutex<()>,
    nano_clock: NanoClock,
    aeron_client_invoker: AgentInvoker,
    agent_invoker: AgentInvoker,
    recording_descriptor_poller: RecordingDescriptorPoller,
    recording_subscription_descriptor_poller: RecordingSubscriptionDescriptorPoller,
}

impl AeronArchive {
    pub fn new(
        control_session_id: i64,
        message_timeout_ns: i64,
        context: Context,
        aeron: Arc<Mutex<Aeron>>,
        archive_proxy: ArchiveProxy,
        idle_strategy: Box<dyn IdleStrategy>,
        control_response_poller: ControlResponsePoller,
        nano_clock: NanoClock,
        aeron_client_invoker: AgentInvoker,
        agent_invoker: AgentInvoker,
        recording_descriptor_poller: RecordingDescriptorPoller,
        recording_subscription_descriptor_poller: RecordingSubscriptionDescriptorPoller,
    ) -> Self {
        Self {
            is_closed: false,
            is_in_callback: false,
            last_correlation_id: NULL_VALUE,
            control_session_id,
            message_timeout_ns,
            context,
            aeron,
            archive_proxy,
            idle_strategy,
            control_response_poller,
            lock: Mutex::new(()),
            nano_clock,
            aeron_client_invoker,
            agent_invoker,
            recording_descriptor_poller,
            recording_subscription_descriptor_poller,
        }
    }


    fn segment_file_base_position(
        start_position: i64,
        position: i64,
        term_buffer_length: i32,
        segment_file_length: i32,
    ) -> i64 {
        let start_term_base_position = start_position - (start_position & (term_buffer_length - 1) as i64);
        let length_from_base_position = position - start_term_base_position;
        let segments = length_from_base_position - (length_from_base_position & (segment_file_length - 1) as i64);

        start_term_base_position + segments
    }

    /**
     * Notify the archive that this control session is closed, so it can promptly release resources then close the
     * local resources associated with the client.
     */
    pub fn close(&mut self) {
        let _lock = self.lock.lock().unwrap();

        if !self.is_closed {
            self.is_closed = true;
            let error_handler = self.context.error_handler();
            let mut result_ex: Option<Box<dyn std::error::Error>> = None;

            if self.archive_proxy.publication().is_connected() {
                result_ex = quiet_close(
                    result_ex,
                    || self.archive_proxy.close_session(self.control_session_id),
                );
            }

            if !self.context.owns_aeron_client() {
                result_ex = quiet_close(result_ex, self.archive_proxy.publication());
                result_ex = quiet_close(result_ex, self.control_response_poller.subscription());
            }

            let mut rethrow = false;
            match self.context.close() {
                Ok(_) => {}
                Err(ex) => {
                    rethrow = true;
                    if let Some(ref mut err) = result_ex {
                        err.add_source(ex);
                    } else {
                        result_ex = Some(Box::new(ex));
                    }
                }
            }

            if let Some(ex) = result_ex {
                if let Some(handler) = error_handler {
                    handler.on_error(&ex);
                }

                if rethrow {
                    // bookmark
                    println!("Bookmark: rethrow")
                    // LangUtil::rethrow_unchecked(&ex);
                }
            }
        }
    }
}