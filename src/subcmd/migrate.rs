use std::path::Path;

use crate::shared::{
    backups::get_backup_path,
    config::Config,
    datum::Datum,
    json::validate_json,
    path::style_path,
    scripts::{RhaiSpace, ScriptFn},
};

pub fn run_migrate(config_path: &Path) {
    let config = Config::new(config_path);

    // make backup
    let backup_path = get_backup_path(&config.data_json);
    std::fs::copy(&config.data_json, &backup_path).expect(
        format!(
            "Should copy backup of {} to {}",
            style_path(&config.data_json, "data JSON"),
            style_path(&backup_path, "backups directory")
        )
        .as_str(),
    );

    // migrate
    let mut rhai_space = RhaiSpace::new(&config.scripts_rhai);
    let data = validate_json::<Vec<Datum>>(&config.data_json, &config.schema_json);
    let mut new_data: Vec<Datum> = Vec::new();
    for datum in data {
        let fn_res: rhai::Dynamic =
            rhai_space.call_fn::<rhai::Dynamic>(ScriptFn::Migrate, (datum,));
        let new_datum = fn_res.cast::<Datum>();
        new_data.push(new_datum);
    }
    let new_data_str =
        serde_json::ser::to_string_pretty(&new_data).expect("turned migration result to string");
    std::fs::write(&config.data_json, &new_data_str).expect("replaced old data");

    // TODO make new type somewhere??
}
