use std::collections::HashMap;
use crate::api_handling::api_desc_dir::{
    OutputFiles, ParameterAndType, RequestFile, RequestFunction, ResponseFile, SpecVersion,
};
use crate::api_handling::{Field, TemplateAction};
use crate::api_handling::helper::rustify_string;

///Struct to deserialize response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ApiDesc {
    #[serde(rename = "specVersion")]
    pub spec_version: SpecVersion,
    #[serde(rename = "actionList")]
    pub action_list: ActionList,
    #[serde(rename = "serviceStateTable")]
    pub service_state_table: ServiceStateTable,
}
///Struct to deserialize the ActionList part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ActionList {
    #[serde(rename = "action")]
    #[serde(default)]
    pub action: Vec<Action>,
}
///Struct to deserialize the Action part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct Action {
    pub name: String,
    #[serde(rename = "argumentList")]
    #[serde(default)]
    pub argument_list: ArgumentList,
}
///Struct to deserialize the ArgumentList part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ArgumentList {
    #[serde(rename = "argument")]
    pub argument: Vec<Argument>,
}

///Struct to deserialize the Argument part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct Argument {
    pub name: String,
    pub direction: String,
    #[serde(rename = "relatedStateVariable")]
    pub related_state_variable: String,
}

///Struct to deserialize the ServiceStateTable part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ServiceStateTable {
    #[serde(rename = "stateVariable")]
    pub state_variable: Vec<StateVariable>,
}

///Struct to deserialize the StateVariable part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct StateVariable {
    pub name: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    pub default_value: String,
    #[serde(rename = "allowedValueList")]
    #[serde(default)]
    pub allowed_value_list: AllowedValueList,
}

///Struct to deserialize the AllowedValueList part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct AllowedValueList {
    #[serde(rename = "allowedValue")]
    pub allowed_value: Vec<String>,
}

impl ApiDesc {
    ///Takes  an `OutputFiles`, `name`, `control_url` and `service_type` and populates the `OutputFiles`
    pub fn fill_output_files(
        &self,
        output_files: &mut OutputFiles,
        name: &str,
        control_url: &str,
        service_type: &str,
    ) {
        let rusty_name = rustify_string(name);
        let mut response_file = ResponseFile::default();
        let mut request_file = RequestFile::default();
        response_file.name = rusty_name.clone();
        request_file.name = rusty_name;

        let mut actions: Vec<TemplateAction> = Vec::new();

        for action in &self.action_list.action {

            let mut request_function = RequestFunction::new(
                action.name.clone(),
                service_type.to_string(),
                control_url.to_string()
            );

            let mut template_action = TemplateAction {
                name: format!("{}Response", action.name
                    .replace(['-', '_'], "")),
                ..Default::default()
            };

            output_files
                .annotation_string
                .push(action.name.clone());

            let mut fields: Vec<Field> = Vec::new();

            for argument in &action.argument_list.argument {

                let mut field = Field::default();

                if argument.direction == "out" {
                    field.xml_name = argument.name.clone();
                    field.name = rustify_string(&argument.name);
                    field.r#type = self
                        .search_state_variable_type(argument.related_state_variable.as_str());

                    fields.push(field);

                } else if argument.direction == "in" {
                    let param = ParameterAndType {
                        parameter_name: argument.name.clone(),
                        parameter_name_rusty: rustify_string(&argument.name),
                        type_name: self.search_state_variable_type(argument.related_state_variable.as_str())
                    };

                    request_function.parameter.push(param);
                }

            }
            template_action.fields = fields;
            actions.push(template_action);

            request_file.request_functions.push(request_function);
        }

        // generate the response file content via handlebars
        let mut template_data:HashMap<&str, Vec<TemplateAction>> = HashMap::new();
        template_data.insert("actions", actions);
        let template_actions_content = output_files.handlebars.render("action_response_types", &template_data).unwrap();
        response_file.content = template_actions_content;
        output_files.response_files.push(response_file);

        output_files.request_files.push(request_file);
    }

    /// Searches for the requested variable and returns the corresponding type.
    /// If you encounter a panic here, please open a ticket with the output of `_ => print!("{}", variable.data_type.as_str()),`
    fn search_state_variable_type(&self, state_variable_name: &str) -> String {
        for variable in &self.service_state_table.state_variable {
            if state_variable_name.eq(&variable.name) {
                match variable.data_type.as_str() {
                    "boolean" => return String::from("bool"),
                    "ui1" => return String::from("u32"),
                    "ui2" => return String::from("u32"),
                    "ui4" => return String::from("u32"),
                    "i1" => return String::from("i32"),
                    "i2" => return String::from("i32"),
                    "i4" => return String::from("i32"),
                    "string" => return String::from("String"),
                    "uuid" => return String::from("String"),
                    "dateTime" => return String::from("String"),
                    _ => print!("{}", variable.data_type.as_str()),
                };
            }
        }

        panic!("variable Type not implemented, please open a ticket")
    }

}
