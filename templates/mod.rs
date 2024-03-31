{{#each mod_files as |mod_file| }}
pub mod {{ mod_file }};
{{/each}}