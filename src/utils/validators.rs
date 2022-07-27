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

use crate::utils::tags_updater;
use requestty::Answers;
use std::path::Path;

pub mod helper {
    pub fn get_str_length(str_text: &str) -> usize {
        str_text.trim().chars().count()
    }
}

/// Length validator
pub fn length_validator(
    name: &str,
    minimum: u8,
    maximum: u8,
) -> impl FnMut(&str, &Answers) -> Result<(), String> + '_ {
    move |value: &str, _| {
        if helper::get_str_length(value) < (minimum as usize) {
            Err(format!(
                "The length of {name} must be greater than {}",
                minimum - 1
            ))
        } else if helper::get_str_length(value) > (maximum as usize) {
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
pub fn tags_validator(
    minimum_tags: u8,
    maximum_tags: u8,
    minimum_tag_length: u8,
    maximum_tag_length: u8,
    separated_by: char,
) -> impl FnMut(&str, &Answers) -> Result<(), String> {
    move |str_tags: &str, _| {
        if minimum_tags != 0 {
            let tags = tags_updater(str_tags, separated_by);
            if tags.len() < (minimum_tags as usize) {
                return Err(format!(
                    "The number of tags must be greater than {}",
                    minimum_tags - 1
                ));
            } else if tags.len() > (maximum_tags as usize) {
                return Err(format!(
                    "The number of tags must be less than {}",
                    maximum_tags + 1
                ));
            } else if let Some(invalid_tag) = tags
                .iter()
                .find(|tag| tag.chars().count() < (minimum_tag_length as usize))
            {
                return Err(format!(
                    "'{invalid_tag}' It's short, the minimum is {minimum_tag_length} characters"
                ));
            } else if let Some(invalid_tag) = tags
                .iter()
                .find(|tag| tag.chars().count() > (maximum_tag_length as usize))
            {
                return Err(format!(
                    "'{invalid_tag}' it's long, the maximum is {maximum_tag_length} characters"
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
pub fn is_valid_tags<'a>(
    minimum_tags: u8,
    maximum_tags: u8,
    minimum_tag_length: u8,
    maximum_tag_length: u8,
    separated_by: char,
) -> impl FnMut(&str, &Answers) -> bool + 'a {
    move |str_tags: &str, answers: &Answers| {
        tags_validator(
            minimum_tags,
            maximum_tags,
            minimum_tag_length,
            maximum_tag_length,
            separated_by,
        )(str_tags, answers)
        .is_ok()
    }
}

/// Return if the length of value is valid
pub fn is_valid_length<'a>(minimum: u8, maximum: u8) -> impl FnMut(&str, &Answers) -> bool + 'a {
    move |value: &str, answers: &Answers| {
        length_validator("_", minimum, maximum)(value, answers).is_ok()
    }
}

/// Join tow validator
pub fn join_str_validators<'a>(
    mut left: impl FnMut(&str, &Answers) -> Result<(), String> + 'a,
    mut right: impl FnMut(&str, &Answers) -> Result<(), String> + 'a,
) -> impl FnMut(&str, &Answers) -> Result<(), String> + 'a {
    move |str_value: &str, answers: &Answers| {
        if let Err(err) = left(str_value, answers) {
            Err(err)
        } else if let Err(err) = right(str_value, answers) {
            Err(err)
        } else {
            Ok(())
        }
    }
}

/// Join tow on key validator
pub fn join_on_key_validator<'a>(
    mut left: impl FnMut(&str, &Answers) -> bool + 'a,
    mut right: impl FnMut(&str, &Answers) -> bool + 'a,
) -> impl FnMut(&str, &Answers) -> bool + 'a {
    move |str_value, answers| left(str_value, answers) && right(str_value, answers)
}
