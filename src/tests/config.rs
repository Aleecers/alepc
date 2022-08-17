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
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.use crate::config::Config;

use crate::config::Config;
use serial_test::serial;
use std::fs;
use std::path::Path;
use test_case::test_case;

#[test]
fn test_config_validation() {
    let config = Config {
        posts_path: "random".to_owned(),
        ..Default::default()
    };
    assert!(config.configuration().is_err());
    let valid_config = Config {
        posts_path: "./src".to_owned(),
        images_path: "./src/config/".to_owned(),
        posts_layout: "tests/mod.rs".to_owned(),
        ..Default::default()
    };
    assert!(valid_config.configuration().is_ok())
}

#[test_case("./some/path/to/config.ron" ; "dot_deep_path")]
#[test_case("some/path/to/config.ron" ; "undot_deep_path")]
#[test_case("./config.ron" ; "dot_path")]
#[test_case("config.ron" ; "undot_path")]
#[serial]
fn test_write_config(config_str_path: &str) {
    let valid_config = Config {
        posts_path: "./src".to_owned(),
        images_path: "./src/config/".to_owned(),
        posts_layout: "tests/mod.rs".to_owned(),
        ..Default::default()
    };
    let config_path = Path::new(config_str_path);
    assert!(!config_path.exists());
    let write_result = valid_config.write(config_path);
    assert!(write_result.is_ok(), "{:?}", write_result.err());
    assert!(config_path.exists());
    assert!(fs::remove_file(config_path).is_ok());
    fs::remove_dir_all("some/").is_ok().then(|| 0);
}
