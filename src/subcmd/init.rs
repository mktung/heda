use std::path::Path;

use crate::shared::{errors::RunningErrors, path::style_path};

// relative paths to add to base_path and create default files / directories
const PATH_CONFIG: &str = "./config.json";
const PATH_DATA: &str = "./data.json";
const PATH_SCHEMA: &str = "./schema.json";
const PATH_SCRIPTS: &str = "./scripts.rhai";
const PATH_TYPE: &str = "./type.rs";
const PATH_PLOTS: &str = "./plots";
const PATH_BACKUPS: &str = "./backups";

#[rustfmt::skip]
const DEFAULT_CONFIG: &str = const_format::formatcp!(r#"{{
    "dataJson": "{PATH_DATA}",
    "schemaJson": "{PATH_SCHEMA}",
    "scriptsRhai": "{PATH_SCRIPTS}",
    "typeRs": "{PATH_TYPE}",
    "plotsDirectory": "{PATH_PLOTS}",
    "backupsDirectory": "{PATH_BACKUPS}"
}}"#);

const DEFAULT_DATA: &str = "[]";
const DEFAULT_SCHEMA: &str = "{}";

// TODO
const DEFAULT_SCRIPTS: &str = "";

// TODO
const DEFAULT_TYPE: &str = "";

pub fn run_init(base_path: &Path) {
    let mut running_errors = RunningErrors::new();

    // should we handle every err type to make it read better?
    let mut create_file = |rel_path: &str, contents: &str| {
        let path = base_path.join(rel_path);
        let write_res = std::fs::write(&path, contents);
        if let Err(write_res) = write_res {
            running_errors.add_err(
                &write_res.to_string(),
                format!("Could not write file {}.", style_path(&path, rel_path)),
            );
        }
    };

    create_file(PATH_CONFIG, DEFAULT_CONFIG);
    create_file(PATH_DATA, DEFAULT_DATA);
    create_file(PATH_SCHEMA, DEFAULT_SCHEMA);
    create_file(PATH_SCRIPTS, DEFAULT_SCRIPTS);
    create_file(PATH_TYPE, DEFAULT_TYPE);

    let mut create_dir = |rel_path: &str| {
        let path = base_path.join(rel_path);
        let dir_res = std::fs::create_dir(&path);
        if let Err(dir_res) = dir_res {
            running_errors.add_err(
                &dir_res.to_string(),
                format!(
                    "Could not create directory {}.",
                    style_path(&path, rel_path)
                ),
            );
        }
    };

    create_dir(PATH_PLOTS);
    create_dir(PATH_BACKUPS);

    running_errors.print_errs();
}
