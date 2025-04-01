use bson::oid::ObjectId;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntryResponse {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TripResponse {
    pub id: ObjectId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuelingStopResponse {
    pub id: ObjectId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EldLogResponse {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteResponse {
    pub id: ObjectId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WaypointResponse {
    pub id: ObjectId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteStopResponse {
    pub id: ObjectId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyLogResponse {
    pub id: ObjectId,
}
