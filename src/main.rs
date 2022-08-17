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

#[macro_use]
extern crate educe;
#[macro_use]
extern crate log_derive;
extern crate lazy_static;
extern crate pretty_env_logger;

mod app;
mod config;
mod errors;
mod utils;

#[cfg(test)]
mod tests;

use errors::{ApcError, Statuses};
use lazy_static::lazy_static;
use std::env::var;

lazy_static! {
    static ref CONFIG: Result<config::Config, ApcError> = config::get_config();
}

fn main() -> Statuses<ApcError> {
    var("RUST_LOG").is_ok().then(pretty_env_logger::init);
    match CONFIG.as_ref() {
        Ok(alepc_config) => app::run(alepc_config).into(),
        Err(err) => Statuses::Failure(err.clone()),
    }
}
