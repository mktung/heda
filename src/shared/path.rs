use std::path::Path;

use console::StyledObject;

pub fn style_path<'a>(path: &'a Path, default: &'a str) -> StyledObject<&'a str> {
    if let Some(path_str) = path.to_str() {
        return if path.is_file() {
            console::style(path_str).cyan()
        } else {
            console::style(path_str).blue()
        }
    }
    console::style(default).blue()
}
