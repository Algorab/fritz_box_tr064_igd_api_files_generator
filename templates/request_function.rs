{{#each request_functions as |func| }}
pub fn generate_{{ func.name_rusty }}_request(
    {{~ #each func.parameter as |parameter| ~}}
    {{~ parameter.parameter_name_rusty }}: {{{ string_to_string_slice parameter.type_name }}}{{#unless @last}},{{/unless}}
    {{~ /each }}) -> (String, String, String) {

    let uri = "{{ func.control_url }}";
    let header = format!("{{ func.service_type }}#{{func.action_name}}");

    let body = format!(
    r#"<?xml version="1.0">
    <s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/"s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
        <s:Body>
            <u:{{ func.action_name }} xmlns:u="{{ func.service_type }}">
                {{ #each func.parameter as |parameter| }}
                <{{ parameter.parameter_name }}>{}</{{ parameter.parameter_name }}>
                {{ /each }}
            </u:{{ func.action_name }}>
        </s:Body>
    </s:Envelope>"#{{~#each func.parameter as |parameter| ~}}
                       , {{ parameter.parameter_name_rusty ~}}
                      {{~ /each }});

    (uri.to_string(), header, body)
}

{{/each}}

