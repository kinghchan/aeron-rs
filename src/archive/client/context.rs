use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::time::Duration;
use crate::aeron::Aeron;
use crate::concurrent::logbuffer::term_reader::ErrorHandler;
use crate::concurrent::strategies::YieldingIdleStrategy;

// IC: Context is not Cloneable because it may contain an Aeron instance.
// #[derive(Clone)]
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
    idle_strategy: YieldingIdleStrategy, // IC: Java uses idle strategy interface, contains idle(int workCount) method
    lock: Arc<Mutex<()>>,
    aeron_directory_name: String,
    // IC: does not make sense to make Aeron cloneable. A new Aeron instance should only be created by connecting to the Media Driver.
    aeron: Option<Aeron>,
    error_handler: Box<dyn ErrorHandler>,
    // IC: commenting out below for now
    // credentials_supplier: Box<dyn CredentialsSupplier>,
    // recording_signal_consumer: Box<dyn RecordingSignalConsumer>,
    // agent_invoker: AgentInvoker, // only 1 writer, the setter, and latter has no usages
    owns_aeron_client: bool,
}
//
// impl Context {
//     // ...
//     pub fn conclude(&mut self) {
//         if 0 != self.is_concluded.fetch_add(1, Ordering::SeqCst) {
//             panic!("ConcurrentConcludeException");
//         }
//
//         if self.control_request_channel.is_none() {
//             panic!("AeronArchive.Context.controlRequestChannel must be set");
//         }
//
//         if self.control_response_channel.is_none() {
//             panic!("AeronArchive.Context.controlResponseChannel must be set");
//         }
//
//         if self.aeron.is_none() {
//             if let aeron = Aeron::connect_ctx(
//                 AeronContext::new()
//                     .aeron_directory_name(self.aeron_directory_name.clone())
//                     .error_handler(self.error_handler.clone()),
//             ) {
//                 self.aeron = ;
//             }
//             self.owns_aeron_client = true;
//         }
//
//         if self.idle_strategy.is_none() {
//             self.idle_strategy = Some(BackoffIdleStrategy::new(
//                 IDLE_MAX_SPINS, IDLE_MAX_YIELDS, IDLE_MIN_PARK_NS, IDLE_MAX_PARK_NS,
//             ));
//         }
//
//         if self.credentials_supplier.is_none() {
//             self.credentials_supplier = Some(NullCredentialsSupplier::new());
//         }
//
//         if self.lock.is_none() {
//             self.lock = Some(Arc::new(Mutex::new(())));
//         }
//
//         self.control_request_channel = apply_default_params(self.control_request_channel.clone());
//         self.control_response_channel = apply_default_params(self.control_response_channel.clone());
//     }
//     // ...
// }