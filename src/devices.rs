use std::time::Duration;
extern crate chrono;

///
/// Nodes
///

pub type NodeId = String;

/// Metadata on a node. A node is a device or collection of devices to
/// services may be connected, as well as other nodes. The FoxBox is
/// the root node. Simple devices that can do a single thing (e.g. a
/// button) are services, while complex devices containing several
/// sensors or effectors are also nodes, in which each sensor and each
/// effector is an service.
#[derive(Debug, Clone)]
pub struct Node {
    /// Tags describing the node.
    ///
    /// These tags can be set by the user, adapters or
    /// applications. They are used by applications.
    ///
    /// For instance "entrance".
    tags: Vec<String>,

    /// An id unique to this node.
    id: NodeId,

    /// If this node has a parent, the id of the parent.
    parent: Option<NodeId>,

    /// Nodes depending on this node.
    subnodes: Vec<Node>,

    /// Services connected directly to this node.
    inputs: Vec<Service<Input>>,
    outputs: Vec<Service<Output>>,
}

impl Node {
    /// Tags describing the node.
    ///
    /// These tags can be set by the user, adapters or
    /// applications. They are used by applications.
    ///
    /// For instance "entrance".
    pub fn get_tags<'a>(&'a self) -> &'a Vec<String> {
        &self.tags
    }

    /// An id unique to this node.
    pub fn get_id<'a>(&'a self) -> &'a NodeId {
        &self.id
    }

    /// If this node has a parent, the id of the parent.
    pub fn get_parent<'a>(&'a self) -> &'a Option<NodeId> {
        &self.parent
    }

    /// Nodes depending on this node.
    pub fn get_subnodes<'a>(&'a self) -> &'a Vec<Node> {
        &self.subnodes
    }

    /// Input services connected directly to this node.
    pub fn get_inputs<'a>(&'a self) -> &'a Vec<Service<Input>> {
        &self.inputs
    }

    /// Output services connected directly to this node.
    pub fn get_outputs<'a>(&'a self) -> &'a Vec<Service<Output>> {
        &self.outputs
    }
}

///
/// Services
///

pub type ServiceId = String;

/// The kind of the service, i.e. a strongly-typed description of
/// _what_ the service can do. Used both for locating services
/// (e.g. "I need a clock" or "I need something that can provide
/// pictures") and for determining the data structure that these
/// services can provide or consume.
///
/// A number of service kinds are standardized, and provided as a set
/// of strongly-typed enum constructors. It is clear, however, that
/// many devices will offer services that cannot be described by
/// pre-existing constructors. For this purpose, this enumeration
/// offers a constructor `Extension`, designed to describe novel
/// services.
#[derive(Debug, Clone)]
pub enum ServiceKind {
    ///
    /// # No payload
    ///

    /// The service is ready. Used for instance once a countdown has
    /// reached completion.
    Ready,

    ///
    /// # Boolean
    ///

    /// The service is used to detect or decide whether some device
    /// is on or off.
    OnOff,

    /// The service is used to detect or decide whether some device
    /// is open or closed.
    OpenClosed,

    ///
    /// # Time
    ///

    /// The service is used to read or set the current absolute time.
    /// Used for instance to wait until a specific time and day before
    /// triggering an action, or to set the appropriate time on a new
    /// device.
    CurrentTime,

    /// The service is used to read or set the current time of day.
    /// Used for instance to trigger an action at a specific hour
    /// every day.
    CurrentTimeOfDay,

    /// The service is part of a countdown. This is the time
    /// remaining until the countdown is elapsed.
    RemainingTime,

    ///
    /// # Temperature
    ///

    Thermostat,
    ActualTemperature,

    /// TODO: Add more

    /// An operation of a kind that has not been standardized yet.
    Extension {
        /// The vendor. Used for namespacing purposes, to avoid
        /// confusing two incompatible extensions with similar
        /// names. For instance, "foxlink@mozilla.com".
        vendor: String,

        /// Identification of the adapter introducing this operation.
        /// Designed to aid with tracing and debugging.
        adapter: String,

        /// A string describing the nature of the value, designed to
        /// let applications discover the devices.
        ///
        /// Examples: `"GroundHumidity"`.
        kind: String,

        /// The data type of the value.
        typ: Type
    }
}

impl ServiceKind {
    /// Get the type of values used to communicate with this service.
    pub fn get_type(&self) -> Type {
        use self::ServiceKind::*;
        use self::Type::*;
        match *self {
            Ready => Unit,
            OnOff | OpenClosed => Bool,
            CurrentTime => TimeStamp,
            CurrentTimeOfDay | RemainingTime => Duration,
            Thermostat | ActualTemperature => Temperature,
            Extension { ref typ, ..} => typ.clone(),
        }
    }
}


/// An input operation available on an service.
#[derive(Debug, Clone)]
pub struct Input {
    /// The kind of value that can be obtained from this service.
    kind: ServiceKind,

    /// If `Some(duration)`, this service can be polled, i.e. it
    /// will respond when the FoxBox requests the latest value.
    /// Parameter `duration` indicates the smallest interval
    /// between two updates.
    ///
    /// Otherwise, the service cannot be polled and will push
    /// data to the FoxBox when it is available.
    ///
    /// # Examples
    ///
    /// - Long-running pollution or humidity sensors typically
    ///   do not accept requests and rather send batches of
    ///   data every 24h.
    poll: Option<Duration>,

    /// If `Some(duration)`, this service can send the data to
    /// the FoxBox whenever it is updated. Parameter `duration`
    /// indicates the smallest interval between two updates.
    ///
    /// Otherwise, the service cannot send data to the FoxBox
    /// and needs to be polled.
    trigger: Option<Duration>,

    /// Date at which the latest value was received, whether through
    /// polling or through a trigger.
    updated: chrono::DateTime<chrono::UTC>,
}

impl Input {
    /// The kind of value that can be obtained from this service.
    pub fn get_kind(&self) -> ServiceKind {
        self.kind.clone()
    }

    /// If `Some(duration)`, this service can be polled, i.e. it
    /// will respond when the FoxBox requests the latest value.
    /// Parameter `duration` indicates the smallest interval
    /// between two updates.
    ///
    /// Otherwise, the service cannot be polled and will push
    /// data to the FoxBox when it is available.
    ///
    /// # Examples
    ///
    /// - Long-running pollution or humidity sensors typically
    ///   do not accept requests and rather send batches of
    ///   data every 24h.
    pub fn get_poll(&self) -> Option<Duration> {
        self.poll.clone()
    }

    /// If `Some(duration)`, this service can send the data to
    /// the FoxBox whenever it is updated. Parameter `duration`
    /// indicates the smallest interval between two updates.
    ///
    /// Otherwise, the service cannot send data to the FoxBox
    /// and needs to be polled.
    pub fn get_trigger(&self) -> Option<Duration> {
        self.trigger.clone()
    }

    /// Date at which the latest value was received, whether through
    /// polling or through a trigger.
    ///
    /// # Limitation
    ///
    /// This is *not* a live view.
    pub fn get_updated(&self) -> chrono::DateTime<chrono::UTC> {
        self.updated.clone()
    }
}

/// An output operation available on an service.
#[derive(Debug, Clone)]
pub struct Output {
    /// The kind of value that can be sent to this service.
    kind: ServiceKind,

    /// If `Some(duration)`, this service supports pushing,
    /// i.e. the FoxBox can send values.
    push: Option<Duration>,

    /// Date at which the latest value was sent to the service.
    updated: chrono::DateTime<chrono::UTC>,
}

impl Output {
    /// The kind of value that can be sent to this service.
    pub fn get_kind(&self) -> ServiceKind {
        self.kind.clone()
    }

    /// If `Some(duration)`, this service supports pushing,
    /// i.e. the FoxBox can send values.
    pub fn get_push(&self) -> Option<Duration> {
        self.push.clone()
    }

    /// Date at which the latest value was sent.
    ///
    /// # Limitation
    ///
    /// This is *not* a live view.
    pub fn get_updated(&self) -> chrono::DateTime<chrono::UTC> {
        self.updated.clone()
    }
}

/// An service represents a single place where data can enter or
/// leave a device. Note that services support either a single kind
/// of input or a single kind of output. Devices that support both
/// inputs or outputs, or several kinds of inputs, or several kinds of
/// outputs, are represented as nodes containing several services.
#[derive(Debug, Clone)]
pub struct Service<IO> {
    /// Tags describing the service.
    ///
    /// These tags can be set by the user, adapters or
    /// applications. They are used to regroup services for rules.
    ///
    /// For instance "entrance".
    tags: Vec<String>,

    /// An id unique to this service.
    id: ServiceId,

    /// The node owning this service.
    node: NodeId,

    /// The update mechanism for this service.
    mechanism: IO,

    /// The last time the device was seen.
    last_seen: chrono::DateTime<chrono::UTC>,
}

impl<IO> Service<IO> {
    /// Tags describing the service.
    ///
    /// These tags can be set by the user, adapters or
    /// applications. They are used to regroup services for rules.
    ///
    /// For instance "entrance".
    pub fn get_tags<'a>(&'a self) -> &'a Vec<String> {
        &self.tags
    }

    /// An id unique to this service.
    pub fn get_id<'a>(&'a self) -> &'a ServiceId {
        &self.id
    }

    /// The node owning this service.
    pub fn get_node_id<'a>(&'a self) -> &'a NodeId {
        &self.node
    }

    /// The update mechanism for this service.
    pub fn get_mechanism<'a>(&'a self) -> &'a IO {
        &self.mechanism
    }

    /// The last time the device was seen.
    pub fn get_last_seen(&self) -> chrono::DateTime<chrono::UTC> {
        self.last_seen
    }
}

///
/// Values
///

#[derive(Debug, Clone)]
pub enum Type {
    ///
    /// # Trivial values
    ///

    /// An empty value. Used for instance to inform that a countdown
    /// has reached 0 or that a device is ready.
    Unit,

    /// A boolean. Used for instance for on-off switches, presence
    /// detectors, etc.
    Bool,

    ///
    /// # Time
    ///

    /// A duration. Used for instance in countdowns.
    Duration,

    /// A precise timestamp. Used for instance to determine when an
    /// event has taken place.
    TimeStamp,

    Temperature,

    ///
    /// ...
    ///
    Color,
}
