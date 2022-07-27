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

use crate::errors::{ApcError, ApcResult};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
/// Select action configuration structure
pub struct SelectAction {
    /// The question of select action
    pub select_action_message: String,
    /// Create a new post choice in select
    pub new_post_choice: String,
    /// Update modified post date choice
    pub update_modified_post_date: String,
    /// Version choice
    pub version_choice: String,
}

#[derive(Deserialize, Debug)]
/// Inputs setting structure
pub struct InputSettings {
    /// Minimum length of post title
    pub minimum_title_length: u8,
    /// Maximum length of post title
    pub maximum_title_length: u8,
    /// Ask for post title message
    pub title_message: String,

    /// Minimum length of post description
    pub minimum_description_length: u8,
    /// Maximum length of post description
    pub maximum_description_length: u8,
    /// Ask for post description message
    pub description_message: String,

    /// Minimum tags on post
    pub minimum_tags_count: u8,
    /// Maximum tags on post
    pub maximum_tags_count: u8,
    /// Ask for post tags message
    pub tags_message: String,
    /// separated tags by
    pub separated_tags_by: char,
    /// Minimum single tag length
    pub minimum_single_tag_length: u8,
    /// Maximum single tag length
    pub maximum_single_tag_length: u8,

    /// Ask for post slug message
    pub slug_message: String,
    /// Minimum length of post slug
    pub minimum_slug_length: u8,
    /// Maximum length of post slug
    pub maximum_slug_length: u8,

    /// Ask for post image message
    pub image_message: String,
}

#[derive(Deserialize, Debug)]
/// Config structure for Alepc
pub struct Config {
    /// Path of posts
    pub posts_path: String,
    /// Path to images directory
    pub images_path: String,
    /// Path of blog in the site
    pub blog_site_path: String,
    /// Path of images in the site
    pub images_site_path: String,
    /// Layout path of posts ( path start from `posts_path` )
    pub posts_layout: String,
    /// Repository url
    pub repository_url: String,
    /// Date format
    pub date_format: String,
    /// Select action structure
    pub select_action: SelectAction,
    /// Inputs setting
    pub input_settings: InputSettings,
}

impl Config {
    /// Return the configuration if it's valid
    pub fn configuration(self) -> ApcResult<Config> {
        if !Path::new(&self.posts_path).exists() {
            return Err(ApcError::Validation(format!(
                "Invalid posts_path, '{}' does not exist. Update the path from 'src/config/config.ron'",
                self.posts_path
            )));
        }
        if !(self.blog_site_path.ends_with('/') && self.blog_site_path.starts_with('/')) {
            return Err(ApcError::Validation(format!(
                "Invalid blog_path, '{}' should start and end with a slash ('/')",
                self.blog_site_path,
            )));
        }
        Ok(self)
    }
}

/// Return config
pub fn config() -> ApcResult<ApcResult<Config>> {
    match fs::read_to_string("./src/config/config.ron") {
        Ok(str_ron) => Ok(ron::from_str(&str_ron).map_err(|err| ApcError::ParseRon {
            code: err.code,
            position: err.position,
        })),
        Err(err) => Err(ApcError::LoadConfig(err.to_string())),
    }
}

/// Return [`Config`] if there is no errors else [`None`]
pub fn get_config() -> Option<Config> {
    match config() {
        Ok(ron_result) => match ron_result {
            Ok(apc_config) => match apc_config.configuration() {
                Ok(config) => return Some(config),
                Err(err) => err.print(),
            },
            Err(err) => err.print(),
        },
        Err(err) => err.print(),
    }
    None
}
