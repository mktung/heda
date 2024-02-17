use std::path::Path;

use chrono::{DateTime, FixedOffset};

use crate::shared::path::style_path;

use super::errors::RunningErrors;

const BACKUP_TIME_FMT: &str = "%Y-%m-%dT%H:%M:%S";
const BACKUP_SPLIT_TOKEN: &str = "_backup_";

pub fn get_backup_path(path: &Path) -> Box<Path> {
    let file_stem = path
        .file_stem()
        .expect(format!("Should get file stem from {}", style_path(path, "backup")).as_str())
        .to_str()
        .expect(format!("{} file stem should be UTF-8", style_path(path, "backup")).as_str());

    let file_extension = path
        .extension()
        .expect(format!("{} file should have extension", style_path(path, "backup")).as_str());

    let now_date: DateTime<chrono::Local> = chrono::Local::now();
    let now_str = now_date.format(BACKUP_TIME_FMT).to_string();
    let backup_stem = format!("{file_stem}{BACKUP_SPLIT_TOKEN}{now_str}");

    // return with cur dir so it can be used with std::path::Path.join
    // without it, join will ignore the left if the right appears absolute
    Box::from(
        Path::new(std::path::Component::CurDir.as_os_str())
            .join(Path::new(backup_stem.as_str()).with_extension(file_extension)),
    )
}

pub fn get_backup_timestamp(path: &Path) -> DateTime<FixedOffset> {
    let file_stem = path
        .file_stem()
        .expect(format!("Should get file stem from {}", style_path(&path, "backup")).as_str());
    let time_str = file_stem
        .to_str()
        .expect(format!("{} file stem should be UTF-8", style_path(&path, "backup")).as_str())
        .split(BACKUP_SPLIT_TOKEN)
        .last()
        .expect(
            format!(
                "{} name should have format FILE_backup_TIMESTAMP.EXTENSION",
                style_path(&path, "backup")
            )
            .as_str(),
        );
    DateTime::parse_from_str(time_str, BACKUP_TIME_FMT).expect(
        format!(
            "{} name should contain timestamp with format {}",
            style_path(&path, "backup"),
            BACKUP_TIME_FMT
        )
        .as_str(),
    )
}

pub fn remove_old_backups(
    backups_path: &std::path::Path,
    remove_older_than: DateTime<FixedOffset>,
) {
    let backups = std::fs::read_dir(&backups_path).expect(
        format!(
            "Should read {}",
            style_path(&backups_path, "backups directory")
        )
        .as_str(),
    );

    for entry_res in backups {
        let entry = entry_res
            .expect(
                format!(
                    "Should read contents of {}",
                    style_path(&backups_path, "backups directory")
                )
                .as_str(),
            )
            .path();
        let timestamp = get_backup_timestamp(&entry);

        let mut running_errors = RunningErrors::new();
        if timestamp < remove_older_than {
            let rm_res = std::fs::remove_file(&entry);
            if let Err(rm_err) = rm_res {
                running_errors.add_err(
                    &rm_err.to_string(),
                    format!("Could not remove {}.", style_path(&entry, "backup")),
                );
            }
        }
    }
}
