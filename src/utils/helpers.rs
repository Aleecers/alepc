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
use requestty::Answers;

pub fn is_new_post(config: &'static Config) -> impl Fn(&Answers) -> bool {
    move |answers| {
        answers.get("action").unwrap().as_list_item().unwrap().text
            == config.select_action.new_post_choice
    }
}

pub fn get_str_length(str_text: &str) -> usize {
    str_text.trim().chars().count()
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
