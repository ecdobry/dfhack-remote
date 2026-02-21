#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::{fmt::Display, ops::Deref};

/// Raw protobuf messages
#[allow(missing_docs)]
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages/includes.rs"));
    include!(concat!(env!("OUT_DIR"), "/reexports.rs"));
}

/// Stubs exposing the feature of the DFHack remote API.
#[allow(clippy::let_unit_value)]
#[allow(missing_docs)]
pub mod stubs {
    include!(concat!(env!("OUT_DIR"), "/stubs/mod.rs"));
}

/// Message exchanged by dfhack-remote
pub trait Message: prost::Message + prost::Name + Default {}
impl<T: prost::Message + prost::Name + Default> Message for T {}

/// The `Channel` is the low-level exchange implementation.
///
/// It is in charge to serialize/deserialize messages, and exchange
/// them with Dwarf Fortress. It is not meant to be used as is, but to be passed to
/// It is analoguous to the gRPC channel.
pub trait Channel {
    /// Type of the errors raised by the stub.
    ///
    /// Defined by the channel implementation.
    type TError;

    /// Send a request to DFHack, and return its reply.
    ///
    /// # Errors
    ///
    /// The error type is defined by the channel implementation
    ///
    /// # Arguments
    ///
    /// * `plugin` - Name of the plugin implementing the request. Example: "RemoteFortressReader". Empty for core messages.
    /// * `name` - Name of the method. Example: "GetDFVersion"
    /// * `request` - Input of the method.
    ///
    /// # Returns
    ///
    /// A protobuf result type.
    ///
    fn request<TRequest: Message, TReply: Message>(
        &mut self,
        plugin: &'static str,
        name: &'static str,
        request: TRequest,
    ) -> Result<Reply<TReply>, Self::TError>;
}

/// Reply to a request, it contains the actual reply value, and additional
/// text fragments.
pub struct Reply<T> {
    /// The actual reply value
    pub reply: T,
    /// Additional text fragments received during the rpc
    pub fragments: Vec<messages::CoreTextFragment>,
}

impl<T> Deref for Reply<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.reply
    }
}

impl<T: Display> Display for Reply<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reply)
    }
}

/// Reflection for runtime inspection of the stubs.
pub mod reflection {
    /// Descriptor of a remote procedure call
    ///
    /// These are all the needed information to make a call
    pub struct RemoteProcedureDescriptor {
        /// Name of the RPC
        pub name: &'static str,

        /// Plugin implementing the RPC
        ///
        /// An empty string means the core API
        pub plugin_name: &'static str,

        /// Input type
        ///
        /// This is the full name of the protobuf message
        pub input_type: String,

        /// Output type
        ///
        /// This is the full name of the protobuf message
        pub output_type: String,
    }

    /// Ability for a stub to list its supported methods
    ///
    /// This is mostly useful for testing purpose.
    pub trait StubReflection {
        /// List the supported remote calls
        fn list_methods() -> Vec<RemoteProcedureDescriptor>;
    }
}
