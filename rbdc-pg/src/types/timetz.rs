use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use byteorder::{BigEndian, ReadBytesExt};
use rbdc::Error;
use rbs::Value;
use std::fmt::{Display, Formatter};
use std::io::Cursor;
use std::time::Duration;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Timez")]
pub struct Timetz(pub OffsetTz);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "OffsetTz")]
pub struct OffsetTz {
    pub time: fastdate::Time,
    pub offset: i32,
}

impl Display for OffsetTz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OffsetTz")
            .field("time", &self.time)
            .field("offset", &self.offset)
            .finish()
    }
}

impl Display for Timetz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timetz({})", self.0)
    }
}

impl From<Timetz> for Value {
    fn from(arg: Timetz) -> Self {
        Value::Ext("Timetz", Box::new(Value::String(arg.0.to_string())))
    }
}

impl Decode for Timetz {
    fn decode(value: PgValue) -> Result<Self, Error> {
        match value.format() {
            PgValueFormat::Binary => {
                let mut buf = Cursor::new(value.as_bytes()?);

                // TIME is encoded as the microseconds since midnight
                let microseconds = buf.read_i64::<BigEndian>()?;

                // OFFSET is encoded as seconds from UTC
                let seconds = buf.read_i32::<BigEndian>()?;

                Ok(Self(OffsetTz {
                    time: fastdate::Time::from(Duration::from_micros(microseconds as u64)),
                    offset: seconds,
                }))
            }
            PgValueFormat::Text => {
                // the `time` crate has a limited ability to parse and can't parse the
                // timezone format
                Err("reading a `TIMETZ` value in text format is not supported.".into())
            }
        }
    }
}

impl Encode for Timetz {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let _ = (Duration::from(self.0.time).as_micros() as i64).encode(buf)?;
        let _ = self.0.offset.encode(buf)?;
        Ok(IsNull::No)
    }
}
