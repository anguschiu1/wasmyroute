// Define Rust structs corresponding to GPX elements
// Use an XML parsing crate to deserialize GPX XML into Rust structs

// Convert Rust structs to a format that can be exposed to JavaScript
// This might involve serializing the Rust structs to JSON

// Convert the GPX track points to a format Leaflet can use (e.g., an array of [lat, lon] pairs)
// Create a Leaflet polyline or layer group with these points
// Add the polyline or layer group to the map

use gpx::read;
use std::{cell::RefCell, rc::Rc};

use log::{error, info};
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    Event, File, FileReader,
};

pub fn parse_gpx_file(file: File) {
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
                if let Some(text) = result.as_string() {
                    info!("File content: {}", text);
                    // Now you can parse the text as needed, e.g., parse GPX data
                    parse_gpx(text);
                } else {
                    error!("Error reading content as string.");
                }
            }
            Err(e) => {
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

fn parse_gpx(text: String) {
    // Convert the GPX data string into a byte slice
    let data = text.as_bytes();

    // Attempt to parse the GPX data
    match read(data) {
        Ok(gpx) => {
            info!("Successfully parsed GPX data.");
            // Here you can work with the `gpx` variable, which is of type `Gpx`
            // For example, accessing waypoints, tracks, etc.
            info!(
                "Successfully parsed GPX data.\nTracks: {:?}, \nWaypoints: {:?}, \nRoutes: {:?}, \nVersion: {:?}, \nCreator: {:?} \nMetadata: {:?} \n",
                gpx.tracks.len(),
                gpx.waypoints.len(),
                gpx.routes.len(),
                gpx.version,
                gpx.creator.unwrap(),
                gpx.metadata.unwrap(),
            );
        }
        Err(e) => {
            error!("Failed to parse GPX data: {:?}", e);
        }
    }
}
