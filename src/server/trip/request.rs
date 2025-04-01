use crate::server::trip::model::Waypoint;
use bson::oid::ObjectId;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreTripRequest {
    pub token: String,
    pub current_location: String,
    pub pickup_location: String,
    pub dropoff_location: String,
    pub cycle_used_hours: f64,
    pub status: String,
    pub route_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTripRequest {
    pub token: String,
    pub trip_id: String,
    pub status: Option<String>,
    pub current_location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreFuelingStopRequest {
    pub token: String,
    pub trip_id: String,
    pub location: String,
    pub fuel_amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreEldLogRequest {
    pub token: String,
    pub trip_id: String,
    pub status: String,
    pub start_hour: f64,
    pub end_hour: f64,
    pub driving_hours: f64,
    pub on_duty_hours: f64,
    pub off_duty_hours: f64,
    pub sleeper_berth_hours: f64,
    pub location: String,
    pub note: String,
    pub odometer_reading: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreRouteRequest {
    pub token: String,
    pub trip_id: String,
    pub start_location: String,
    pub end_location: String,
    pub waypoints: Vec<Waypoint>,
    pub total_distance_miles: f64,
    pub estimated_time_minutes: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreWaypointRequest {
    pub token: String,
    pub location: String,
    pub eta: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreRouteStopRequest {
    pub token: String,
    pub route_id: String,
    pub location: String,
    pub stop_type: String,
    pub duration_minutes: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreDailyLogRequest {
    pub token: String,
    pub trip_id: String,
    pub log_date: DateTime<Utc>,
    pub signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreLogEntryRequest {
    pub token: String,
    pub log_id: String,
    pub time: DateTime<Utc>,
    pub status: String,
    pub location: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteTripRequest {
    pub token: String,
    pub trip_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTripsForUserRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetEldsForUserRequest {
    pub token: String,
    pub trip_id: String,
}
