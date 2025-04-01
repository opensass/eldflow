#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::server::auth::controller::auth;
use crate::server::common::response::SuccessResponse;
use crate::server::trip::model::*;
use crate::server::trip::request::*;
use crate::server::trip::response::*;
use std::env;

use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use regex::Regex;
#[cfg(feature = "server")]
use {
    crate::ai::get_ai,
    crate::db::get_client,
    crate::unsplash::get_unsplash_client,
    http_api_isahc_client::{Client as _, IsahcClient},
    rand::thread_rng,
    rand::Rng,
    unsplash_api::endpoints::common::EndpointRet,
    unsplash_api::endpoints::search_photos::SearchPhotos,
    unsplash_api::endpoints::search_photos::SearchPhotosResponseBodyOkJson,
    unsplash_api::objects::pagination::Pagination,
    unsplash_api::objects::rate_limiting::RateLimiting,
};

#[cfg(feature = "server")]
use reqwest::Client as ReqClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GooglePlacesResponse {
    pub predictions: Vec<Prediction>,
}

#[derive(Serialize, Deserialize)]
pub struct GoogleDurationResponse {
    pub distance: f64,
    pub duration: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Prediction {
    pub description: String,
    pub place_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DistanceMatrixResponse {
    rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    elements: Vec<Element>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Element {
    distance: Distance,
    duration: DurationValue,
}

#[derive(Serialize, Deserialize, Debug)]
struct Distance {
    value: f64, // Distance in meters
}

#[derive(Serialize, Deserialize, Debug)]
struct DurationValue {
    value: i64, // Duration in seconds
}

#[server]
pub async fn store_trip(
    req: StoreTripRequest,
) -> Result<SuccessResponse<TripResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;
    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME").unwrap());
    let trip_collection = db.collection::<Trip>("trips");
    let distance_duration = fetch_distance_duration(req.route_url).await?;

    let photo_url = fetch_cover(req.current_location.clone()).await?;

    let new_trip = Trip {
        id: ObjectId::new(),
        driver_id: user.id,
        current_location: req.current_location,
        pickup_location: req.pickup_location,
        picture: photo_url.unwrap_or_default(),
        dropoff_location: req.dropoff_location,
        cycle_used_hours: req.cycle_used_hours,
        status: req.status,
        distance_miles: Some(distance_duration.distance),
        estimated_duration: Some(distance_duration.duration as u64),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    trip_collection.insert_one(new_trip.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: TripResponse { id: new_trip.id },
    })
}

#[server]
pub async fn fetch_cover(topic: String) -> Result<Option<String>, ServerFnError> {
    let client = get_unsplash_client().await.lock().await;

    let search_photos = SearchPhotos::new(
        &env::var("UNSPLASH_API_KEY").expect("UNSPLASH_API_KEY must be set."),
        topic,
    );

    let response: EndpointRet<(SearchPhotosResponseBodyOkJson, Pagination, RateLimiting)> =
        client.respond_endpoint(&search_photos).await?;

    let mut extracted_data = Vec::new();

    if let EndpointRet::Ok((ok_json, _pagination, _rate_limiting)) = response {
        for photo in ok_json.results {
            let image_url = photo.urls.regular.to_string();

            extracted_data.push(image_url);
        }
    } else {
        tracing::error!("Unexpected response type");
    }

    if extracted_data.is_empty() {
        return Ok(None);
    }

    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..extracted_data.len());
    Ok(Some(extracted_data[random_index].clone()))
}
#[server]
pub async fn get_trips_for_user(
    req: GetTripsForUserRequest,
) -> Result<SuccessResponse<Vec<Trip>>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME").unwrap());
    let trip_collection = db.collection::<Trip>("trips");

    let filter = doc! {"driverId": user.id};
    let cursor = trip_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    let trips: Vec<Trip> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(SuccessResponse {
        status: "success".into(),
        data: trips,
    })
}

#[server]
pub async fn store_fueling_stop(
    req: StoreFuelingStopRequest,
) -> Result<SuccessResponse<FuelingStopResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
    let collection = db.collection::<FuelingStop>("fueling_stops");

    let trip_id = ObjectId::parse_str(&req.trip_id)?;
    let new_fueling_stop = FuelingStop {
        id: ObjectId::new(),
        trip_id,
        location: req.location,
        fuel_amount: req.fuel_amount,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_fueling_stop.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: FuelingStopResponse {
            id: new_fueling_stop.id,
        },
    })
}

#[server]
pub async fn store_eld_log(
    req: StoreEldLogRequest,
) -> Result<SuccessResponse<EldLogResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
    let collection = db.collection::<EldLog>("eld_logs");

    let new_log = EldLog {
        id: ObjectId::new(),
        driver_id: user.id,
        start_hour: req.start_hour,
        end_hour: req.end_hour,
        status: req.status,
        trip_id: ObjectId::parse_str(&req.trip_id)?,
        driving_hours: req.driving_hours,
        on_duty_hours: req.on_duty_hours,
        off_duty_hours: req.off_duty_hours,
        sleeper_berth_hours: req.sleeper_berth_hours,
        location: req.location,
        note: req.note,
        odometer_reading: req.odometer_reading,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_log.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: EldLogResponse { id: new_log.id },
    })
}

#[server]
pub async fn get_elds_for_user(
    req: GetEldsForUserRequest,
) -> Result<SuccessResponse<Vec<EldLog>>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME").unwrap());
    let collection = db.collection::<EldLog>("eld_logs");

    let filter = doc! {"driverId": user.id, "tripId":  ObjectId::parse_str(&req.trip_id)?};
    let cursor = collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    let elds: Vec<EldLog> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(SuccessResponse {
        status: "success".into(),
        data: elds,
    })
}

#[server]
pub async fn store_route(
    req: StoreRouteRequest,
) -> Result<SuccessResponse<RouteResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
    let collection = db.collection::<Route>("routes");

    let new_route = Route {
        id: ObjectId::new(),
        trip_id: ObjectId::parse_str(&req.trip_id)?,
        start_location: req.start_location,
        end_location: req.end_location,
        waypoints: req.waypoints,
        total_distance_miles: req.total_distance_miles,
        estimated_time_minutes: req.estimated_time_minutes,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_route.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: RouteResponse { id: new_route.id },
    })
}

#[server]
pub async fn store_waypoint(
    req: StoreWaypointRequest,
) -> Result<SuccessResponse<WaypointResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
    let collection = db.collection::<Waypoint>("waypoints");

    let new_waypoint = Waypoint {
        id: ObjectId::new(),
        location: req.location,
        eta: req.eta,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_waypoint.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: WaypointResponse {
            id: new_waypoint.id,
        },
    })
}
#[server]
pub async fn store_route_stop(
    req: StoreRouteStopRequest,
) -> Result<SuccessResponse<RouteStopResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
    let collection = db.collection::<RouteStop>("route_stops");

    let new_stop = RouteStop {
        id: ObjectId::new(),
        route_id: ObjectId::parse_str(&req.route_id)?,
        location: req.location,
        stop_type: req.stop_type,
        duration_minutes: req.duration_minutes,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_stop.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: RouteStopResponse { id: new_stop.id },
    })
}
#[server]
pub async fn store_daily_log(
    req: StoreDailyLogRequest,
) -> Result<SuccessResponse<DailyLogResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
    let collection = db.collection::<DailyLog>("daily_logs");

    let new_log = DailyLog {
        id: ObjectId::new(),
        driver_id: user.id,
        trip_id: ObjectId::parse_str(&req.trip_id)?,
        log_date: req.log_date,
        signature: req.signature,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_log.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: DailyLogResponse { id: new_log.id },
    })
}
#[server]
pub async fn store_log_entry(
    req: StoreLogEntryRequest,
) -> Result<SuccessResponse<LogEntryResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
    let collection = db.collection::<LogEntry>("log_entries");

    let new_entry = LogEntry {
        id: ObjectId::new(),
        log_id: ObjectId::parse_str(&req.log_id)?,
        time: req.time,
        status: req.status,
        location: req.location,
        remarks: req.remarks,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_entry.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: LogEntryResponse { id: new_entry.id },
    })
}

#[server]
pub async fn fetch_google_places_autocomplete(
    input: String,
    api_key: String,
) -> Result<GooglePlacesResponse, ServerFnError> {
    let url = format!(
        "https://maps.googleapis.com/maps/api/place/autocomplete/json?input={}&key={}",
        input, api_key
    );

    let client = ReqClient::new();

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| ServerFnError::new("Error fetching autocomplete data from Google API"))?;

    let google_response = response
        .json::<GooglePlacesResponse>()
        .await
        .map_err(|_| ServerFnError::new("Error parsing response from Google API"))?;

    Ok(google_response)
}

#[server]
pub async fn fetch_distance_duration(
    route_url: String,
) -> Result<GoogleDurationResponse, ServerFnError> {
    let client = ReqClient::new();

    use dioxus_logger::tracing;
    let response = client
        .get(&route_url)
        .send()
        .await
        .map_err(|_| ServerFnError::new("Error fetching autocomplete data from Google API"))?;

    let google_response = response
        .json::<DistanceMatrixResponse>()
        .await
        .map_err(|_| ServerFnError::new("Error parsing response from Google API"))?;

    if let Some(distance) = google_response
        .rows
        .first()
        .and_then(|row| row.elements.first())
        .map(|element| element.distance.value)
    {
        if let Some(duration) = google_response
            .rows
            .first()
            .and_then(|row| row.elements.first())
            .map(|element| element.duration.value)
        {
            return Ok(GoogleDurationResponse {
                distance: distance / 1609.34,
                duration: duration,
            });
        }
    }

    Ok(GoogleDurationResponse {
        distance: 0.0,
        duration: 0,
    })
}
