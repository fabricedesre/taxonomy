//!
//! Values manipulated by services
//!
use std::cmp::{PartialOrd, Ordering};
use std::time::Duration;
use std::str::FromStr;
use std::sync::Arc;

use serde_json;
use chrono;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer, Error};

///
/// The type of values manipulated by endpoints.
///
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
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
    String, 
    ///
    /// ...
    ///
    Color,
    Json,
    Binary,
    ExtNumeric,
}

/// A temperature. Internal representation may be either Fahrenheit or
/// Celcius. The FoxBox adapters are expected to perform conversions
/// to the format requested by their devices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Temperature {
    /// Fahrenheit
    F(f64),
    /// Celcius
    C(f64),
}

impl Temperature {
    /// Get a temperature in Fahrenheit.
    pub fn as_f(&self) -> f64 {
        unimplemented!();
    }

    /// Get a temperature in Celcius.
    pub fn as_c(&self) -> f64 {
        unimplemented!();
    }
}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_c().partial_cmp(&other.as_c())
    }
}

/// A color. Internal representation may vary. The FoxBox adapters are
/// expected to perform conversions to the format requested by their
/// device.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Color {
    RGBA(f64, f64, f64, f64, f64)
}

/// Representation of an object in JSON. It is often (albeit not
/// always) possible to choose a more precise data structure for
/// representing values send/accepted by a service. If possible,
/// adapters should rather pick such more precise data structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Json(pub serde_json::value::Value);

impl PartialOrd for Json {
    /// Two Json objects are never comparable to each other.
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        None
    }
}

/// A data structure holding a numeric value of a type that has not
/// been standardized yet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtNumeric {
    pub value: f64,

    /// The vendor. Used for namespacing purposes, to avoid
    /// confusing two incompatible extensions with similar
    /// names. For instance, "foxlink@mozilla.com".
    pub vendor: String,

    /// Identification of the adapter introducing this value.
    /// Designed to aid with tracing and debugging.
    pub adapter: String,

    /// A string describing the nature of the value, designed to
    /// aid with type-checking.
    ///
    /// Examples: `"GroundHumidity"`.
    pub kind: String,
}

impl PartialOrd for ExtNumeric {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.vendor != other.vendor {
            return None;
        } else if self.kind != other.kind {
            return None;
        } else {
            self.value.partial_cmp(&other.value)
        }
    }
}

/// Representation of an actual value that can be sent to/received
/// from a service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Unit,
    Bool(bool),
    Duration(ValDuration),
    TimeStamp(TimeStamp),
    Temperature(Temperature),
    Color(Color),
    String(Arc<String>),

    // FIXME: Add more as we identify needs

    /// A numeric value representing a unit that has not been
    /// standardized yet into the API.
    ExtNumeric(ExtNumeric),

    /// A Json value. We put it behind an `Arc` to make sure that
    /// cloning remains unexpensive.
    Json(Arc<Json>),

    /// Binary data.
    Binary {
        /// The actual data. We put it behind an `Arc` to make sure
        /// that cloning remains unexpensive.
        data: Arc<Vec<u8>>,
        mimetype: String
    }
}

impl Value {
    pub fn get_type(&self) -> Type {
        match *self {
            Value::Unit => Type::Unit,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Duration(_) => Type::Duration,
            Value::TimeStamp(_) => Type::TimeStamp,
            Value::Temperature(_) => Type::Temperature,
            Value::Color(_) => Type::Color,
            Value::Json(_) => Type::Json,
            Value::Binary{..} => Type::Binary,
            Value::ExtNumeric(_) => Type::ExtNumeric,
        }
    }
}

impl PartialOrd for Value {
    /// Two values of the same type can be compared using the usual
    /// comparison for values of this type. Two values of distinct
    /// types cannot be compared.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use self::Value::*;
        use std::cmp::Ordering::*;
        match (self, other) {
            (&Unit, &Unit) => Some(Equal),
            (&Unit, _) => None,

            (&Bool(a), &Bool(b)) => a.partial_cmp(&b),
            (&Bool(_), _) => None,

            (&Duration(ref a), &Duration(ref b)) => a.partial_cmp(b),
            (&Duration(_), _) => None,

            (&TimeStamp(ref a), &TimeStamp(ref b)) => a.partial_cmp(b),
            (&TimeStamp(_), _) => None,

            (&Temperature(ref a), &Temperature(ref b)) => a.partial_cmp(b),
            (&Temperature(_), _) => None,

            (&Color(ref a), &Color(ref b)) => a.partial_cmp(b),
            (&Color(_), _) => None,

            (&ExtNumeric(ref a), &ExtNumeric(ref b)) => a.partial_cmp(b),
            (&ExtNumeric(_), _) => None,

            (&String(ref a), &String(ref b)) => a.partial_cmp(b),
            (&String(_), _) => None,

            (&Json(ref a), &Json(ref b)) => a.partial_cmp(b),
            (&Json(_), _) => None,

            (&Binary{mimetype: ref a_mimetype, data: ref a_data},
             &Binary{mimetype: ref b_mimetype, data: ref b_data}) if a_mimetype == b_mimetype => a_data.partial_cmp(b_data),
            (&Binary{..}, _) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ValDuration(Duration);
impl ValDuration {
    pub fn new(duration: Duration) -> Self {
        ValDuration(duration)
    }
}
impl Serialize for ValDuration {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer {
        let as_ms : u64 = self.0.as_secs() * 1000
            + (self.0.subsec_nanos() as u64) / 1_000_000;
        as_ms.serialize(serializer)
    }
}
impl Deserialize for ValDuration {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer {
        let as_sec : f64 = try!(f64::deserialize(deserializer));
        Ok(ValDuration(Duration::new(as_sec as u64, as_sec.fract() as u32)))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TimeStamp(chrono::DateTime<chrono::UTC>);
impl TimeStamp {
    pub fn from_datetime(datetime: chrono::DateTime<chrono::UTC>) -> Self {
        TimeStamp(datetime)
    }
    pub fn from_s(s: i64) -> Self {
        use chrono::*;
        let naive = chrono::naive::datetime::NaiveDateTime::from_timestamp(s, 0);
        let date = DateTime::<UTC>::from_utc(naive, chrono::UTC);
        TimeStamp(date)
    }
}
impl Serialize for TimeStamp {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer {
        let str = self.0.to_rfc3339();
        str.serialize(serializer)
    }
}
impl Deserialize for TimeStamp {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer {
        let str = try!(String::deserialize(deserializer));
        match chrono::DateTime::<chrono::UTC>::from_str(&str) {
            Ok(dt) => Ok(TimeStamp(dt)),
            Err(_) => Err(D::Error::syntax("Invalid date"))
        }
    }
}


#[derive(Clone, Deserialize, Serialize)]
/// A comparison between two values.
pub enum Range {
    /// Leq(x) accepts any value v such that v <= x.
    Leq(Value),

    /// Geq(x) accepts any value v such that v >= x.
    Geq(Value),

    /// BetweenEq {min, max} accepts any value v such that `min <= v`
    /// and `v <= max`. If `max < min`, it never accepts anything.
    BetweenEq {min:Value, max:Value},

    /// OutOfStrict {min, max} accepts any value v such that `v < min`
    /// or `max < v`
    OutOfStrict {min:Value, max:Value},


    /// Eq(x) accespts any value v such that v == x
    Eq(Value),
}

impl Range {
    /// Determine if a value is accepted by this range.
    pub fn contains(&self, value: &Value) -> bool {
        use self::Range::*;
        match *self {
            Leq(ref max) => value <= max,
            Geq(ref min) => value >= min,
            BetweenEq {ref min, ref max} => min <= value && value <= max,
            OutOfStrict {ref min, ref max} => value < min || max < value,
            Eq(ref val) => value == val,
        }
    }

    /// Get the type associated to this range.
    ///
    /// If this range has a `min` and a `max` with conflicting types,
    /// produce an error.
    pub fn get_type(&self) -> Result<Type, ()> {
        use self::Range::*;
        match *self {
            Leq(ref v) | Geq(ref v) | Eq(ref v) => Ok(v.get_type()),
            BetweenEq{ref min, ref max} | OutOfStrict{ref min, ref max} => {
                let min_typ = min.get_type();
                let max_typ = max.get_type();
                if min_typ == max_typ {
                    Ok(min_typ)
                } else {
                    Err(())
                }
            }
        }
    }
}
