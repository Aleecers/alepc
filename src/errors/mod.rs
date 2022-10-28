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

mod statuses;

use colored::Colorize;
use ron::error::{ErrorCode, Position};
use strum::IntoStaticStr;
use thiserror::Error;

pub use statuses::Statuses;
/// Alepc errors
#[derive(IntoStaticStr, Error, Debug, Clone)]
pub enum ApcError {
    #[error("Cannot parse config file '{code}' in {position}")]
    ParseRon { code: ErrorCode, position: Position },
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    FileSystem(String),
    #[error("{0}")]
    Requestty(String),
    #[error("{0}")]
    PostProperties(String),
    #[error("{0}")]
    Other(String),
}

impl ApcError {
    /// return variant name
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn name(&self) -> &'static str {
        self.into()
    }

    /// Print error message
    #[logfn(Debug)]
    #[logfn_inputs(Info)]
    pub fn print(&self) {
        (!matches!(self, Self::Requestty(_)))
            .then(|| eprintln!("{}: {}", format!("{}Error", self.name()).red(), self));
    }
}

pub type ApcResult<T> = Result<T, ApcError>;
