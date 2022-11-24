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

use requestty::{Question, Separator};

use crate::config::Config;
use crate::utils::{helpers, slug_updater, to_post_path, validators, PostProperties};

/// Question for choice a post to modify it
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn ask_for_post(config: &'static Config) -> Question {
    Question::input("post_file")
        .message(&config.modify_post_settings.post_name_question)
        .validate_on_key(validators::is_exsiting_post_slug(config))
        .validate(validators::validate_post_path_by_slug(config))
        .transform(|slug, _, backend| write!(backend, "{}", to_post_path(config, slug)))
        .auto_complete(helpers::autocomplete_files(
            Some(&config.posts_path),
            Some("md"),
            true,
        ))
        .when(helpers::is_modify_post(config))
        .build()
}

/// Choices to update the modified date
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn modify_action_choices(config: &'static Config) -> Question {
    Question::multi_select("modify_action")
        .message(&config.modify_post_settings.choice_action)
        .choice_with_default(&config.modify_post_settings.update_the_date_question, true)
        .choices(vec![
            config
                .modify_post_settings
                .update_draft_status_question
                .as_str()
                .into(),
            Separator("== OR ==".into()),
            config
                .modify_post_settings
                .show_all_question
                .as_str()
                .into(),
        ])
        .validate(validators::modify_action(config))
        .when(helpers::is_modify_post(config))
        .build()
}

/// Ask for new slug (Show all action)
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn new_slug(config: &'static Config) -> Question {
    Question::input("new_post_slug")
        .message(&config.modify_post_settings.new_post_slug)
        .default(&config.modify_post_settings.keep_old_value_message)
        .validate_on_key(helpers::join_on_key_validator(
            validators::is_valid_slug_length(config),
            validators::is_valid_slug_path(config),
        ))
        .validate(helpers::join_str_validators(
            validators::slug_lenth(config),
            validators::slug_path_validator(config),
        ))
        .transform(helpers::join_transform(
            helpers::default_value_transform(config, PostProperties::Slug),
            |slug, _, backend| write!(backend, "{}", slug_updater(slug)),
        ))
        .when(helpers::is_show_all_action(config))
        .build()
}

/// Ask for new title (Show all action)
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn new_title(config: &'static Config) -> Question {
    Question::input("new_post_title")
        .message(&config.modify_post_settings.new_post_title)
        .default(&config.modify_post_settings.keep_old_value_message)
        .validate_on_key(validators::is_valid_title_length(config))
        .validate(validators::title_length(config))
        .transform(helpers::join_transform(
            helpers::default_value_transform(config, PostProperties::Title),
            |title, _, backend| write!(backend, "{}", title.trim()),
        ))
        .when(helpers::is_show_all_action(config))
        .build()
}

/// Ask for new description (Show all action)
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn new_description(config: &'static Config) -> Question {
    Question::input("new_post_description")
        .message(&config.modify_post_settings.new_post_descrioption)
        .default(&config.modify_post_settings.keep_old_value_message)
        .validate_on_key(validators::is_valid_description_length(config))
        .validate(validators::description_length(config))
        .transform(helpers::join_transform(
            helpers::default_value_transform(config, PostProperties::Description),
            |description, _, backend| write!(backend, "{}", description.trim()),
        ))
        .when(helpers::is_show_all_action(config))
        .build()
}

/// Ask for new image (Show all action)
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn new_image(config: &'static Config) -> Question {
    Question::input("new_post_image")
        .message(&config.modify_post_settings.new_post_image)
        .default(&config.modify_post_settings.keep_old_value_message)
        .validate_on_key(validators::is_valid_path(false))
        .validate(validators::file_path_validator(false))
        .transform(helpers::join_transform(
            helpers::default_value_transform(config, PostProperties::Image),
            helpers::full_path_transform(),
        ))
        .auto_complete(helpers::autocomplete_files(None, None, false))
        .when(helpers::is_show_all_action(config))
        .build()
}

/// Ask for new tags (Show all action)
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn new_tags(config: &'static Config) -> Question {
    Question::input("new_post_tags")
        .message(&config.modify_post_settings.new_post_tags)
        .default(&config.modify_post_settings.keep_old_value_message)
        .validate_on_key(validators::is_valid_tags(config))
        .validate(validators::tags_validator(config))
        .transform(helpers::join_transform(
            helpers::default_value_transform(config, PostProperties::Tags),
            helpers::tags_transform(config),
        ))
        .when(helpers::is_show_all_action(config))
        .build()
}

/// Ask for new draft stutus (Show all action)
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn new_draft(config: &'static Config) -> Question {
    Question::confirm("new_post_draft")
        .message(&config.modify_post_settings.new_post_draft)
        .when(helpers::is_show_all_action(config))
        .build()
}

/// Return all properties questions of modify post
#[logfn(Debug)]
#[logfn_inputs(Info)]
pub fn modify_post_properties(config: &'static Config) -> Vec<Question> {
    vec![
        ask_for_post(config),
        modify_action_choices(config),
        new_slug(config),
        new_title(config),
        new_description(config),
        new_image(config),
        new_tags(config),
        new_draft(config),
    ]
}
