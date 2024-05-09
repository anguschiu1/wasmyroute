use leaflet;

use crate::geo::Coord;
#[derive(Default)]
pub struct Model {
    pub zoomlevel: i32,
    pub position: Coord,
    pub map: Option<leaflet::Map>,
}
