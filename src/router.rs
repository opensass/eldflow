#![allow(non_snake_case)]

use crate::components::navbar::HomeNavBar;
use crate::components::navbar::LoginNavBar;
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::signup::Register;
use crate::pages::trip::EditTrip;
use crate::pages::trip::EldLogs;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(HomeNavBar)]
    #[route("/")]
    Home {},
    #[end_layout]
    #[layout(LoginNavBar)]
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Register {},
    #[end_layout]
    #[route("/dashboard/trip/read/:id")]
    EldLogs { id: String },
    #[route("/dashboard/trip/edit/:id")]
    EditTrip { id: String },
    #[route("/dashboard")]
    Dashboard {},
}
