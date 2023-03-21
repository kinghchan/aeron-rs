// use std::time::Duration;
// use std::sync::Arc;
// use crate::agrona::concurrent::system_nano_clock;
// use crate::archive::client::aeron_archive::AeronArchive;
// use crate::concurrent::strategies::YieldingIdleStrategy;
// use crate::publication::Publication;
//
// pub const DEFAULT_RETRY_ATTEMPTS: i32 = 3;
//
// pub struct ArchiveProxy {
//     connect_timeout_ns: Duration,
//     retry_attempts: i32,
//     retry_idle_strategy: Box<dyn IdleStrategy>,
//     nano_clock: NanoClock,
//     // credentials_supplier: Box<dyn CredentialsSupplier>,
//     buffer: ExpandableArrayBuffer,
//     publication: Publication,
//     message_header: MessageHeaderEncoder,
//     // Add the rest of the encoders for each message type here
// }
//
// impl ArchiveProxy {
//     pub fn new(publication: Publication) -> Self {
//         Self {
//             connect_timeout_ns: Duration::from_nanos(AeronArchive::MESSAGE_TIMEOUT_DEFAULT_NS),
//             retry_attempts: Self::DEFAULT_RETRY_ATTEMPTS,
//             retry_idle_strategy: Box::new(YieldingIdleStrategy::default()),
//             nano_clock: system_nano_clock::default(),
//             // credentials_supplier: Box::new(NullCredentialsSupplier::default()),
//             buffer: ExpandableArrayBuffer::new(256),
//             publication,
//             message_header: MessageHeaderEncoder::default(),
//             // Initialize the rest of the encoders for each message type here
//         }
//     }
//
//     pub fn with_configuration(
//         publication: Publication,
//         retry_idle_strategy: Box<dyn IdleStrategy>,
//         nano_clock: NanoClock,
//         connect_timeout_ns: Duration,
//         retry_attempts: i32,
//         credentials_supplier: Box<dyn CredentialsSupplier>,
//     ) -> Self {
//         Self {
//             connect_timeout_ns,
//             retry_attempts,
//             retry_idle_strategy,
//             nano_clock,
//             credentials_supplier,
//             buffer: ExpandableArrayBuffer::new(256),
//             publication,
//             message_header: MessageHeaderEncoder::default(),
//             // Initialize the rest of the encoders for each message type here
//         }
//     }
// }
