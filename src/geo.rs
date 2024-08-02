use leaflet::LatLng;

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
