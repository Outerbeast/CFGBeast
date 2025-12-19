/*
	CFGBeast Version 2.1

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
use std::
{
    env,
    fs
};

use native_windows_gui::
{
    MessageButtons,
    MessageIcons
};

use crate::gui::window::message_box;

pub fn create_motd(motd_content: String)
{
    if motd_content.trim().is_empty()
    {
        return;
    }

    let current_dir = 
    env::current_dir()
        .inspect_err( |e|
        {
            message_box("Error",
                format!( "Failed to get current directory.\nReason:\n {}", e ).as_str(),
                MessageButtons::Ok,
                MessageIcons::Error );
        })
    .unwrap_or_default();

    let mut count = 0u8;
    // Iterate over *.bsp files in current directory
    for entry in fs::read_dir( &current_dir ).expect( "Failed to read directory" )
    {
        let path = entry.unwrap().path();

        if path.extension().and_then( |s| s.to_str() ) != Some( "bsp" )
        {
            continue;
        }

        let Some( base_name ) = path.file_stem().and_then( |s| s.to_str() ) 
        else
        { 
            continue
        };
  
        let motd_filename = format!( "{}_motd.txt", base_name );
        count += fs::write( &motd_filename, &motd_content ).is_ok() as u8;
    }

    match count
    {
        0 =>
        { 
            message_box( "No MOTD files written",
                "No MOTD files written.\n\nPlease place the app executable in a map folder with valid BSPs and try again.",
                MessageButtons::Ok,
                MessageIcons::Warning );
        }

        _ =>
        {
            message_box( "Done",
            &format!( "Processed {} MOTD file(s).", count ),
            MessageButtons::Ok,
            MessageIcons::Info );
        }
    }
}
