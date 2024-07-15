#[derive(Default, Clone)]
pub struct Model {
    pub zoomlevel: u8,
    pub map: Option<leaflet::Map>,
    pub gpx: Option<gpx::Gpx>,
    pub position_lg: Option<leaflet::LayerGroup>,
    pub gpx_lg: Option<leaflet::LayerGroup>,
}
impl Model {}
