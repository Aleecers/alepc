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

use super::ApcError;
use std::process::{ExitCode, Termination};

pub enum Statuses<T: Termination> {
    Success,
    Failure(T),
}

impl Termination for ApcError {
    fn report(self) -> ExitCode {
        self.print();
        match self {
            // permission denied
            Self::FileSystem(_) => 77,
            // configuration error
            Self::Validation(_)
            | Self::ParseRon {
                code: _,
                position: _,
            } => 78,
            _ => 1,
        }
        .into()
    }
}

impl<E: Termination> Termination for Statuses<E> {
    fn report(self) -> ExitCode {
        if let Self::Failure(err) = self {
            err.report()
        } else {
            ExitCode::SUCCESS
        }
    }
}

impl<T, E: Termination> From<Result<T, E>> for Statuses<E> {
    fn from(r: Result<T, E>) -> Self {
        if let Err(err) = r {
            Self::Failure(err)
        } else {
            Self::Success
        }
    }
}
