use std::path::Path;

use super::json::validate_json;

/// # heda config
/// Paths to user defined files and directories for heda (Human Editable Data Assistant)
#[allow(non_snake_case)]
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct ConfigJsonSchema {
    /// Path to a directory to store copies of the data before a migration is run.
    pub backupsDirectory: String,
    /// Path to the file containing the data as a JSON array.
    pub dataJson: String,
    /// Path to a directory to store outputs of plots.
    pub plotsDirectory: String,
    /// Path to the JSON Schema file for the data. File contents are managed automatically.
    pub schemaJson: String,
    /// Path to the .rhai file containing the rhai scripts for derivation, sorting, plotting, and migration.
    pub scriptsRhai: String,
    /// Path to the .rs file containing the struct typing an individual item in the JSON array.
    pub typeRs: String,
}

// rust parsing of the config JSON
pub struct Config {
    pub backups_directory: Box<Path>,
    pub data_json: Box<Path>,
    pub plots_directory: Box<Path>,
    pub schema_json: Box<Path>,
    pub scripts_rhai: Box<Path>,
    pub type_rs: Box<Path>,
}

impl Config {
    pub fn new(config_path: &Path) -> Self {
        let raw_json =
            validate_json::<ConfigJsonSchema>(config_path, Path::new("./config-schema.json"));
        let box_path = |base_path: &Path, path_str: &String| {
            Box::from(base_path.join(path_str).as_path())
        };
        Config {
            backups_directory: box_path(config_path, &raw_json.backupsDirectory),
            data_json: box_path(config_path, &raw_json.dataJson),
            plots_directory: box_path(config_path, &raw_json.plotsDirectory),
            schema_json: box_path(config_path, &raw_json.schemaJson),
            scripts_rhai: box_path(config_path, &raw_json.scriptsRhai),
            type_rs: box_path(config_path, &raw_json.typeRs),
        }
    }
}
