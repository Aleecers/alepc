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

use crate::config::Config;
use crate::utils::slug_updater;
use crate::utils::{helpers, validators};
use requestty::Question;

/// Returns the post title question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_title_question(config: &'static Config) -> Question {
    Question::input("post_title")
        .message(&config.create_post_settings.title_message)
        .validate_on_key(validators::is_valid_title_length(config))
        .validate(validators::title_length(config))
        .transform(|title, _, backend| write!(backend, "{}", title.trim()))
        .when(helpers::is_new_post(config))
        .build()
}

/// Returns the post description question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_description_question(config: &'static Config) -> Question {
    Question::input("post_description")
        .message(&config.create_post_settings.description_message)
        .validate_on_key(validators::is_valid_description_length(config))
        .validate(validators::description_length(config))
        .transform(|description, _, backend| write!(backend, "{}", description.trim()))
        .when(helpers::is_new_post(config))
        .build()
}

/// Returns the psst tags question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_tags_question(config: &'static Config) -> Question {
    Question::input("post_tags")
        .message(&config.create_post_settings.tags_message)
        .validate_on_key(validators::is_valid_tags(config))
        .validate(validators::tags_validator(config))
        .transform(helpers::tags_transform(config))
        .when(helpers::is_new_post(config))
        .build()
}

/// Returns the post slug question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_slug_question(config: &'static Config) -> Question {
    Question::input("post_slug")
        .message(&config.create_post_settings.slug_message)
        .validate_on_key(helpers::join_on_key_validator(
            validators::is_valid_slug_length(config),
            validators::is_valid_slug_path(config),
        ))
        .validate(helpers::join_str_validators(
            validators::slug_lenth(config),
            validators::slug_path_validator(config),
        ))
        .transform(|slug, _, backend| write!(backend, "{}", slug_updater(slug)))
        .when(helpers::is_new_post(config))
        .build()
}

/// Returns the post image question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_image_question(config: &'static Config) -> Question {
    Question::input("post_image")
        .message(&config.create_post_settings.image_message)
        .validate_on_key(validators::is_valid_path(false))
        .validate(validators::file_path_validator(false))
        .transform(helpers::full_path_transform())
        .when(helpers::is_new_post(config))
        .build()
}

/// Return all properties questions of post
#[logfn(Debug)]
#[logfn_inputs(Info)]
pub fn post_properties(config: &'static Config) -> Vec<Question> {
    vec![
        post_title_question(config),
        post_description_question(config),
        post_tags_question(config),
        post_slug_question(config),
        post_image_question(config),
    ]
}
