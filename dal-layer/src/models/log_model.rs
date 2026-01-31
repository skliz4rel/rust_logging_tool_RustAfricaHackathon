use chrono::Utc;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{alloc::System, time::SystemTime};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, ToSchema)]
pub enum LogLevel {
    INFO,
    ERROR,
    DEBUG,
    WARN,
    TRACE,
    OTHER,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Log {
    pub _id: ObjectId,
    pub my_service_id: ObjectId,
    pub level: LogLevel,
    pub line_content: String,
    pub created_at: DateTime,
}

impl TryFrom<LogRequest> for Log {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: LogRequest) -> Result<Self, Self::Error> {
        let chono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.created_at)
            .map_err(|err| format!("Format to parse start_time: {} ", err))?
            .with_timezone(&Utc)
            .into();

        Ok(Self {
            _id: ObjectId::new(),
            my_service_id: ObjectId::parse_str(&item.my_service_id).expect("Failed to parse owner"),
            level: item.level,
            line_content: item.line_content,
            created_at: DateTime::from(chono_datetime),
        })
    }
}

impl Log {
    pub fn from_bulk(items: Vec<LogRequest>) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        items.into_iter().map(Self::try_from).collect()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct LogRequest {
    pub level: LogLevel,
    pub my_service_id: String,
    pub line_content: String,
    pub created_at: String,
}

impl fmt::Display for LogRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
