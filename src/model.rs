use leaflet;

use crate::geo::Coord;
#[derive(Default)]
pub struct Model {
    pub zoomlevel: u8,
    pub position: Coord,
    pub map: Option<leaflet::Map>,
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
