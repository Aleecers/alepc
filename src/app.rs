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
use crate::utils::questions::{create::post_properties, modify::modify_post_properties};
use crate::utils::Post;
use crate::CONFIG;
use requestty::{prompt, Answers, Question};

#[derive(Debug)]
pub enum Action {
    Create(Post),
    Modify {
        new_post: Post,
        new_slug: String,
        new_image_path: String,
    },
    Version,
}

impl TryFrom<&Answers> for Action {
    type Error = ApcError;

    fn try_from(answers: &Answers) -> ApcResult<Action> {
        let action = answers
            .get("action")
            .unwrap()
            .as_list_item()
            .unwrap()
            .text
            .as_str();
        let config: &Config = CONFIG.as_ref().unwrap();
        if action == config.select_action.new_post_choice {
            // Create action
            Post::create_action(config, answers)
        } else if action == config.select_action.update_existing_post {
            // Modify action
            Post::modify_action(config, answers)
        } else {
            // Version action
            Ok(Self::Version)
        }
    }
}

const VERSION: &str = "0.1.0";

/// Return the questions
#[logfn(Debug)]
#[logfn_inputs(Info)]
fn questions(config: &'static Config) -> Vec<Question> {
    let mut questions = vec![Question::select("action")
        .message(&config.select_action.select_action_message)
        .choices(vec![
            &config.select_action.new_post_choice,
            &config.select_action.update_existing_post,
            &config.select_action.version_choice,
        ])
        .default(0)
        .build()];
    questions.append(&mut post_properties(config));
    questions.append(&mut modify_post_properties(config));
    questions
}

#[logfn(Debug)]
#[logfn_inputs(Info)]
pub fn run(config: &'static Config) -> ApcResult<()> {
    let answers = prompt(questions(config)).map_err(|err| {
        log::error!("{:?}", err);
        ApcError::Requestty(err.to_string())
    })?;
    let action = Action::try_from(&answers)?;
    log::debug!("answers = {answers:?}\naction = {action:?}");
    match action {
        Action::Create(post) => post.write_in_file(config)?,
        Action::Modify {
            mut new_post,
            new_slug,
            new_image_path,
        } => new_post.modify_post(new_slug, new_image_path)?,
        Action::Version => {
            println!(
                "⚙ Version: v{}\n🕸 Repository: {}",
                VERSION, config.repository_url
            );
        }
    }
    Ok(())
}
