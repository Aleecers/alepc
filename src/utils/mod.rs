// Simple CLI to create/modify aleecers post template
//     Copyright (C) 2020-2022  TheAwiteb
//     https://github.com/aleecers/Alepc
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod helpers;
mod post;
pub mod properties;
pub mod questions;
pub mod validators;

use chrono::{DateTime, Local, NaiveDate};
pub use post::*;

use crate::config::Config;
use crate::errors::{ApcError, ApcResult};
use std::fs;
use std::path::{Path, PathBuf};

/// Update the slug to correct one
pub fn slug_updater(slug: &str) -> String {
    slug.trim().to_ascii_lowercase().replace([' ', '_'], "-")
}

/// Update the string tags to correct one
pub fn tags_updater(str_tags: &str, separated_by: char) -> Vec<String> {
    str_tags
        .split(separated_by)
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Return full path of existing one
/// Panic if path is not existing
#[logfn_inputs(Info)]
#[logfn(Debug)]
pub fn full_path(str_path: &str) -> String {
    fs::canonicalize(str_path)
        .unwrap()
        .to_str()
        .unwrap_or(str_path)
        .to_owned()
}

/// Convert a slug to markdown path
pub fn to_post_path(config: &Config, slug: &str) -> String {
    format!("{}{}.md", config.posts_path, slug_updater(slug))
}

/// Parse a str date
#[logfn_inputs(Info)]
#[logfn(Debug)]
pub fn parse_str_date(date: &str, date_format: &str) -> ApcResult<DateTime<Local>> {
    NaiveDate::parse_from_str(date, date_format)
        .map_err(|err| ApcError::PostProperties(err.to_string()))?
        .and_hms_opt(0, 0, 0)
        .expect("Failed to parse date")
        .and_local_timezone(Local)
        .single()
        .ok_or_else(|| {
            ApcError::PostProperties(format!(
                "Cannot parse `{date}` date with this `{date_format}` format"
            ))
        })
}

/// Return the home directory
/// Panic if home directory is not found
#[logfn_inputs(Info)]
#[logfn(Debug)]
pub fn home_dir() -> String {
    directories::UserDirs::new()
        .expect("Failed to get home directory")
        .home_dir()
        .to_str()
        .expect("Failed to get home directory")
        .to_owned()
}

/// Try to replace `~` with home dircetory
/// If `~` is not present, return the same string
#[logfn_inputs(Info)]
#[logfn(Debug)]
#[allow(clippy::manual_strip)]
fn replace_tilde_with_home_dir(path: &str) -> String {
    if path.starts_with('~') {
        format!("{}{}", home_dir(), &path[1..])
    } else {
        path.to_owned()
    }
}

/// Move new post headeer to images directory, return new post headeer image path
#[logfn_inputs(Info)]
#[logfn(Debug)]
pub fn copy_post_header(config: &Config, slug: &str, new_post_header: &str) -> ApcResult<String> {
    let full_new_header_path = full_path(&replace_tilde_with_home_dir(new_post_header));
    let extension = Path::new(&full_new_header_path)
        .extension()
        .map(|os_str| os_str.to_str().unwrap_or("png"))
        .unwrap_or("png");
    let slug = slug_updater(slug);
    let slug_dir = PathBuf::from(format!("{}{slug}/", config.images_path));
    let filename = format!("{slug}-header.{extension}");
    let to_path = format!("{}{slug}/{filename}", config.images_path,);
    if !slug_dir.exists() {
        fs::create_dir(slug_dir).map_err(|err| {
            log::error!("{:?}", err);
            ApcError::FileSystem(err.to_string())
        })?;
    }
    fs::copy(full_new_header_path, &to_path).map_err(|err| {
        log::error!("{:?}", err);
        ApcError::FileSystem(err.to_string())
    })?;
    Ok(to_path)
}

#[logfn_inputs(Info)]
#[logfn(Debug)]
pub fn parse_string(str_string: &str) -> ApcResult<String> {
    let str_string = str_string.trim();
    if str_string.starts_with('"') && str_string.ends_with('"') {
        Ok(str_string.trim_matches('"').chars().collect())
    } else {
        Err(ApcError::PostProperties(format!(
            "`{str_string:?}` invalid string propertie, should start and end with '\"'"
        )))
    }
}

#[logfn_inputs(Info)]
#[logfn(Debug)]
pub fn parse_str_vec(str_vec: &str) -> ApcResult<Vec<String>> {
    let str_vec = str_vec.trim();
    if str_vec.starts_with('[') && str_vec.ends_with(']') {
        str_vec
            .trim_matches(|c| "[]".contains(c))
            .split(',')
            .map(parse_string)
            .collect()
    } else {
        Err(ApcError::PostProperties(format!(
            "`{str_vec:?}` invalid list propertie, should start and end with '\"'"
        )))
    }
}

#[logfn_inputs(Info)]
#[logfn(Debug)]
pub fn parse_bool(str_bool: &str) -> ApcResult<bool> {
    match str_bool {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ApcError::PostProperties(format!(
            "`{str_bool}` invalid boolean"
        ))),
    }
}
