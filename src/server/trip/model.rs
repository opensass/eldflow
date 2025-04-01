#![allow(non_snake_case)]

use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Trip {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "driverId")]
    pub driver_id: ObjectId,
    #[serde(rename = "currentLocation")]
    pub current_location: String,
    pub picture: String,
    #[serde(rename = "pickupLocation")]
    pub pickup_location: String,
    #[serde(rename = "dropoffLocation")]
    pub dropoff_location: String,
    #[serde(rename = "cycleUsedHours")]
    pub cycle_used_hours: f64,
    #[serde(rename = "status")]
    pub status: String, // "Pending", "Ongoing", "Completed"
    #[serde(rename = "distanceMiles")]
    pub distance_miles: Option<f64>,
    #[serde(rename = "estimatedDuration")]
    pub estimated_duration: Option<u64>, // In minutes
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FuelingStop {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "tripId")]
    pub trip_id: ObjectId,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "fuelAmount")]
    pub fuel_amount: f64, // Gallons
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EldLog {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "driverId")]
    pub driver_id: ObjectId,
    #[serde(rename = "tripId")]
    pub trip_id: ObjectId,
    #[serde(rename = "drivingHours")]
    pub driving_hours: f64,
    #[serde(rename = "startHour")]
    pub start_hour: f64,
    #[serde(rename = "endHour")]
    pub end_hour: f64,
    #[serde(rename = "onDutyHours")]
    pub on_duty_hours: f64,
    #[serde(rename = "offDutyHours")]
    pub off_duty_hours: f64,
    #[serde(rename = "sleeperBerthHours")]
    pub sleeper_berth_hours: f64,
    pub location: String,
    pub status: String,
    pub note: String,
    #[serde(rename = "odometerReading")]
    pub odometer_reading: Option<f64>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Route {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "tripId")]
    pub trip_id: ObjectId,
    #[serde(rename = "startLocation")]
    pub start_location: String,
    #[serde(rename = "endLocation")]
    pub end_location: String,
    #[serde(rename = "waypoints")]
    pub waypoints: Vec<Waypoint>,
    #[serde(rename = "totalDistanceMiles")]
    pub total_distance_miles: f64,
    #[serde(rename = "estimatedTimeMinutes")]
    pub estimated_time_minutes: u64,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Waypoint {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "eta")]
    pub eta: Option<DateTime<Utc>>, // Estimated Time of Arrival
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RouteStop {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "routeId")]
    pub route_id: ObjectId,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "stopType")]
    pub stop_type: String, // "Rest", "Fueling", "Inspection"
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: u64,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DailyLog {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "driverId")]
    pub driver_id: ObjectId,
    #[serde(rename = "tripId")]
    pub trip_id: ObjectId,
    #[serde(rename = "logDate")]
    pub log_date: DateTime<Utc>,
    #[serde(rename = "signature")]
    pub signature: Option<String>, // Base64 driver signature
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogEntry {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "logId")]
    pub log_id: ObjectId,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>,
    #[serde(rename = "status")]
    pub status: String, // "Driving", "On-Duty", "Off-Duty", "Sleeper Berth"
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "remarks")]
    pub remarks: Option<String>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
