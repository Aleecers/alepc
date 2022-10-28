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

use chrono::{offset::Local, DateTime};
use requestty::Answers;

use super::{
    helpers::{get_modifing_post, is_show_all_action},
    parse_bool, parse_str_vec, parse_string, slug_updater, tags_updater, Post, PostProperties,
};
use crate::{
    errors::{ApcError, ApcResult},
    utils::parse_str_date,
    CONFIG,
};
use std::{collections::HashMap, fmt::Debug, path::Path};

#[derive(Debug)]
pub struct Props {
    pub slug: String,
    pub title: String,
    pub desctiption: String,
    pub image_path: String,
    pub tags: Vec<String>,
    pub draft: bool,
    pub date: DateTime<Local>,
    pub modified_date: DateTime<Local>,
}

#[derive(Debug)]
pub struct ModifyStatus {
    /// Modify all properties
    pub all: bool,
    /// Modify date property
    pub date: bool,
    /// Modify draft property
    pub draft: bool,
}

impl ModifyStatus {
    pub fn new(all: bool, date: bool, draft: bool) -> Self {
        Self { all, date, draft }
    }
}

impl From<&Answers> for ModifyStatus {
    fn from(answers: &Answers) -> Self {
        let config = CONFIG.as_ref().unwrap();
        let modify_actions: Vec<&str> = answers
            .get("modify_action")
            .expect("Modify action should have actions with 'modify_action' name")
            .as_list_items()
            .expect("Modify action should be a select input")
            .iter()
            .map(|item| item.text.as_str())
            .collect();

        Self::new(
            modify_actions.contains(&config.modify_post_settings.show_all_question.as_str()),
            modify_actions.contains(
                &config
                    .modify_post_settings
                    .update_the_date_question
                    .as_str(),
            ),
            modify_actions.contains(
                &config
                    .modify_post_settings
                    .update_draft_status_question
                    .as_str(),
            ),
        )
    }
}

pub trait ExtractProp<'a> {
    type Output;
    type NameType: Debug;

    fn extract_prop(&self, prop_name: Self::NameType) -> ApcResult<Self::Output> {
        Err(ApcError::PostProperties(format!(
            "Can't extract {prop_name:?}"
        )))
    }

    fn extract_ref_prop(&'a self, prop_name: Self::NameType) -> ApcResult<&'a Self::Output> {
        Err(ApcError::PostProperties(format!(
            "Can't extract {prop_name:?}"
        )))
    }
}

impl<'a, UU: Debug> ExtractProp<'a> for HashMap<String, UU> {
    type NameType = &'a str;
    type Output = UU;

    #[logfn_inputs(Info)]
    #[logfn(Debug)]
    fn extract_ref_prop(&'a self, prop_name: Self::NameType) -> ApcResult<&'a Self::Output> {
        self.get(prop_name).ok_or_else(|| {
            ApcError::PostProperties(format!(
                "Post don't have `{prop_name}` properite, and it's should"
            ))
        })
    }
}

impl<'a> ExtractProp<'a> for Answers {
    type NameType = &'a str;
    type Output = String;

    #[logfn_inputs(Info)]
    #[logfn(Debug)]
    fn extract_prop(&self, prop_name: Self::NameType) -> ApcResult<Self::Output> {
        let config = CONFIG.as_ref().unwrap();
        if is_show_all_action(config)(self) {
            self.get(prop_name)
                .ok_or_else(|| {
                    ApcError::PostProperties(format!("Cannot find `{prop_name}` from answers"))
                })
                .map(|answer| {
                    answer.clone().try_into_string().unwrap_or_else(|err| {
                        err.as_bool()
                            .expect("if it's not string, it's bool")
                            .to_string()
                    })
                })
        } else {
            Err(ApcError::PostProperties(
                "The action it's not `show_all` action".to_owned(),
            ))
        }
    }
}

impl TryFrom<Post> for Props {
    type Error = ApcError;

    fn try_from(post: Post) -> Result<Self, Self::Error> {
        let config = CONFIG.as_ref().unwrap();
        Self::from_str(&post.str_properties(config), post.path(config))
    }
}

impl Props {
    /// Create new [`Props`] instance
    /// ### Arguments
    /// * `slug` - Slug of the post
    /// * `title` - Title of the post
    /// * `desctiption` - Description of the post
    /// * `image_path` - Image path of the post
    /// * `tags` - Tags of the post
    /// * `draft` - Draft status of the post
    /// * `date` - Date of the post
    #[logfn_inputs(Info)]
    #[logfn(Debug)]
    #[allow(clippy::too_many_arguments)]
    fn new(
        slug: String,
        title: String,
        desctiption: String,
        image_path: String,
        tags: Vec<String>,
        draft: bool,
        date: DateTime<Local>,
        modified_date: DateTime<Local>,
    ) -> Self {
        Self {
            slug,
            title,
            desctiption,
            image_path,
            tags,
            draft,
            date,
            modified_date,
        }
    }

    /// Try to create new [`Props`] instance.
    /// ### Errors
    /// - If the `image_site_path` doesn't starts with `config.image_site_path`
    /// - If the image doesn't exist
    /// ### Arguments
    /// - `slug` - The slug of the post
    /// - `title` - The title of the post
    /// - `desctiption` - The description of the post
    /// - `image_site_path` - The path of the image in the site
    /// - `tags` - The tags of the post
    /// - `draft` - The draft status of the post
    /// - `date` - The creation date of the post
    /// - `modified_date` - The last modified date of the post
    #[logfn_inputs(Info)]
    #[logfn(Debug)]
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        slug: String,
        title: String,
        desctiption: String,
        image_site_path: String,
        tags: Vec<String>,
        draft: bool,
        date: DateTime<Local>,
        modified_date: DateTime<Local>,
    ) -> ApcResult<Self> {
        let config = CONFIG.as_ref().unwrap();
        let image_path = image_site_path.replace(&config.images_site_path, &config.images_path);

        if !image_site_path.starts_with(&config.images_site_path) {
            return Err(ApcError::PostProperties(format!(
                "The post image doesn't start with {} (its path from config)",
                config.images_site_path
            )));
        }

        if !Path::new(&image_path).exists() {
            return Err(ApcError::Validation(format!(
                "The post image doesn't exist: {}",
                image_path
            )));
        }

        Ok(Self::new(
            slug.clone(),
            title,
            desctiption,
            image_path,
            tags,
            draft,
            date,
            modified_date,
        ))
    }

    /// Return modified props from answers
    /// ### Panics
    /// * If the action is not `show_all` action
    /// ### Errors
    /// * If image path doesn't exist
    /// ### Arguments
    /// * `answers` - Ref [`Answers`] from user
    pub fn modified_from_answers(answers: &Answers) -> ApcResult<Self> {
        let config = CONFIG.as_ref().unwrap();
        let status = ModifyStatus::from(answers);
        let post_path =
            get_modifing_post(config, answers).expect("The action should be a `Modify` action");
        if status.draft || status.date {
            let mut post = Post::from_file(config, &post_path)?;
            if status.draft {
                post.is_draft = !post.is_draft;
            };
            post.try_into()
        } else {
            // Means is `show_all` action

            // Get image path from answers
            let image_path = PostProperties::Image.str_from_answers(answers)?;
            // Check if image path exist
            if !Path::new(&image_path).exists() {
                return Err(ApcError::Validation(format!(
                    "The post image doesn't exist: {}",
                    image_path
                )));
            }

            Ok(Self::new(
                slug_updater(&PostProperties::Slug.str_from_answers(answers)?),
                PostProperties::Title.str_from_answers(answers)?,
                PostProperties::Description.str_from_answers(answers)?,
                image_path,
                tags_updater(
                    &PostProperties::Tags.str_from_answers(answers)?,
                    config.create_post_settings.separated_tags_by,
                ),
                parse_bool(&PostProperties::Draft.str_from_answers(answers)?)?,
                parse_str_date(
                    &PostProperties::Date.from_file(&post_path)?,
                    &config.date_format,
                )?,
                chrono::offset::Local::now(),
            ))
        }
    }

    /// Parse a props from str
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn from_str<P>(str_props: &str, post_path: P) -> ApcResult<Self>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        fn inner(props: &str, path: &Path) -> ApcResult<Props> {
            let mut hash_props: HashMap<String, String> = HashMap::new();
            let config = CONFIG.as_ref().unwrap();
            let keys = [
                "title",
                "layout",
                "image",
                "link",
                "date",
                "dateModified",
                "description",
                "draft",
                "tags",
            ];
            for line in props.lines() {
                let mut split_line = line.split(':');
                if let Some(key) = split_line.next() {
                    if keys.contains(&key) {
                        if let Some(value) = split_line.next() {
                            if !hash_props.contains_key(key) {
                                hash_props.insert(key.to_owned(), value.trim().to_owned());
                            } else {
                                // duplicate key
                                return Err(ApcError::PostProperties(format!(
                                    "'{key}' i'ts duplicate key in {}",
                                    path.display()
                                )));
                            }
                        } else {
                            // invalid line format
                            return Err(ApcError::PostProperties(format!(
                                "'{line}' invalid fomrat (key:value) in '{}'",
                                path.display()
                            )));
                        }
                    } else {
                        // invalid key
                        return Err(ApcError::PostProperties(format!(
                            "'{key}' i'ts invalid key in {}",
                            path.display()
                        )));
                    }
                } else {
                    // invalid line format
                    return Err(ApcError::PostProperties(format!(
                        "'{line}' invalid fomrat (key:value) in '{}'",
                        path.display()
                    )));
                }
            }

            Props::try_new(
                parse_string(hash_props.extract_ref_prop("link")?)?
                    .rsplit('/')
                    .next()
                    .ok_or_else(|| {
                        ApcError::PostProperties(format!(
                            "'{}' have a invalid post link",
                            path.display()
                        ))
                    })?
                    .to_owned(),
                parse_string(hash_props.extract_ref_prop("title")?)?,
                parse_string(hash_props.extract_ref_prop("description")?)?,
                parse_string(hash_props.extract_ref_prop("image")?)?,
                parse_str_vec(hash_props.extract_ref_prop("tags")?)?,
                parse_bool(hash_props.extract_ref_prop("draft")?)?,
                parse_str_date(
                    &parse_string(hash_props.extract_ref_prop("date")?)?,
                    &config.date_format,
                )?,
                parse_str_date(
                    &parse_string(hash_props.extract_ref_prop("dateModified")?)?,
                    &config.date_format,
                )?,
            )
        }
        inner(str_props, post_path.as_ref())
    }
}
