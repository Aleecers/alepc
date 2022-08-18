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

use super::helpers;
use crate::{config::Config, utils::tags_updater};
use requestty::Answers;
use std::path::Path;

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

/// Tags validator
pub fn tags_validator(config: &'static Config) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    move |str_tags: &str, _| {
        if config.input_settings.minimum_tags_count != 0 {
            let tags = tags_updater(str_tags, config.input_settings.separated_tags_by);
            if tags.len() < (config.input_settings.minimum_tags_count as usize) {
                return Err(format!(
                    "The number of tags must be greater than {}",
                    config.input_settings.minimum_tags_count - 1
                ));
            } else if tags.len() > (config.input_settings.maximum_tags_count as usize) {
                return Err(format!(
                    "The number of tags must be less than {}",
                    config.input_settings.maximum_tags_count + 1
                ));
            } else if let Some(invalid_tag) = tags.iter().find(|tag| {
                tag.chars().count() < (config.input_settings.minimum_single_tag_length as usize)
            }) {
                return Err(format!(
                    "'{invalid_tag}' It's short, the minimum is {} characters",
                    config.input_settings.minimum_single_tag_length
                ));
            } else if let Some(invalid_tag) = tags.iter().find(|tag| {
                tag.chars().count() > (config.input_settings.maximum_single_tag_length as usize)
            }) {
                return Err(format!(
                    "'{invalid_tag}' it's long, the maximum is {} characters",
                    config.input_settings.maximum_single_tag_length
                ));
            }
        }
        Ok(())
    }
}

/// File path validator
pub fn file_path_validator<'a>(
    is_exist: bool,
) -> impl FnMut(&str, &Answers) -> Result<(), String> + 'a {
    move |str_path, _| {
        let path = Path::new(str_path);
        if path.exists() && !path.is_file() {
            return Err(format!("'{str_path}' is not a file"));
        }
        if is_exist && path.exists() {
            return Err(format!(
                "'{}' is already exists",
                path.file_name().unwrap().to_str().unwrap_or(str_path)
            ));
        } else if !is_exist && !path.exists() {
            return Err(format!("No such file named '{str_path}'"));
        }
        Ok(())
    }
}

/// Return if the tags is valid tags
pub fn is_valid_tags<'a>(config: &'static Config) -> impl FnMut(&str, &Answers) -> bool + 'a {
    move |str_tags, answers| tags_validator(config)(str_tags, answers).is_ok()
}

/// Return if the length of value is valid
pub fn is_valid_length<'a>(minimum: u8, maximum: u8) -> impl FnMut(&str, &Answers) -> bool + 'a {
    move |value: &str, answers: &Answers| {
        length_validator("_", minimum, maximum)(value, answers).is_ok()
    }
}
