// Simple CLI to create aleecers post template
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

mod creator;
pub mod helpers;
pub mod questions;
pub mod validators;

pub use creator::*;
use std::ffi::OsStr;

use crate::config::Config;
use crate::errors::{ApcError, ApcResult};
use std::fs;
use std::path::Path;

/// Update the slug to correct one
pub fn slug_updater(slug: &str) -> String {
    slug.trim()
        .to_ascii_lowercase()
        .replace(' ', "-")
        .replace('_', "-")
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
pub fn full_path(str_path: &str) -> String {
    fs::canonicalize(str_path)
        .unwrap()
        .to_str()
        .unwrap_or(str_path)
        .to_owned()
}

/// Convert a slug to markdown path
pub fn to_post_path(config: &Config, slug: &str) -> String {
    format!("{}/{}.md", config.posts_path, slug_updater(slug))
}

/// Move image to images directory, return image name
pub fn copy_image(config: &Config, slug: &str, image_path: &str) -> ApcResult<String> {
    let full_image_path = full_path(image_path);
    let extension = Path::new(&full_image_path)
        .extension()
        .unwrap_or_else(|| OsStr::new("png"))
        .to_str()
        .unwrap_or("png");
    let slug = slug_updater(slug);
    let filename = format!("{slug}-header.{extension}");
    let to_path = format!("{}{slug}/{filename}", config.images_path,);
    fs::create_dir(format!("{}{slug}/", config.images_path)).map_err(|err| {
        log::error!("{:?}", err);
        ApcError::FileSystem(err.to_string())
    })?;
    fs::copy(full_image_path, to_path).map_err(|err| {
        log::error!("{:?}", err);
        ApcError::FileSystem(err.to_string())
    })?;
    Ok(filename)
}
