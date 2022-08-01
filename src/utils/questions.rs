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

use crate::config::Config;
use crate::utils::validators::{
    file_path_validator, is_valid_length, is_valid_tags, join_on_key_validator,
    join_str_validators, length_validator, tags_validator,
};
use crate::utils::{full_path, slug_updater, tags_updater, to_post_path};
use requestty::{Answers, Question};

/// Returns the post title question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_title_question(config: &Config) -> Question {
    Question::input("post_title")
        .message(&config.input_settings.title_message)
        // Validate when write the title ( title will be red if `is_valid_length` return false )
        .validate_on_key(is_valid_length(
            config.input_settings.minimum_title_length,
            config.input_settings.maximum_title_length,
        ))
        // Validate when pressing Enter
        .validate(length_validator(
            "post title",
            config.input_settings.minimum_title_length,
            config.input_settings.maximum_title_length,
        ))
        .transform(|title, _, backend| write!(backend, "{}", title.trim()))
        // Run this question when user choice to create a new post
        .when(|answer: &Answers| {
            answer.get("action").unwrap().as_list_item().unwrap().text
                == config.select_action.new_post_choice
        })
        .build()
}

/// Returns the post description question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_description_question(config: &Config) -> Question {
    Question::input("post_description")
        .message(&config.input_settings.description_message)
        // Validate when write the description ( description will be red if `is_valid_length` return false )
        .validate_on_key(is_valid_length(
            config.input_settings.minimum_description_length,
            config.input_settings.maximum_description_length,
        ))
        // Validate when pressing Enter
        .validate(length_validator(
            "post description",
            config.input_settings.minimum_description_length,
            config.input_settings.maximum_description_length,
        ))
        .transform(|description, _, backend| write!(backend, "{}", description.trim()))
        // Run this question when user choice to create a new post
        .when(|answer: &Answers| {
            answer.get("action").unwrap().as_list_item().unwrap().text
                == config.select_action.new_post_choice
        })
        .build()
}

/// Returns the psst tags question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_tags_question(config: &Config) -> Question {
    Question::input("post_tags")
        .message(&config.input_settings.tags_message)
        .validate_on_key(is_valid_tags(
            config.input_settings.minimum_tags_count,
            config.input_settings.maximum_tags_count,
            config.input_settings.minimum_single_tag_length,
            config.input_settings.maximum_single_tag_length,
            config.input_settings.separated_tags_by,
        ))
        .validate(tags_validator(
            config.input_settings.minimum_tags_count,
            config.input_settings.maximum_tags_count,
            config.input_settings.minimum_single_tag_length,
            config.input_settings.maximum_single_tag_length,
            config.input_settings.separated_tags_by,
        ))
        .transform(|str_tags, _, backend| {
            write!(
                backend,
                "{}",
                tags_updater(str_tags, config.input_settings.separated_tags_by)
                    .join(&format!("{} ", config.input_settings.separated_tags_by))
            )
        })
        .when(|answer: &Answers| {
            answer.get("action").unwrap().as_list_item().unwrap().text
                == config.select_action.new_post_choice
        })
        .build()
}

/// Returns the post slug question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_slug_question(config: &Config) -> Question {
    Question::input("post_slug")
        .message(&config.input_settings.slug_message)
        // Validate when write the slug ( slug will be red if `is_valid_length` return false )
        .validate_on_key(join_on_key_validator(
            is_valid_length(
                config.input_settings.minimum_slug_length,
                config.input_settings.maximum_slug_length,
            ),
            |slug, answers| {
                file_path_validator(true)(&to_post_path(config, &slug_updater(slug)), answers)
                    .is_ok()
            },
        ))
        // Validate if slug already exists, and slug length
        .validate(join_str_validators(
            length_validator(
                "post description",
                config.input_settings.minimum_slug_length,
                config.input_settings.maximum_slug_length,
            ),
            |slug, answers: &Answers| {
                file_path_validator(true)(&to_post_path(config, slug), answers)
            },
        ))
        .transform(|slug, _, backend| write!(backend, "{}", slug_updater(slug)))
        // Run this question when user choice to create a new post
        .when(|answer: &Answers| {
            answer.get("action").unwrap().as_list_item().unwrap().text
                == config.select_action.new_post_choice
        })
        .build()
}

/// Returns the post image question
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn post_image_question(config: &Config) -> Question {
    Question::input("post_image")
        .message(&config.input_settings.image_message)
        .validate_on_key(|str_path, answers| file_path_validator(false)(str_path, answers).is_ok())
        .validate(file_path_validator(false))
        .transform(|str_path, _, backend| write!(backend, "{}", full_path(str_path)))
        // Run this question when user choice to create a new post
        .when(|answer: &Answers| {
            answer.get("action").unwrap().as_list_item().unwrap().text
                == config.select_action.new_post_choice
        })
        .build()
}

/// Return all properties questions of post
#[logfn(Debug)]
#[logfn_inputs(Info)]
pub fn post_properties(config: &Config) -> Vec<Question> {
    vec![
        post_title_question(config),
        post_description_question(config),
        post_tags_question(config),
        post_slug_question(config),
        post_image_question(config),
    ]
}
