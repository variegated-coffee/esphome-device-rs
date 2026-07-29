//! Error type for esphome-device.
//!
//! The crate used to lean on `anyhow` throughout. `anyhow` is std-only, which
//! made the library impossible to build for a bare-metal consumer even with
//! `default-features = false`, so the shared and embassy paths use this
//! enumeration instead. `anyhow` now only appears in the build script.

use core::fmt;

/// Everything that can go wrong talking the ESPHome API.
///
/// Deliberately a plain enum with no payload beyond small copies: it has to be
/// constructible in a `no_std` context and cheap to pass around on a
/// microcontroller.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EspHomeError {
    /// The peer closed the connection.
    ConnectionClosed,
    /// The transport failed while reading.
    ConnectionReadError,
    /// The transport failed while writing.
    ConnectionWriteError,
    /// A varint ran past the 64-bit limit.
    VarintTooLong,
    /// A frame did not start with the expected zero byte.
    ExpectedZeroByte(u8),
    /// The frame's type id is not one we know.
    UnknownMessageType(u8),
    /// A message could not be decoded from the wire.
    DecodeError,
    /// A message could not be encoded for the wire.
    EncodeError,
    /// A request arrived before the client authenticated.
    NotAuthenticated,
    /// A request arrived before the hello/connect handshake finished.
    ConnectionNotSetup,
    /// The message type is valid but this server does not handle it.
    UnsupportedMessageType,
    /// The connection's write half has already been taken.
    WriterNotAvailable,
    /// The connection's read half has already been taken.
    ReaderNotAvailable,
    /// A channel the server reads from was closed.
    ChannelRecvError,
    /// A channel the server writes to was closed.
    ChannelSendError,
}

impl fmt::Display for EspHomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionClosed => write!(f, "connection closed"),
            Self::ConnectionReadError => write!(f, "connection read error"),
            Self::ConnectionWriteError => write!(f, "connection write error"),
            Self::VarintTooLong => write!(f, "varint too long"),
            Self::ExpectedZeroByte(got) => write!(f, "expected zero byte, got {got}"),
            Self::UnknownMessageType(id) => write!(f, "unknown message type: {id}"),
            Self::DecodeError => write!(f, "message decode error"),
            Self::EncodeError => write!(f, "message encode error"),
            Self::NotAuthenticated => write!(f, "not authenticated"),
            Self::ConnectionNotSetup => write!(f, "connection not set up"),
            Self::UnsupportedMessageType => write!(f, "unsupported message type"),
            Self::WriterNotAvailable => write!(f, "writer not available"),
            Self::ReaderNotAvailable => write!(f, "reader not available"),
            Self::ChannelRecvError => write!(f, "channel receive error"),
            Self::ChannelSendError => write!(f, "channel send error"),
        }
    }
}

impl core::error::Error for EspHomeError {}

// The std backend propagates async-std channel failures with `?`. Both only
// ever mean "the other end is gone", so they collapse onto one variant each.
#[cfg(feature = "std")]
impl From<async_std::channel::RecvError> for EspHomeError {
    fn from(_: async_std::channel::RecvError) -> Self {
        Self::ChannelRecvError
    }
}

#[cfg(feature = "std")]
impl<T> From<async_std::channel::SendError<T>> for EspHomeError {
    fn from(_: async_std::channel::SendError<T>) -> Self {
        Self::ChannelSendError
    }
}

/// Convenience alias used throughout the crate.
pub type Result<T> = core::result::Result<T, EspHomeError>;
