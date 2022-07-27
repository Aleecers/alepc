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
use crate::errors::{ApcError, ApcResult};
use crate::utils;
use crate::utils::questions::post_properties;
use crate::utils::Post;
use requestty::{prompt, Answers, Question};

pub enum PostAction {
    Create,
    Version,
    _Modify,
}

const VERSION: &str = "0.1.0";

/// Return the questions
fn questions(config: &Config) -> Vec<Question> {
    let mut questions = vec![Question::select("action")
        .message(&config.select_action.select_action_message)
        .choices(vec![
            &config.select_action.new_post_choice,
            &config.select_action.update_modified_post_date,
            &config.select_action.version_choice,
        ])
        .default(0)
        .build()];
    questions.append(&mut post_properties(config));
    questions
}

/// Returns post from answers, None if there is no post
pub fn post_from_answers<'a>(
    apc_config: &'a Config,
    answers: &'a Answers,
) -> ApcResult<(Option<Post<'a>>, PostAction)> {
    if answers.get("action").unwrap().as_list_item().unwrap().text
        == apc_config.select_action.new_post_choice
    {
        let slug = answers.get("post_slug").unwrap().as_string().unwrap();
        Ok((
            Some(Post::new(
                apc_config,
                answers
                    .get("post_title")
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .trim(),
                slug,
                true,
                answers
                    .get("post_description")
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .trim(),
                utils::tags_updater(
                    answers.get("post_tags").unwrap().as_string().unwrap(),
                    apc_config.input_settings.separated_tags_by,
                ),
                utils::copy_image(
                    apc_config,
                    slug,
                    answers.get("post_image").unwrap().as_string().unwrap(),
                )?,
            )),
            PostAction::Create,
        ))
    } else if answers.get("action").unwrap().as_list_item().unwrap().text
        == apc_config.select_action.new_post_choice
    {
        unimplemented!("Unimplemented action")
    } else if answers.get("action").unwrap().as_list_item().unwrap().text
        == apc_config.select_action.version_choice
    {
        Ok((None, PostAction::Version))
    } else {
        panic!("Unsupported action")
    }
}

pub fn run(config: &Config) -> ApcResult<()> {
    let answers = prompt(questions(config)).map_err(|err| ApcError::Requestty(err.to_string()))?;
    let (post, action) = post_from_answers(config, &answers)?;
    match action {
        PostAction::Create => post.unwrap().write_in_file(config)?,
        PostAction::Version => {
            println!(
                "⚙ Version: v{}\n🕸 Repository: {}",
                VERSION, config.repository_url
            );
        }
        _ => unimplemented!("Unimplemented Action"),
    }
    Ok(())
}
