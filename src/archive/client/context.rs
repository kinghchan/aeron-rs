use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct Context {
    is_concluded: Arc<Mutex<bool>>,
    message_timeout_ns: Duration,
    recording_events_channel: String,
    recording_events_stream_id: i32,
    control_request_channel: String,
    control_request_stream_id: i32,
    control_response_channel: String,
    control_response_stream_id: i32,
    control_term_buffer_sparse: bool,
    control_term_buffer_length: i32,
    control_mtu_length: i32,
    idle_strategy: Box<dyn IdleStrategy>,
    lock: Arc<Mutex<()>>,
    aeron_directory_name: String,
    aeron: Aeron,
    error_handler: Box<dyn ErrorHandler>,
    credentials_supplier: Box<dyn CredentialsSupplier>,
    recording_signal_consumer: Box<dyn RecordingSignalConsumer>,
    agent_invoker: AgentInvoker,
    owns_aeron_client: bool,
}
