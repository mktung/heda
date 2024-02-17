use std::path::Path;

use crate::shared::errors::RunningErrors;

use super::path::style_path;

fn read_json(path: &Path) -> serde_json::Value {
    // reading into memory is faster than parsing via stream
    // https://github.com/serde-rs/json/issues/160
    let str_res = std::fs::read_to_string(path);
    let str = str_res.expect(format!("Should read {}", style_path(path, "JSON")).as_str());
    let json_res = serde_json::from_str(str.as_str());
    json_res.expect(format!("Should parse {} as JSON", style_path(path, "file")).as_str())
}

pub fn validate_json<Instance: for<'a> serde::Deserialize<'a>>(
    instance_path: &Path,
    schema_path: &Path,
) -> Instance {
    let instance_json = read_json(instance_path);
    let schema_json = read_json(schema_path);
    let schema = jsonschema::JSONSchema::options()
        .compile(&schema_json)
        .expect(
            format!(
                "{} should be a valid JSON schema",
                style_path(schema_path, "schema")
            )
            .as_str(),
        );
    let validate_res = schema.validate(&instance_json);
    if let Err(errors) = validate_res {
        let err_type = "Validation error".to_string();
        let mut running_errors = RunningErrors::new();
        for error in errors {
            running_errors.add_err(&err_type, error.to_string())
        }
        running_errors.print_errs();
        panic!();
    }

    let instance_res = serde_json::from_value::<Instance>(instance_json.clone());
    instance_res.expect(
        format!(
            "{} should parse into rust",
            style_path(instance_path, "instance")
        )
        .as_str(),
    )
}
