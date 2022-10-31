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

use crate::errors::{ApcError, ApcResult};
use directories::ProjectDirs;
use serde::Deserialize;
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

/// Select action configuration structure
#[derive(Deserialize, Debug, Educe)]
#[educe(Default)]
#[serde(default)]
pub struct SelectAction {
    /// The question of select action
    #[educe(Default = "What do you want to do ❓")]
    pub select_action_message: String,
    /// Create a new post choice in select
    #[educe(Default = "Create a new post ✍")]
    pub new_post_choice: String,
    /// Update an existing post choice
    #[educe(Default = "Update existing post 🖌️")]
    pub update_existing_post: String,
    /// Version choice
    #[educe(Default = "Alepc Version ⚙")]
    pub version_choice: String,
}

/// Inputs setting structure
#[derive(Deserialize, Debug, Educe)]
#[educe(Default)]
#[serde(default)]
pub struct CreatePostSettings {
    /// Minimum length of post title
    #[educe(Default = 7)]
    pub minimum_title_length: u8,
    /// Maximum length of post title
    #[educe(Default = 30)]
    pub maximum_title_length: u8,
    /// Ask for post title message
    #[educe(Default = "Title of post 📝")]
    pub title_message: String,

    /// Minimum length of post description
    #[educe(Default = 10)]
    pub minimum_description_length: u8,
    /// Maximum length of post description
    #[educe(Default = 255)]
    pub maximum_description_length: u8,
    /// Ask for post description message
    #[educe(Default = "Description of post 📝")]
    pub description_message: String,

    /// Minimum tags on post
    #[educe(Default = 1)]
    pub minimum_tags_count: u8,
    /// Maximum tags on post
    #[educe(Default = 3)]
    pub maximum_tags_count: u8,
    /// Ask for post tags message
    #[educe(Default = "Tags of post (separated by comma)")]
    pub tags_message: String,
    /// separated tags by
    #[educe(Default = ',')]
    pub separated_tags_by: char,
    /// Minimum single tag length
    #[educe(Default = 3)]
    pub minimum_single_tag_length: u8,
    /// Maximum single tag length
    #[educe(Default = 8)]
    pub maximum_single_tag_length: u8,

    /// Ask for post slug message
    #[educe(Default = "Slug of post")]
    pub slug_message: String,
    /// Minimum length of post slug
    #[educe(Default = 5)]
    pub minimum_slug_length: u8,
    /// Maximum length of post slug
    #[educe(Default = 20)]
    pub maximum_slug_length: u8,

    /// Ask for post image message
    #[educe(Default = "Image of post")]
    pub image_message: String,
}

#[derive(Deserialize, Debug, Educe)]
#[educe(Default)]
#[serde(default)]
pub struct ModifyPostSettings {
    /// The question of post name
    #[educe(Default = "What's the post you want to modify it (Write the slug)")]
    pub post_name_question: String,
    /// Choice modify action
    #[educe(Default = "What do you want to update?")]
    pub choice_action: String,
    /// Update date question
    #[educe(Default = "Update modified date")]
    pub update_the_date_question: String,
    /// Update draft status question ( Will add the currently status in the end)
    #[educe(Default = "Update draft status")]
    pub update_draft_status_question: String,
    /// Show all fields to update it question
    #[educe(Default = "Show all")]
    pub show_all_question: String,
    /// New post slug question (Wheen show_all)
    #[educe(Default = "New post slug")]
    pub new_post_slug: String,
    /// New post title question (Wheen show_all)
    #[educe(Default = "New post title")]
    pub new_post_title: String,
    /// New post descripation question (Wheen show_all)
    #[educe(Default = "New post description")]
    pub new_post_descrioption: String,
    /// New post image question (Wheen show_all)
    #[educe(Default = "New post image")]
    pub new_post_image: String,
    /// New post tags question (Wheen show_all)
    #[educe(Default = "New post tags")]
    pub new_post_tags: String,
    /// New post draft status question (Wheen show_all)
    #[educe(Default = "Do you want to change draft status?")]
    pub new_post_draft: String,
    /// Message to keep old value
    #[educe(Default = "Press enter to keep it 🤏")]
    pub keep_old_value_message: String,
}

/// Config structure for Alepc
#[derive(Deserialize, Debug, Educe)]
#[educe(Default)]
#[serde(default)]
pub struct Config {
    /// Path of posts
    #[educe(Default = "../Aleecers.github.io/src/pages/blog/")]
    pub posts_path: String,
    /// Path to images directory
    #[educe(Default = "../Aleecers.github.io/public/images/")]
    pub images_path: String,
    /// Path of blog in the site
    #[educe(Default = "/blog/")]
    pub blog_site_path: String,
    /// Path of images in the site
    #[educe(Default = "/images/")]
    pub images_site_path: String,
    /// Layout path of posts ( path start from `posts_path` )
    #[educe(Default = "../../layouts/blog.astro")]
    pub posts_layout: String,
    /// Repository url
    #[educe(Default = "https://github.com/aleecers/alepc")]
    pub repository_url: String,
    /// Date format
    #[educe(Default = "%Y/%m/%d")]
    pub date_format: String,
    /// Select action structure
    pub select_action: SelectAction,
    /// Creat post setting
    pub create_post_settings: CreatePostSettings,
    /// Modify post setting
    pub modify_post_settings: ModifyPostSettings,
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
}

/// Return config
#[logfn(Debug)]
#[logfn_inputs(Info)]
pub fn config() -> ApcResult<Config> {
    let config_path = ProjectDirs::from("", ORGANIZATION, APP_NAME)
        .map(|path| path.config_dir().join("config.ron"))
        .ok_or_else(|| ApcError::FileSystem("Can't get config path".to_string()))?;

    if config_path.exists() {
        match fs::read_to_string(config_path) {
            Ok(str_ron) => ron::from_str(&str_ron).map_err(|err| ApcError::ParseRon {
                code: err.code,
                position: err.position,
            }),
            Err(err) => Err(ApcError::FileSystem(err.to_string())),
        }
    } else {
        fs::write(config_path, "(\n    \n)")
            .map_err(|err| ApcError::FileSystem(err.to_string()))?;
        Ok(Config::default())
    }
}

/// Return [`Config`]
#[logfn(Debug)]
#[logfn_inputs(Info)]
pub fn get_config() -> ApcResult<Config> {
    config()?.configuration()
}
