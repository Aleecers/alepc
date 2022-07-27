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
use crate::errors::ApcResult;
use crate::utils::slug_updater;
use crate::ApcError;
use chrono::prelude::*;
use std::fs;
use std::path::Path;

/// Struct containing the post information
pub struct Post<'a> {
    title: &'a str,
    layout: &'a str,
    slug: &'a str,
    is_draft: bool,
    description: &'a str,
    tags: Vec<String>,
    image_path: String,
    date: DateTime<Local>,
    date_modified: Option<DateTime<Local>>,
    link: String,
}

impl<'a> Post<'a> {
    pub fn new(
        config: &'a Config,
        title: &'a str,
        slug: &'a str,
        is_draft: bool,
        description: &'a str,
        tags: Vec<String>,
        image_name: String,
    ) -> Self {
        let modified_slug = slug_updater(slug);
        Self {
            title,
            slug,
            is_draft,
            description,
            tags,
            layout: &config.posts_layout,
            image_path: format!(
                "{}{}/{}",
                config.images_site_path, modified_slug, image_name
            ),
            date: Local::now(),
            date_modified: None,
            link: format!("{}{}", config.blog_site_path, modified_slug),
        }
    }

    pub fn to_post_properties(&self, config: &Config) -> String {
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
            self.image_path,
            self.link,
            self.date.format(&config.date_format),
            self.date_modified
                .unwrap_or(self.date)
                .format(&config.date_format),
            self.description,
            self.is_draft,
            self.tags,
            self.title,
        )
    }

    /// Write the post properties to file
    pub fn write_in_file(&self, config: &Config) -> ApcResult<()> {
        let slug = slug_updater(self.slug);
        let file_path = Path::new(&config.posts_path)
            .join(&slug)
            .with_extension("md");
        fs::write(file_path, self.to_post_properties(config))
            .map_err(|err| ApcError::FileSystem(err.to_string()))?;

        Ok(())
    }
}
