use gpx::{read, Gpx};
use log::{error, info};

use crate::model::Model;

use core::fmt;
use std::rc::Rc;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast, JsValue},
    Event, File, FileList, FileReader, HtmlInputElement,
};
use yew::prelude::*;
pub struct GpxFile;

#[derive(Properties, PartialEq)]
pub struct GpxFileProps {
    pub on_model_update: Callback<Model>,
}

pub enum Msg {
    Files(Vec<File>),
}

impl Component for GpxFile {
    type Message = Msg;
    type Properties = GpxFileProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <input
                    id="file-upload"
                    type="file"
                    accept=".gpx"
                    multiple={false}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Self::upload_files(input.files())
                    })}
                />
            </>
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(files) => {
                info!("Files uploaded: {:?}", files);
                files.iter().for_each(|file| {
                    if let Err(e) =
                        Self::read_gpx_file(file.clone(), ctx.props().on_model_update.clone())
                    {
                        error!("Error reading GPX file: {:?}", e);
                    }
                });
                true
            }
        }
    }
}
// Define a custom error type that wraps JsValue
#[derive(Debug)]
struct JsValueError(JsValue);

impl fmt::Display for JsValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JsValueError: {:?}", self.0)
    }
}

impl std::error::Error for JsValueError {}

impl GpxFile {
    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }
    /// Read the GPX file and parse it into a Rust struct
    // TODO: return Result<Gpx, Box<dyn Error>> to handle errors
    fn read_gpx_file(
        file: File,
        on_model_update: Callback<Model>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_reader = match FileReader::new() {
            Ok(file_reader) => Rc::new(file_reader),
            Err(e) => {
                error!("Error creating FileReader: {:?}", e);
                return Err(Box::new(JsValueError(e)));
            }
        };

        // Start reading the file as text
        // As described in https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText,
        // When the read operation is complete, the readyState property is changed to DONE,
        // the loadend event is triggered, and the result property contains the contents of the file as a text string.
        if let Err(e) = file_reader.clone().read_as_text(&file) {
            error!("Error reading file as text: {:?}", e);
            return Err(Box::new(JsValueError(e)));
        }

        // Clone the FileReader and model for use inside the closure
        let file_reader_rc: Rc<FileReader> = file_reader.clone();
        let mut model = Model::default();

        // Clone the application and message mapper from the orders for use in the callback closures.
        // let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());

        let gpx_file_callback = move |_event| {
            let file_reader = file_reader_rc.clone();
            match file_reader.result() {
                Ok(result) => {
                    // TODO:To avoid the copying and re-encoding, consider the JsString::try_from() function from js-sys instead.
                    if let Some(text) = result.as_string() {
                        model.gpx = Self::parse_gpx(text);
                        info!("GPX file read successfully.");
                        on_model_update.emit(model.clone());
                    } else {
                        error!("Error reading file content as string.");
                    }
                    // map::draw_gpx_route(&model_clone);
                }
                Err(e) => {
                    // TODO: rethrow the error
                    error!("Error reading file: {:?}", e);
                }
            }
            // app.update(msg_mapper(Msg::Position(model_clone.position)));
        };
        // Create a closure to capture the FileReader and perform actions once the file is read
        let onloadend_closure = Closure::wrap(Box::new(gpx_file_callback) as Box<dyn FnMut(Event)>);

        // Set the onloadend event handler of the FileReader
        file_reader
            .clone()
            .set_onloadend(Some(onloadend_closure.as_ref().unchecked_ref()));

        // Prevent the closure from being garbage-collected prematurely
        // Note: This is necessary but be cautious of memory leaks.
        onloadend_closure.forget();
        Ok(())
    }

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

        assert!(GpxFile::parse_gpx(text_gpx.to_string()).is_some());
        assert!(GpxFile::parse_gpx("".to_string()).is_none());
        assert!(GpxFile::parse_gpx("not a gpx".to_string()).is_none());
        assert_eq!(
            GpxFile::parse_gpx(text_gpx.to_string()).unwrap().tracks[0]
                .name
                .as_ref()
                .unwrap(),
            "Where was the rain?",
            "Testing track name"
        );
        assert_eq!(
            GpxFile::parse_gpx(text_gpx.to_string()).unwrap().tracks[0]
                .type_
                .as_ref()
                .unwrap(),
            "cycling",
            "Testing track type"
        );
        assert_eq!(
            GpxFile::parse_gpx(text_gpx.to_string()).unwrap().tracks[0]
                .segments
                .len(),
            1,
            "Testing number of track segments"
        );
        assert_eq!(
            GpxFile::parse_gpx(text_gpx.to_string()).unwrap().tracks[0].segments[0]
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
        for (i, &(lon, lat)) in gpx_points.iter().enumerate() {
            let parsed_gpx = GpxFile::parse_gpx(text_gpx.to_string()).unwrap();
            let point = &parsed_gpx.tracks[0].segments[0].points[i];
            assert_eq!(point.point().x(), lon, "Testing track points x coordinates");
            assert_eq!(point.point().y(), lat, "Testing track points y coordinates");
        }
    }
}
