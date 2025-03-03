use chrono::DateTime;
use chrono::Utc;
use serde::Serializer;
use serde::Deserializer;
use serde::Serialize;
use serde::Deserialize;


#[derive(Debug,Clone,Eq,PartialEq) ]
pub struct DomainDateTime(pub DateTime<Utc>);

 
  


impl Serialize for DomainDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize DateTime<Utc> as an RFC3339 string
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}

impl<'de> Deserialize<'de> for DomainDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize an RFC3339 string into DateTime<Utc>
        let s = String::deserialize(deserializer)?;
        s.parse::<DateTime<Utc>>()
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

