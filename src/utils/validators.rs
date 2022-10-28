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

use super::{helpers, to_post_path};
use crate::{
    config::Config,
    utils::{replace_tilde_with_home_dir, tags_updater},
};
use requestty::Answers;
use std::path::PathBuf;

/// Length validator
pub fn length_validator(
    name: &str,
    minimum: u8,
    maximum: u8,
) -> impl FnMut(&str, &Answers) -> Result<(), String> + '_ {
    move |value: &str, _| {
        if helpers::get_str_length(value) < (minimum as usize) {
            Err(format!(
                "The length of {name} must be greater than {}",
                minimum - 1
            ))
        } else if helpers::get_str_length(value) > (maximum as usize) {
            Err(format!(
                "The length of {name} must be less than {}",
                maximum + 1
            ))
        } else {
            Ok(())
        }
    }
}

/// Validator of modify action selection
pub fn modify_action(
    config: &'static Config,
) -> impl FnMut(&[bool], &Answers) -> Result<(), String> {
    move |choices, _| {
        let trues = choices.iter().filter(|a| **a).count();
        if trues == 0 {
            Err("You must choose at least one cheese.".into())
        } else if trues >= 2 && *choices.iter().last().unwrap() {
            // Last choice it's to update all post properties
            Err(format!(
                "You cannot choice '{}' with other choices.",
                &config.modify_post_settings.show_all_question
            ))
        } else {
            Ok(())
        }
    }
}

/// Tags validator
pub fn tags_validator(config: &'static Config) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    move |str_tags: &str, _| {
        if config.create_post_settings.minimum_tags_count != 0 {
            let tags = tags_updater(str_tags, config.create_post_settings.separated_tags_by);
            if tags.len() < (config.create_post_settings.minimum_tags_count as usize) {
                return Err(format!(
                    "The number of tags must be greater than {}",
                    config.create_post_settings.minimum_tags_count - 1
                ));
            } else if tags.len() > (config.create_post_settings.maximum_tags_count as usize) {
                return Err(format!(
                    "The number of tags must be less than {}",
                    config.create_post_settings.maximum_tags_count + 1
                ));
            } else if let Some(invalid_tag) = tags.iter().find(|tag| {
                tag.chars().count()
                    < (config.create_post_settings.minimum_single_tag_length as usize)
            }) {
                return Err(format!(
                    "'{invalid_tag}' It's short, the minimum is {} characters",
                    config.create_post_settings.minimum_single_tag_length
                ));
            } else if let Some(invalid_tag) = tags.iter().find(|tag| {
                tag.chars().count()
                    > (config.create_post_settings.maximum_single_tag_length as usize)
            }) {
                return Err(format!(
                    "'{invalid_tag}' it's long, the maximum is {} characters",
                    config.create_post_settings.maximum_single_tag_length
                ));
            }
        }
        Ok(())
    }
}

/// Make `is_exist` true if you want error when the file are existing
pub fn file_path_validator(is_exist: bool) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    move |str_path, _| {
        // Replace '~' with the home directory
        let path = PathBuf::from(replace_tilde_with_home_dir(str_path));
        if path.exists() && !path.is_file() {
            return Err(format!("'{}' is not a file", path.display()));
        }
        if is_exist && path.exists() {
            return Err(format!("'{}' is already exists", path.display()));
        } else if !is_exist && !path.exists() {
            return Err(format!("'{}' is not exists", path.display()));
        }
        Ok(())
    }
}

/// Make `is_exist` true if you want error when the file are existing
pub fn is_valid_path(is_exist: bool) -> impl FnMut(&str, &Answers) -> bool {
    move |path, answers| file_path_validator(is_exist)(path, answers).is_ok()
}

/// Validate post by slug
pub fn validate_post_path_by_slug(
    config: &'static Config,
) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    helpers::join_str_validators(slug_lenth(config), move |slug, answers| {
        let post_path = to_post_path(config, slug);
        file_path_validator(false)(&post_path, answers)
            .map_err(|_| format!("No such post in this path `{post_path}`"))
    })
}

/// Is the slug is existing
pub fn is_exsiting_post_slug(config: &'static Config) -> impl FnMut(&str, &Answers) -> bool {
    move |slug, answers| validate_post_path_by_slug(config)(slug, answers).is_ok()
}

/// Return if the tags is valid tags
pub fn is_valid_tags(config: &'static Config) -> impl FnMut(&str, &Answers) -> bool {
    move |str_tags, answers| tags_validator(config)(str_tags, answers).is_ok()
}

/// Title validator
pub fn title_length(config: &'static Config) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    length_validator(
        "post title",
        config.create_post_settings.minimum_title_length,
        config.create_post_settings.maximum_title_length,
    )
}

/// Is valid title length (Use for on key validator)
pub fn is_valid_title_length(config: &'static Config) -> impl FnMut(&str, &Answers) -> bool {
    move |title, answers| title_length(config)(title, answers).is_ok()
}

/// Description validator
pub fn description_length(
    config: &'static Config,
) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    length_validator(
        "post title",
        config.create_post_settings.minimum_description_length,
        config.create_post_settings.maximum_description_length,
    )
}

/// Is valid description length (Use for on key validator)
pub fn is_valid_description_length(config: &'static Config) -> impl FnMut(&str, &Answers) -> bool {
    move |title, answers| description_length(config)(title, answers).is_ok()
}

/// Slug validator
pub fn slug_lenth(config: &'static Config) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    length_validator(
        "post slug",
        config.create_post_settings.minimum_slug_length,
        config.create_post_settings.maximum_slug_length,
    )
}

/// Is valid slug
pub fn is_valid_slug_length(config: &'static Config) -> impl FnMut(&str, &Answers) -> bool {
    move |slug, answers| slug_lenth(config)(slug, answers).is_ok()
}

/// Slug path validator
pub fn slug_path_validator(
    config: &'static Config,
) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    move |slug, answers| file_path_validator(true)(&to_post_path(config, slug), answers)
}

/// Is valid slug path
pub fn is_valid_slug_path(config: &'static Config) -> impl FnMut(&str, &Answers) -> bool {
    move |slug, answers| slug_path_validator(config)(slug, answers).is_ok()
}
