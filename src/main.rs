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
extern crate pretty_env_logger;

mod app;
mod config;
mod errors;
mod utils;

fn main() -> errors::Statuses<errors::ApcError> {
    pretty_env_logger::init();
    let alepc_config = config::get_config();
    if let Ok(alepc_config) = alepc_config {
        // Will run `errors::Statuses::report`-> `errors::ApcError::report`
        // if it's error else return `ExitCode::SUCCESS`
        app::run(&alepc_config).into()
    } else {
        // Will run `errors::Statuses::report`-> `errors::ApcError::report`
        alepc_config.into()
    }
}
