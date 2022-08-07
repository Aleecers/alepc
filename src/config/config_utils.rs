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
use directories::ProjectDirs;
use ron::ser as ron_ser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const ORGANIZATION: &str = "Aleecers";
pub const APP_NAME: &str = "alepc";

/// Make if statements if (`bool_expr`) return `true` will return `Err(ApcError::Validation(`error_message`))`
macro_rules! validation_check {
    ($bool_expr: expr, $error_message: expr) => {
        if $bool_expr {
            return Err(ApcError::Validation($error_message.to_string()));
        }
    };
    ($bool_expr: expr, $error_message: expr, $($bool_exprs: expr, $error_messages: expr),+) => {
        validation_check!($bool_expr, $error_message);
        validation_check!($($bool_exprs, $error_messages),+)
    };
}

macro_rules! validate_configuration_path {
    ($path: expr, $path_name: ident, $config_message: expr, $check_is_dir: expr) => {
        validation_check!(
            !Path::new($path).exists(),
            format!(
                "Invalid `{p_n}` '{p}' does not exist. Update `{p_n}` from config file{c}",
                p_n = stringify!($path_name),
                p = $path,
                c = $config_message,
            )
        );

        validation_check!(
            if $check_is_dir {
                !Path::new($path).is_dir()
            } else {
                !Path::new($path).is_file()
            },
            format!(
                "Invalid `{p_n}` `{p}` it's not {path_type}{c}",
                p_n = stringify!($path_name),
                p = $path,
                c = $config_message,
                path_type = if $check_is_dir { "directory" } else { "file" }
            )
        )
    };
}

macro_rules! validate_configuration_slashes {
    ($value: expr, $name: ident, $config_message: expr) => {
        let name = stringify!($name);
        let config_message = $config_message;
        let value = $value;
        validation_check!(
            !$value.ends_with('/'),
            format!("Invalid `{name}` '{value}' must ends with a slash {config_message}",),
            !$value.starts_with('/'),
            format!("Invalid `{name}` '{value}' must starts with a slash {config_message}",)
        )
    };
}

#[derive(Deserialize, Serialize, Debug, Educe)]
#[educe(Default)]
/// Select action configuration structure
pub struct SelectAction {
    #[educe(Default = "What do you want to do ❓")]
    /// The question of select action
    pub select_action_message: String,
    #[educe(Default = "Create a new post ✍")]
    /// Create a new post choice in select
    pub new_post_choice: String,
    #[educe(Default = "Update modified date 📅")]
    /// Update modified post date choice
    pub update_modified_post_date: String,
    #[educe(Default = "Alepc Version ⚙")]
    /// Version choice
    pub version_choice: String,
}

#[derive(Deserialize, Serialize, Debug, Educe)]
#[educe(Default)]
/// Inputs setting structure
pub struct InputSettings {
    #[educe(Default = 7)]
    /// Minimum length of post title
    pub minimum_title_length: u8,
    #[educe(Default = 30)]
    /// Maximum length of post title
    pub maximum_title_length: u8,
    #[educe(Default = "Title of post 📝")]
    /// Ask for post title message
    pub title_message: String,

    #[educe(Default = 10)]
    /// Minimum length of post description
    pub minimum_description_length: u8,
    #[educe(Default = 255)]
    /// Maximum length of post description
    pub maximum_description_length: u8,
    #[educe(Default = "Description of post 📝")]
    /// Ask for post description message
    pub description_message: String,

    #[educe(Default = 1)]
    /// Minimum tags on post
    pub minimum_tags_count: u8,
    #[educe(Default = 3)]
    /// Maximum tags on post
    pub maximum_tags_count: u8,
    #[educe(Default = "Tags of post (separated by comma)")]
    /// Ask for post tags message
    pub tags_message: String,
    #[educe(Default = ',')]
    /// separated tags by
    pub separated_tags_by: char,
    #[educe(Default = 3)]
    /// Minimum single tag length
    pub minimum_single_tag_length: u8,
    #[educe(Default = 8)]
    /// Maximum single tag length
    pub maximum_single_tag_length: u8,

    #[educe(Default = "Slug of post")]
    /// Ask for post slug message
    pub slug_message: String,
    #[educe(Default = 5)]
    /// Minimum length of post slug
    pub minimum_slug_length: u8,
    #[educe(Default = 20)]
    /// Maximum length of post slug
    pub maximum_slug_length: u8,

    #[educe(Default = "Image of post")]
    /// Ask for post image message
    pub image_message: String,
}

#[derive(Deserialize, Serialize, Debug, Educe)]
#[educe(Default)]
/// Config structure for Alepc
pub struct Config {
    #[educe(Default = "../Aleecers.github.io/src/pages/blog/")]
    /// Path of posts
    pub posts_path: String,
    #[educe(Default = "../Aleecers.github.io/public/images/")]
    /// Path to images directory
    pub images_path: String,
    #[educe(Default = "/blog/")]
    /// Path of blog in the site
    pub blog_site_path: String,
    #[educe(Default = "/images/")]
    /// Path of images in the site
    pub images_site_path: String,
    #[educe(Default = "../../layouts/blog.astro")]
    /// Layout path of posts ( path start from `posts_path` )
    pub posts_layout: String,
    #[educe(Default = "https://github.com/aleecers/alepc")]
    /// Repository url
    pub repository_url: String,
    #[educe(Default = "%Y/%m/%d")]
    /// Date format
    pub date_format: String,
    /// Select action structure
    pub select_action: SelectAction,
    /// Inputs setting
    pub input_settings: InputSettings,
}

impl Config {
    /// Return the configuration if it's valid
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn configuration(self) -> ApcResult<Self> {
        let config_issue = "\n\tsee: <https://github.com/Aleecers/alepc/issues/2>";
        validate_configuration_path!(&self.posts_path, posts_path, config_issue, true);
        validate_configuration_path!(&self.images_path, images_path, config_issue, true);
        validate_configuration_path!(
            Path::new(&self.posts_path)
                .join(&self.posts_layout)
                .to_str()
                .unwrap(),
            posts_layout,
            config_issue,
            false
        );
        validate_configuration_slashes!(&self.blog_site_path, blog_site_path, config_issue);
        validate_configuration_slashes!(&self.images_site_path, images_site_path, config_issue);
        Ok(self)
    }
    /// Write configuration file in config directory
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn write(self, config_path: &Path) -> ApcResult<Self> {
        if !config_path.parent().unwrap().exists() {
            fs::create_dir_all(config_path.parent().unwrap()).map_err(|err| {
                log::error!("{:?}", err);
                ApcError::FileSystem(err.to_string())
            })?
        }
        fs::write(
            config_path,
            ron_ser::to_string_pretty(&self, ron_ser::PrettyConfig::default()).map_err(|err| {
                ApcError::ParseRon {
                    code: err.code,
                    position: err.position,
                }
            })?,
        )
        .map_err(|err| {
            log::error!("{:?}", err);
            ApcError::FileSystem(err.to_string())
        })?;
        Ok(self)
    }
}

/// Return config
#[logfn(Debug)]
#[logfn_inputs(Info)]
pub fn config() -> ApcResult<ApcResult<Config>> {
    let config_path = ProjectDirs::from("", ORGANIZATION, APP_NAME)
        .map(|path| path.config_dir().join("config.ron"));
    if let Some(config_path) = config_path {
        if config_path.exists() {
            match fs::read_to_string(config_path) {
                Ok(str_ron) => Ok(ron::from_str(&str_ron).map_err(|err| ApcError::ParseRon {
                    code: err.code,
                    position: err.position,
                })),
                Err(err) => Err(ApcError::FileSystem(err.to_string())),
            }
        } else {
            Ok(Config::default().write(&config_path))
        }
    } else {
        Ok(Ok(Config::default()))
    }
}

/// Return [`Config`]
#[logfn(Debug)]
#[logfn_inputs(Info)]
pub fn get_config() -> ApcResult<Config> {
    config()??.configuration()
}
