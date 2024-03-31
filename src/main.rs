#[macro_use]
extern crate serde;
extern crate serde_xml_rs;

mod api_handling;

use handlebars::{DirectorySourceOptions, Handlebars, handlebars_helper};
use crate::api_handling::api_desc_dir::ApiDescDir;

/// If you can't reach your target or wish to specify it via IP, this is the place.
const ADDRESS: &str = "http://10.0.0.254:49000";
/// Specify the requests output folder.
const REQUESTS_OUTPUT_FOLDER: &str = "requests";
/// Specify the responses output folder.
const RESPONSES_OUTPUT_FOLDER: &str = "responses";
/// Specify the TR-064 folder and files prefix
const TR064_PREFIX: &str = "tr064";
/// Specify the IGD folder and files prefix
const IGD_PREFIX: &str = "igd";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut handlebars = Handlebars::new();
    handlebars_helper!(string_to_string_slice: |value: String| if value == "String" {"&str".to_string()} else {value});
    handlebars.register_helper("string_to_string_slice", Box::new(string_to_string_slice));

    let directory_source_options = DirectorySourceOptions{
        tpl_extension: ".rs".to_string(),
        hidden: false,
        temporary: false,
    };
    handlebars.register_templates_directory("templates", directory_source_options).expect("template directory");

    let resp = reqwest::blocking::get(format!("{}/tr64desc.xml", ADDRESS).as_str())?.text()?;
    let tr64desc: ApiDescDir = serde_xml_rs::from_str(&resp)?;
    tr64desc.generate_files(
        ADDRESS,
        RESPONSES_OUTPUT_FOLDER.to_string(),
        REQUESTS_OUTPUT_FOLDER.to_string(),
        Some(TR064_PREFIX.to_string()),
        &handlebars,
    );
    let resp = reqwest::blocking::get(format!("{}/igddesc.xml", ADDRESS).as_str())?.text()?;
    let igddesc: ApiDescDir = serde_xml_rs::from_str(&resp)?;
    igddesc.generate_files(
        ADDRESS,
        RESPONSES_OUTPUT_FOLDER.to_string(),
        REQUESTS_OUTPUT_FOLDER.to_string(),
        Some(IGD_PREFIX.to_string()),
        &handlebars
    );

    Ok(())
}
