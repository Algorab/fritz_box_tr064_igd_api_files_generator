use serde::Deserialize;

{{#each actions as |action| }}
#[derive(Deserialize, Debug)]
pub struct {{ action.name }} {
    {{#each action.fields as |field | }}
    #[serde(rename = "{{ xml_name }}")]
    pub {{field.name}}: {{type}},
    {{/each}}
}

{{/each}}

