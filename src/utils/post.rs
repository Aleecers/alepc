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

use crate::app::Action;
use crate::config::Config;
use crate::errors::{ApcError, ApcResult};
use crate::utils::{properties, slug_updater};
use crate::{utils, CONFIG};
use chrono::prelude::*;
use requestty::Answers;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use super::helpers::get_modifing_post;
use super::properties::{ExtractProp, ModifyStatus, Props};
use super::{copy_post_header, full_path, parse_bool, replace_tilde_with_home_dir};

#[derive(Debug, Clone, Copy)]
pub enum PostProperties {
    Slug,
    Title,
    Description,
    Image,
    Tags,
    Draft,
    Date,
}

impl PostProperties {
    /// Return a value of propertie from a post
    #[logfn_inputs(Info)]
    #[logfn(Debug)]
    pub fn from_file(&self, post_path: &str) -> ApcResult<String> {
        let config = CONFIG.as_ref().expect("Cannot get the config");
        let post = Post::from_file(config, post_path)?;
        match self {
            Self::Slug => Ok(post.slug),
            Self::Title => Ok(post.title),
            Self::Description => Ok(post.description),
            Self::Image => Ok(post.image_path),
            Self::Tags => Ok(post
                .tags
                .join(&config.create_post_settings.separated_tags_by.to_string())),
            Self::Draft => Ok(post.is_draft.to_string()),
            Self::Date => Ok(post.date.format(&config.date_format).to_string()),
        }
    }

    /// Return a value of prop from a answers
    pub fn str_from_answers(&self, answers: &Answers) -> ApcResult<String> {
        let config = CONFIG.as_ref().unwrap();
        // FIXME: fix this shit, '.unwrap_or(<question>)' ????????
        match self {
            Self::Slug => answers.extract_prop("new_post_slug").map(|slug| {
                if slug == config.modify_post_settings.keep_old_value_message {
                    self.from_file(
                        &get_modifing_post(config, answers)
                            .expect("The action it's not `show_all`"),
                    )
                    .unwrap_or(slug)
                } else {
                    slug
                }
            }),
            Self::Title => answers.extract_prop("new_post_title").map(|title| {
                if title == config.modify_post_settings.keep_old_value_message {
                    self.from_file(
                        &get_modifing_post(config, answers)
                            .expect("The action it's not `show_all`"),
                    )
                    .unwrap_or(title)
                } else {
                    title
                }
            }),
            Self::Description => answers
                .extract_prop("new_post_description")
                .map(|description| {
                    if description == config.modify_post_settings.keep_old_value_message {
                        self.from_file(
                            &get_modifing_post(config, answers)
                                .expect("The action it's not `show_all`"),
                        )
                        .unwrap_or(description)
                    } else {
                        description
                    }
                }),
            Self::Image => answers.extract_prop("new_post_image").map(|image| {
                if image == config.modify_post_settings.keep_old_value_message {
                    self.from_file(
                        &get_modifing_post(config, answers)
                            .expect("The action it's not `show_all`"),
                    )
                    .unwrap_or(image)
                } else {
                    full_path(&replace_tilde_with_home_dir(&image))
                }
            }),
            Self::Tags => answers.extract_prop("new_post_tags").map(|tags| {
                if tags == config.modify_post_settings.keep_old_value_message {
                    self.from_file(
                        &get_modifing_post(config, answers)
                            .expect("The action it's not `show_all`"),
                    )
                    .unwrap_or(tags)
                } else {
                    tags
                }
            }),
            Self::Draft => answers
                .extract_prop("new_post_draft")
                .map(|v| parse_bool(&v).map(|v| (!v).to_string()))?,
            Self::Date => unreachable!("Cannot get date from answers"),
        }
    }
}

/// Struct containing the post information
#[derive(Debug, Educe)]
#[educe(Default)]
pub struct Post {
    pub title: String,
    pub layout: String,
    pub slug: String,
    pub is_draft: bool,
    pub description: String,
    pub tags: Vec<String>,
    pub image_path: String,
    #[educe(Default(expression = "chrono::offset::Local::now()"))]
    pub date: DateTime<Local>,
    #[educe(Default(expression = "chrono::offset::Local::now()"))]
    pub date_modified: DateTime<Local>,
    pub link: String,
}

impl Post {
    /// Retrun create action with a post from answers
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn create_action(config: &'static Config, answers: &Answers) -> ApcResult<Action> {
        // FIXME: Replace unwrap with expect
        let slug = answers
            .get("post_slug")
            .expect("Create post action should have a slug with 'post_slug' name")
            .as_string()
            .expect("'post_slug' should be string");
        Ok(Action::Create(Post::try_new(
            config,
            answers
                .get("post_title")
                .unwrap()
                .as_string()
                .unwrap()
                .trim()
                .to_owned(),
            slug.to_owned(),
            true,
            answers
                .get("post_description")
                .unwrap()
                .as_string()
                .unwrap()
                .trim()
                .to_owned(),
            utils::tags_updater(
                answers.get("post_tags").unwrap().as_string().unwrap(),
                config.create_post_settings.separated_tags_by,
            ),
            utils::copy_post_header(
                config,
                slug,
                answers.get("post_image").unwrap().as_string().unwrap(),
            )?,
            chrono::offset::Local::now(),
            chrono::offset::Local::now(),
        )?))
    }

    /// Return a modify action with a new post from answers
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn modify_action(config: &'static Config, answers: &Answers) -> ApcResult<Action> {
        let old_path =
            get_modifing_post(config, answers).expect("The action should be `Modify` action");
        let status = ModifyStatus::from(answers);
        let mut post = Post::from_file(config, &old_path)?;
        let new_props = Props::modified_from_answers(answers)?;

        if post.slug != new_props.slug {
            // Check if the new slug is already existing or not
            if fs::read_dir(&config.images_path)
                .map_err(|err| ApcError::FileSystem(format!("'{}' {}", config.images_path, err)))?
                .collect::<Result<Vec<fs::DirEntry>, std::io::Error>>()
                .map_err(|err| {
                    ApcError::FileSystem(format!("File in `{}`: {}", config.images_path, err))
                })?
                .iter()
                .all(|entry| entry.file_name() == OsString::from(&new_props.slug))
            {
                return Err(ApcError::PostProperties(
                    "The new slug is already existing".to_owned(),
                ));
            }
        }

        post.title = new_props.title;
        post.description = new_props.desctiption;
        post.is_draft = new_props.draft;
        post.tags = new_props.tags;
        if status.all || status.date {
            post.date_modified = chrono::offset::Local::now();
        }
        Ok(Action::Modify {
            new_post: post,
            new_slug: new_props.slug,
            new_image_path: new_props.image_path,
        })
    }

    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        config: &'static Config,
        title: String,
        slug: String,
        is_draft: bool,
        description: String,
        tags: Vec<String>,
        image_path: String,
        date: DateTime<Local>,
        date_modified: DateTime<Local>,
    ) -> ApcResult<Self> {
        let modified_slug = slug_updater(&slug);

        if !Path::new(&image_path).exists() {
            return Err(ApcError::Validation(format!(
                "The image `{}` doesn't exist",
                image_path
            )));
        }

        Ok(Self {
            title,
            slug,
            is_draft,
            description,
            tags,
            layout: config.posts_layout.clone(),
            image_path,
            date,
            date_modified,
            link: format!("{}{}", config.blog_site_path, modified_slug),
        })
    }

    /// Update post slug.
    /// This method will update `slug`, `link`, `post_file_name`
    #[logfn_inputs(Info)]
    #[logfn(Debug)]
    pub fn update_slug(&mut self, config: &'static Config, new_slug: &str) -> ApcResult<()> {
        let new_slug = slug_updater(new_slug);
        if self.slug != new_slug {
            fs::rename(
                format!("{}{}.md", config.posts_path, self.slug),
                format!("{}{}.md", config.posts_path, new_slug),
            )
            .map_err(|err| ApcError::FileSystem(err.to_string()))?;
            self.slug = new_slug;
            self.link = format!("{}{}", config.blog_site_path, self.slug);
        }
        Ok(())
    }

    /// Update post image.
    /// This method will update `image_path` and remove old images folder to create new one with new slug.
    /// ## Notes:
    /// - This method should be called after update the slug.
    #[logfn_inputs(Info)]
    #[logfn(Debug)]
    pub fn update_images(&mut self, config: &'static Config, old_slug: &str) -> ApcResult<()> {
        if self.slug != old_slug {
            let old_image_path = Path::new(&self.image_path);
            let new_image_path = copy_post_header(config, &self.slug, &self.image_path)?;

            for image in fs::read_dir(
                old_image_path
                    .parent()
                    .expect("The parent is slug directory"),
            )
            .map_err(|err| ApcError::FileSystem(format!("`{}` {err}", old_image_path.display())))?
            {
                let image = image.map_err(|err| ApcError::FileSystem(err.to_string()))?;
                let image_path = image.path();
                if image_path.is_file() {
                    // This mean the image its not post header
                    if image_path != old_image_path {
                        fs::rename(
                            &image_path,
                            format!(
                                "{}{}/{}",
                                config.images_path,
                                self.slug,
                                image
                                    .path()
                                    .file_name()
                                    .expect("Was check is file")
                                    .to_str()
                                    .ok_or_else(|| ApcError::FileSystem(format!(
                                        "Invalid image name: {}",
                                        image_path.display()
                                    )))?
                            ),
                        )
                        .map_err(|err| ApcError::FileSystem(err.to_string()))?
                    }
                } else {
                    return Err(ApcError::Other(format!(
                        "Images directory cannot contain directory: `{}`",
                        image_path.display()
                    )));
                }
            }
            // Remove the old images directory
            fs::remove_dir_all(old_image_path.parent().ok_or_else(|| {
                ApcError::FileSystem(format!(
                    "Post image should have a parent `{}`",
                    old_image_path.display()
                ))
            })?)
            .map_err(|err| ApcError::FileSystem(err.to_string()))?;

            self.image_path = new_image_path;
        }
        Ok(())
    }

    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn full_properties(&self, config: &Config) -> String {
        // FIXME: Remove a escape and use `#""#`.
        // Use key value in format macro
        format!(
            "---\ntitle: \"{}\"\n\
            layout: \"{}\"\n\
            image: \"{}\"\n\
            link: \"{}\"\n\
            date: \"{}\"\n\
            dateModified: \"{}\"\n\
            description: \"{}\"\n\
            draft: {}\n\
            tags: {:?}\n\
            ---\n\n\
            # {}\n",
            self.title,
            self.layout,
            self.image_path
                .replace(&config.images_path, &config.images_site_path),
            self.link,
            self.date.format(&config.date_format),
            self.date_modified.format(&config.date_format),
            self.description,
            self.is_draft,
            self.tags,
            self.title,
        )
    }

    /// Return str props
    #[logfn(Debug)]
    pub fn str_properties(&self, config: &'static Config) -> String {
        // parse_props` will return error if the properties are invalid syntax.
        Self::parse_props(self.full_properties(config)).unwrap()
    }

    /// Return the post path
    pub fn path(&self, config: &'static Config) -> PathBuf {
        let slug = slug_updater(&self.slug);
        Path::new(&config.posts_path)
            .join(&slug)
            .with_extension("md")
    }

    /// Write the post properties in file
    #[logfn(Debug)]
    pub fn write_in_file(&self, config: &'static Config) -> ApcResult<()> {
        fs::write(self.path(config), self.full_properties(config)).map_err(|err| {
            log::error!("{:?}", err);
            ApcError::FileSystem(err.to_string())
        })?;

        Ok(())
    }

    /// Update the file
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn modify_post(&mut self, new_slug: String, new_image: String) -> ApcResult<()> {
        let config = CONFIG.as_ref().unwrap();
        let old_slug = self.slug.clone();
        self.update_slug(config, &new_slug)?;
        self.update_images(config, &old_slug)?;
        self.write_in_file(config)?;
        Ok(())
    }

    /// Return a properties from file
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn properties_from_file<P>(post_path: P) -> ApcResult<String>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        // FIXME: I don't think it's the best way to read large files
        fn inner(path: &Path) -> ApcResult<String> {
            Post::parse_props(
                fs::read_to_string(path).map_err(|err| ApcError::FileSystem(err.to_string()))?,
            )
        }
        inner(post_path.as_ref())
    }

    /// Parse full props to key val str props
    fn parse_props(full_props: String) -> ApcResult<String> {
        let re = regex::Regex::new(r"^---\n(\w*: ?.*\n){9}---").unwrap();
        if re.is_match(&full_props) {
            Ok(full_props
                .lines()
                .skip(1)
                .take(9)
                .collect::<Vec<&str>>()
                .join("\n"))
        } else {
            Err(ApcError::PostProperties(
                "Invalid post properties in ".to_owned(),
            ))
        }
    }

    /// Returns post by file properties
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn from_file<P>(config: &'static Config, post_path: P) -> ApcResult<Self>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        let props = properties::Props::from_str(
            Self::properties_from_file(&post_path)?.as_str(),
            post_path,
        )?;
        Post::try_new(
            config,
            props.title,
            props.slug,
            props.draft,
            props.desctiption,
            props.tags,
            props.image_path,
            props.date,
            props.modified_date,
        )
    }
}
