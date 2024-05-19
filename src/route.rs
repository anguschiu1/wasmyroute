use gpx::{read, Gpx};
use log::{error, info};

pub fn parse_gpx(text: String) -> Option<Gpx> {
    let data = text.as_bytes();
    match read(data) {
        Ok(gpx) => {
            info!("parse_gpx: Successfully parsed GPX string data.");
            gpx.tracks.iter().for_each(|track| {
                info!(
                    "Track name: {:?}",
                    track.name.as_ref().unwrap_or(&"N/A".to_string())
                );
                info!(
                    "Track type: {:?}",
                    track.type_.as_ref().unwrap_or(&"N/A".to_string())
                );
                info!("Number of track segment: {:?}", track.segments.len());
                track.segments.iter().for_each(|segment| {
                    info!("Number of point in this : {:?}", segment.points.len());
                });
            });
            Some(gpx)
        }
        Err(e) => {
            error!("parse_gpx: Failed to parse GPX string data. {:?}", e);
            None
        }
    }
}

#[cfg(test)]
// TODO: Add test cases for `parse_gpx`
// TODO: Add test cases for `parse_gpx_file`
mod tests {
    use super::*;

    #[test]
    fn test_parse_gpx() {
        let text_gpx = r#"<?xml version="1.0" encoding="UTF-8"?>
        <gpx creator="StravaGPX" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
          xsi:schemaLocation="http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd"
          version="1.1" xmlns="http://www.topografix.com/GPX/1/1">
          <trk>
            <name>Where was the rain?</name>
            <type>cycling</type>
            <trkseg>
              <trkpt lat="52.1350720" lon="0.1298080">
                <ele>23.6</ele>
              </trkpt>
              <trkpt lat="52.1351360" lon="0.1297770">
                <ele>23.8</ele>
              </trkpt>
              <trkpt lat="52.1352000" lon="0.1297490">
                <ele>23.8</ele>
              </trkpt>
            </trkseg>
          </trk>
        </gpx>"#;

        assert!(parse_gpx(text_gpx.to_string()).is_some());
        assert!(parse_gpx("".to_string()).is_none());
        assert!(parse_gpx("not a gpx".to_string()).is_none());
        assert_eq!(
            parse_gpx(text_gpx.to_string()).unwrap().tracks[0]
                .name
                .as_ref()
                .unwrap(),
            "Where was the rain?",
            "Testing track name"
        );
        assert_eq!(
            parse_gpx(text_gpx.to_string()).unwrap().tracks[0]
                .type_
                .as_ref()
                .unwrap(),
            "cycling",
            "Testing track type"
        );
        assert_eq!(
            parse_gpx(text_gpx.to_string()).unwrap().tracks[0]
                .segments
                .len(),
            1,
            "Testing number of track segments"
        );
        assert_eq!(
            parse_gpx(text_gpx.to_string()).unwrap().tracks[0].segments[0]
                .points
                .len(),
            3,
            "Testing number of track points"
        );
        let gpx_points = [
            (0.1298080, 52.1350720),
            (0.1297770, 52.1351360),
            (0.1297490, 52.1352000),
        ];
        for i in 0..3 {
            assert_eq!(
                parse_gpx(text_gpx.to_string()).unwrap().tracks[0].segments[0].points[i]
                    .point()
                    .x(),
                gpx_points[i].0,
                "Testing track points x coordinates"
            );
            assert_eq!(
                parse_gpx(text_gpx.to_string()).unwrap().tracks[0].segments[0].points[i]
                    .point()
                    .y(),
                gpx_points[i].1,
                "Testing track points y coordinates"
            );
        }
    }
}
