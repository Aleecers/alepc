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

use std::path::Path;

use crate::{config::Config, utils::home_dir};
use requestty::{prompt::Backend, question::Completions, Answers};
use rust_search::SearchBuilder;

use super::{full_path, replace_tilde_with_home_dir, to_post_path, PostProperties};

/// Return true if the action is to create new post
#[logfn_inputs(Info)]
pub fn is_new_post(config: &'static Config) -> impl Fn(&Answers) -> bool {
    move |answers| {
        answers.get("action").unwrap().as_list_item().unwrap().text
            == config.select_action.new_post_choice
    }
}

/// Autocomplete for files
/// ### Arguments
/// * `dir` - Directory to search for files
/// * `ext` - File extension to search for'
/// * `file_name` - Return file name instead of full path
#[logfn_inputs(Info)]
pub fn autocomplete_files<'a>(
    dir: Option<&'a str>,
    ext: Option<&'a str>,
    file_name: bool,
) -> impl FnMut(String, &Answers) -> Completions<String> + 'a {
    move |prefix, _| {
        let prefix = replace_tilde_with_home_dir(&prefix);
        let search_location = dir.map(Into::into).unwrap_or_else(home_dir);
        let mut files = SearchBuilder::default()
            .location(search_location)
            .ignore_case();
        if let Some(ext) = ext {
            // Add extension to search, if exsits
            files = files.ext(ext);
        }
        // Build the search, and convert it to slice
        let files: Vec<_> = files.build().collect();
        let files = files.as_slice();

        if files.is_empty() {
            Completions::from([prefix])
        } else if file_name && ext.is_some() {
            // If file name is requested, and extension is provided
            // Remove the extension from the file name
            let ext = format!(".{}", ext.unwrap());
            Completions::from(
                files
                    .iter()
                    .map(|f| f.strip_suffix(&ext).unwrap_or(f).to_owned())
                    .map(|f| {
                        Path::new(&f)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned()
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            // There is no file name, and there is files
            Completions::from(files)
        }
    }
}

/// Return true if the action is `modify`
#[logfn_inputs(Info)]
pub fn is_modify_post(config: &'static Config) -> impl Fn(&Answers) -> bool {
    move |answers| {
        answers.get("action").unwrap().as_list_item().unwrap().text
            == config.select_action.update_existing_post
    }
}

/// Return true if the action is to create new post
#[logfn_inputs(Info)]
pub fn is_show_all_action(config: &'static Config) -> impl Fn(&Answers) -> bool {
    move |answers| {
        // All mean only show_all are selected
        // Check the validator of modify action (validators::modify_action)
        answers.get("modify_action").map_or_else(
            || false,
            |action| {
                action
                    .as_list_items()
                    .unwrap()
                    .iter()
                    .all(|item| item.text == config.modify_post_settings.show_all_question)
            },
        )
    }
}

/// Full path tranform, will panic if the path dose't exiest
pub fn full_path_transform() -> impl FnMut(&str, &Answers, &mut dyn Backend) -> std::io::Result<()>
{
    move |str_path, _, backend| {
        write!(
            backend,
            "{}",
            full_path(&replace_tilde_with_home_dir(str_path))
        )
    }
}

/// Return a tags transform
pub fn tags_transform(
    config: &'static Config,
) -> impl FnMut(&str, &Answers, &mut dyn Backend) -> std::io::Result<()> {
    move |str_tags, _, backend| {
        write!(
            backend,
            "{}",
            super::tags_updater(str_tags, config.create_post_settings.separated_tags_by)
                .join(&config.create_post_settings.separated_tags_by.to_string())
        )
    }
}

pub fn get_str_length(str_text: &str) -> usize {
    str_text.trim().chars().count()
}

/// Return a path of modifing post or None if the action not `modify`
pub fn get_modifing_post(config: &'static Config, answers: &Answers) -> Option<String> {
    is_modify_post(config)(answers).then(|| {
        let slug = answers.get("post_file").unwrap().as_string().unwrap();
        to_post_path(config, slug)
    })
}

/// Write a default value of propertie
pub fn default_value_transform(
    config: &'static Config,
    propertie: PostProperties,
) -> impl FnMut(&str, &Answers, &mut dyn Backend) -> Option<std::io::Result<()>> {
    move |value, answers, backend| {
        (value == config.modify_post_settings.keep_old_value_message
            && is_show_all_action(config)(answers))
        .then(|| {
            write!(
                backend,
                "{}",
                propertie
                    .from_file(
                        &get_modifing_post(config, answers)
                            .expect("Verified that action is show_all")
                    )
                    .unwrap_or_else(|_| panic!("Cannot get default value of `{propertie:?}`"))
            )
        })
    }
}

/// Join tow validator
pub fn join_str_validators<'a>(
    mut left: impl FnMut(&str, &Answers) -> Result<(), String> + 'a,
    mut right: impl FnMut(&str, &Answers) -> Result<(), String> + 'a,
) -> impl FnMut(&str, &Answers) -> Result<(), String> + 'a {
    move |str_value: &str, answers: &Answers| {
        left(str_value, answers).and_then(|_| right(str_value, answers))
    }
}

/// Join tow on key validator
pub fn join_on_key_validator<'a>(
    mut left: impl FnMut(&str, &Answers) -> bool + 'a,
    mut right: impl FnMut(&str, &Answers) -> bool + 'a,
) -> impl FnMut(&str, &Answers) -> bool + 'a {
    move |str_value, answers| left(str_value, answers) && right(str_value, answers)
}

/// Join transform, if left one return None will run right one
pub fn join_transform(
    mut left: impl FnMut(&str, &Answers, &mut dyn Backend) -> Option<std::io::Result<()>>,
    mut right: impl FnMut(&str, &Answers, &mut dyn Backend) -> std::io::Result<()>,
) -> impl FnMut(&str, &Answers, &mut dyn Backend) -> std::io::Result<()> {
    move |text, answers, backend| {
        left(text, answers, backend).unwrap_or_else(|| right(text, answers, backend))
    }
}
