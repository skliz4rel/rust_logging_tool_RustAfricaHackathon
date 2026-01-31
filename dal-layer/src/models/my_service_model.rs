use chrono::Utc;
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::{alloc::System, time::SystemTime};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyService {
    pub _id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    pub onboarded_datetime: DateTime,
}

impl TryFrom<MyServiceView> for MyService {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: MyServiceView) -> Result<Self, Self::Error> {
        let chono_datetime: SystemTime =
            chrono::DateTime::parse_from_rfc3339(&item.onboarded_datetime.unwrap())
                .map_err(|err| format!("Format to parse start_time: {} ", err))?
                .with_timezone(&Utc)
                .into();

        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            description: item.description,
            onboarded_datetime: DateTime::from(chono_datetime),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyServiceView {
    pub service_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub onboarded_datetime: Option<String>,
}

impl MyServiceView {
    pub fn from_bulk(
        items: Vec<MyService>,
    ) -> Result<Vec<MyServiceView>, Box<dyn std::error::Error>> {
        Ok(items
            .into_iter()
            .map(|s| MyServiceView {
                service_id: Some(s._id.to_string().clone()),
                name: s.name,
                description: s.description,
                onboarded_datetime: Some(s.onboarded_datetime.to_string()),
            })
            .collect())
    }
}
