/*
	CFGBeast Version 3.0

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
#![cfg_attr( target_os = "windows", windows_subsystem = "windows" )]
mod driver;
pub mod app;
pub mod config;
pub mod cvar;
pub mod replacements;
pub mod materials;
pub mod prelude;
pub mod utils;
#[cfg( test )] mod tests;

pub const APPNAME: &str = env!( "CARGO_PKG_NAME" );

fn main() -> std::process::ExitCode
{
    // Force winit to use X11/XWayland on Linux, because winit 0.30
    // does not implement DroppedFile/HoveredFile events on Wayland.
    // SAFETY: called at the very start of main(), before any threads
    // are spawned and before any environment reads by other code.
    // TODO: remove once winit supports Wayland DnD
    #[cfg( target_os = "linux" )] unsafe { std::env::remove_var( "WAYLAND_DISPLAY" ); }

    match driver::run()
    {
        Ok( () ) =>
        {
            println!( "Application ran successfully." );
            std::process::ExitCode::SUCCESS
        }

        Err( e ) =>
        {
            rfd::MessageDialog::new()
                .set_title( "Fatal Error" )
                .set_description( format!( "{e}" ) )
                .set_level( rfd::MessageLevel::Error )
            .show();

            eprintln!( "Application error: {e}" );
            std::process::ExitCode::FAILURE
        }
    }
}
