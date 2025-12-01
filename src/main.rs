/*
	CBFGBeast Version 2.0

Copyright (C) 2025 Outerbeast
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
*/
#![windows_subsystem = "windows"]

pub mod driver;
pub mod init;
pub mod gui;
pub mod cfg;
pub mod motd;

pub const APPNAME: &str = "CFGBeast";

fn main() -> std::process::ExitCode
{
    match driver::run()
    {
        Ok( _ ) =>
        {
            println!( "Application ran successfully." );
            std::process::ExitCode::SUCCESS
        }

        Err( e ) =>
        {
            eprintln!( "Application error: {}", e );
            std::process::ExitCode::FAILURE
        }
    }
}
