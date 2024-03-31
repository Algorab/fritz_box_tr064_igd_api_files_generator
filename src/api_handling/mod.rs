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
