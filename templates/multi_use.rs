use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Envelope<T> {
    #[serde(rename = "Body")]
    pub body: Body<T>,
}

#[derive(Deserialize, Debug)]
pub struct Body<T> {
    #[serde(
    {{#each actions as |action_name| }}
        alias = "{{ action_name }}Response",
    {{/each}}
    )]
    pub response: T,
}