use crate::api_handling::api_desc::ApiDesc;
use crate::api_handling::api_desc_dir::{Device, OutputFiles};

pub mod api_desc;
pub mod api_desc_dir;
pub mod helper;

#[derive(Debug, Default, Serialize)]
pub struct TemplateAction {
    name: String,
    fields: Vec<Field>
}

#[derive(Debug, Default, Serialize)]
pub struct Field {
    name: String,
    xml_name: String,
    r#type: String
}

/// Handles all services of a device and all contained devices.
fn handle_device(device: &Device, address: &str, output_files: &mut OutputFiles) {
    for service in &device.service_list.service {
        let resp = reqwest::blocking::get(format!("{}{}", address, service.scpd_url).as_str())
            .unwrap()
            .text()
            .unwrap();
        let scdp: ApiDesc = serde_xml_rs::from_str(&resp).unwrap();
        service.service_type.split(':').nth(3).unwrap();
        scdp.fill_output_files(
            output_files,
            service.service_type.split(':').nth(3).unwrap(),
            &service.control_url,
            &service.service_type,
        );
    }
    for local_device in &device.device_list.device {
        handle_device(local_device, address, output_files);
    }
}
