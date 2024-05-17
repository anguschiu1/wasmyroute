// Define Rust structs corresponding to GPX elements
// Use an XML parsing crate to deserialize GPX XML into Rust structs

// Convert Rust structs to a format that can be exposed to JavaScript
// This might involve serializing the Rust structs to JSON

// Convert the GPX track points to a format Leaflet can use (e.g., an array of [lat, lon] pairs)
// Create a Leaflet polyline or layer group with these points
// Add the polyline or layer group to the map

use gpx::{errors::GpxError, read, Gpx};
use std::{cell::RefCell, rc::Rc};

use log::{error, info};
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    Event, File, FileReader,
};

/// Read the GPX file and parse it into a Rust struct
// TODO: return Result<Gpx, Box<dyn Error>> to handle errors
pub fn read_gpx_file(file: File) {
    let file_reader = match FileReader::new() {
        Ok(file_reader) => Rc::new(RefCell::new(file_reader)),
        Err(e) => {
            error!("Error creating FileReader: {:?}", e);
            return;
        }
    };

    // Start reading the file as text
    if let Err(e) = file_reader.borrow_mut().read_as_text(&file) {
        error!("Error reading file as text: {:?}", e);
    }

    // Clone the Rc<RefCell<FileReader>> for use inside the closure
    let file_reader_clone = Rc::clone(&file_reader);

    // Create a closure to capture the FileReader and perform actions once the file is read
    let onloadend_closure = Closure::wrap(Box::new(move |_event: Event| {
        // Use the cloned Rc<RefCell<FileReader>> here
        let file_reader = file_reader_clone.borrow();
        // Check if the file was read successfully
        match file_reader.result() {
            Ok(result) => {
                // Here you can access the file's contents as text
                // TODO:To avoid the copying and re-encoding, consider the JsString::try_from() function from js-sys instead.
                if let Some(text) = result.as_string() {
                    // info!("File content: {}", text);
                    // Now you can parse the text as needed, e.g., parse GPX data
                    // FIXME: improve the error handling, possibly returning Result<Gpx, Box<dyn Error>>
                    parse_gpx(text).unwrap();
                } else {
                    error!("Error reading file content as string.");
                }
            }
            Err(e) => {
                // TODO: rethrow the error
                error!("Error reading file: {:?}", e);
            }
        }
    }) as Box<dyn FnMut(Event)>);

    // Set the onloadend event handler of the FileReader
    file_reader
        .borrow_mut()
        .set_onloadend(Some(onloadend_closure.as_ref().unchecked_ref()));

    // Prevent the closure from being garbage-collected prematurely
    // Note: This is necessary but be cautious of memory leaks.
    onloadend_closure.forget();
}

fn parse_gpx(text: String) -> Result<Gpx, GpxError> {
    let data = text.as_bytes();
    let gpx = read(data)?;
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
    Ok(gpx)
}

#[cfg(test)]
// TODO: Add test cases for `parse_gpx`
// TODO: Add test cases for `parse_gpx_file`
mod tests {
    use super::*;

    #[test]
    fn simulate_gpx_parsing() {
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
            </trkseg>
          </trk>
        </gpx>"#;

        assert!(parse_gpx(text_gpx.to_string()).is_ok());
    }
}
