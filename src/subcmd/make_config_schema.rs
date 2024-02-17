use std::path::Path;

use crate::shared::{config::ConfigJsonSchema, schema::write_schema};

pub fn make_config_schema(path: &Path) {
    write_schema::<ConfigJsonSchema>(path);
}
