use std::path::Path;

use super::path::style_path;

pub fn write_schema<FromStruct: schemars::JsonSchema>(path: &Path) {
    let mut root_schema = schemars::schema_for!(FromStruct);
    root_schema.schema.extensions.insert(
        "additionalProperties".to_string(),
        serde_json::Value::Bool(false),
    );
    let contents = serde_json::to_string_pretty(&root_schema)
        .expect(format!("Should serialize schema for internal struct").as_str());
    let res = std::fs::write(path, contents);
    res.expect(format!("Should write JSON schema to {}", style_path(path, "path")).as_str());
}
