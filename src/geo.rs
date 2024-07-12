use gloo_utils::window;
use leaflet::LatLng;
use log::info;
use std::ops::Deref;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    Coordinates, Geolocation, Position, PositionError, PositionOptions,
};
use yew::prelude::*;

use crate::{map, model::Model};
#[derive(Default, Clone, Copy, Debug)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

impl From<Coord> for LatLng {
    fn from(coord: Coord) -> Self {
        LatLng::new(coord.lat, coord.lon)
    }
}
// Implement PartialEq for Coord
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lon == other.lon
    }
}
