use crate::geo::Coord;
use leaflet;

#[derive(Default, Clone)]
pub struct Model {
    pub zoomlevel: u8,
    pub position: Option<Coord>,
    pub theme: String,
    pub map: Option<leaflet::Map>,
    pub gpx: Option<gpx::Gpx>,
    pub position_lg: Option<leaflet::LayerGroup>,
    pub gpx_lg: Option<leaflet::LayerGroup>,
}
impl Model {
    pub fn zoom_in(&mut self) {
        self.zoomlevel = std::cmp::min(self.zoomlevel + 1, 18);
    }
    pub fn zoom_out(&mut self) {
        if self.zoomlevel > 1 {
            self.zoomlevel = std::cmp::max(self.zoomlevel - 1, 1);
        }
    }
}
