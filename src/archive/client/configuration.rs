pub const PROTOCOL_MAJOR_VERSION: i32 = 1;
pub const PROTOCOL_MINOR_VERSION: i32 = 10;
pub const PROTOCOL_PATCH_VERSION: i32 = 0;
// IC: Java does some bit shifting here...
pub const PROTOCOL_SEMANTIC_VERSION: i32 = 0;
// SemanticVersion::compose(
// Self::PROTOCOL_MAJOR_VERSION,
// Self::PROTOCOL_MINOR_VERSION,
// Self::PROTOCOL_PATCH_VERSION,
// );
pub const MESSAGE_TIMEOUT_PROP_NAME: &str = "aeron.archive.message.timeout";
pub const MESSAGE_TIMEOUT_DEFAULT_NS: i64 = 10_000_000_000;
pub const CONTROL_CHANNEL_PROP_NAME: &str = "aeron.archive.control.channel";
pub const CONTROL_CHANNEL_DEFAULT: &str = "aeron:udp?endpoint=localhost:8010";
pub const CONTROL_STREAM_ID_PROP_NAME: &str = "aeron.archive.control.stream.id";
pub const CONTROL_STREAM_ID_DEFAULT: i32 = 10;
pub const LOCAL_CONTROL_CHANNEL_PROP_NAME: &str = "aeron.archive.local.control.channel";
pub const LOCAL_CONTROL_CHANNEL_DEFAULT: &str = "aeron:ipc"; // IC: Java uses CommonContext::IPC_CHANNEL;
pub const LOCAL_CONTROL_STREAM_ID_PROP_NAME: &str = "aeron.archive.local.control.stream.id";
pub const LOCAL_CONTROL_STREAM_ID_DEFAULT: i32 = CONTROL_STREAM_ID_DEFAULT;
pub const CONTROL_RESPONSE_CHANNEL_PROP_NAME: &str = "aeron.archive.control.response.channel";
pub const CONTROL_RESPONSE_CHANNEL_DEFAULT: &str = "aeron:udp?endpoint=localhost:0";
pub const CONTROL_RESPONSE_STREAM_ID_PROP_NAME: &str = "aeron.archive.control.response.stream.id";
pub const CONTROL_RESPONSE_STREAM_ID_DEFAULT: i32 = 20;
pub const RECORDING_EVENTS_CHANNEL_PROP_NAME: &str = "aeron.archive.recording.events.channel";
pub const RECORDING_EVENTS_CHANNEL_DEFAULT: &str = "aeron:udp?control-mode=dynamic|control=localhost:8030";
pub const RECORDING_EVENTS_STREAM_ID_PROP_NAME: &str = "aeron.archive.recording.events.stream.id";
pub const RECORDING_EVENTS_STREAM_ID_DEFAULT: i32 = 30;
pub const RECORDING_EVENTS_ENABLED_PROP_NAME: &str = "aeron.archive.recording.events.enabled";
pub const RECORDING_EVENTS_ENABLED_DEFAULT: bool = true;
pub const CONTROL_TERM_BUFFER_SPARSE_PROP_NAME: &str = "aeron.archive.control.term.buffer.sparse";
pub const CONTROL_TERM_BUFFER_SPARSE_DEFAULT: bool = true;
pub const CONTROL_TERM_BUFFER_LENGTH_PROP_NAME: &str = "aeron.archive.control.term.buffer.length";
pub const CONTROL_TERM_BUFFER_LENGTH_DEFAULT: i32 = 64 * 1024;
pub const CONTROL_MTU_LENGTH_PROP_NAME: &str = "aeron.archive.control.mtu.length";
pub const CONTROL_MTU_LENGTH_DEFAULT: i32 = 1408; // IC: Java uses aeron.driver.Configuration.MTU_LENGTH_DEFAULT

// default configuration for Aeron Archive
pub struct Configuration {}