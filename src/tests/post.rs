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

use crate::{config::Config, utils::Post};
use serial_test::serial;
use std::{fs, path::Path};
use test_case::test_case;

#[test_case("some/path/to/", "slug" ; "undot_one_sllug")]
#[test_case("some/path/to/", "slug-234" ; "undot_tow_sllug")]
#[test_case("./some/path/to/", "slug" ; "dot_one_slug")]
#[test_case("./some/path/to/", "slug-20dk" ; "dot_tow_slug")]
#[serial]
fn test_write_post(posts_path: &str, slug: &str) {
    let config = Config {
        posts_path: posts_path.to_owned(),
        ..Default::default()
    };
    let post = Post {
        slug: slug.to_owned(),
        ..Default::default()
    };
    let filename = Path::new(posts_path).join(slug).with_extension("md");
    assert!(fs::create_dir_all(posts_path).is_ok());
    assert!(!filename.exists());
    let write_post_result = post.write_in_file(&config);
    assert!(write_post_result.is_ok(), "{:?}", write_post_result.err());
    assert!(filename.exists());
    assert!(fs::remove_file(filename).is_ok());
    assert!(fs::remove_dir_all(posts_path).is_ok());
}
