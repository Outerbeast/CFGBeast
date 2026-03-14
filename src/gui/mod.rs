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
pub mod window;
pub mod events;

use std::path::Path;
pub use
{
    window::
    {
        MainWindow,
        build_main_window
    }
};

use native_windows_gui::
{
    dispatch_thread_events,
    init,
    message,
    MessageButtons,
    MessageChoice,
    MessageIcons,
    MessageParams,
    NwgError
};

use crate::alloc_leaked;

const WINDOW_SIZE: (i32, i32) = (860, 440 );
const BUTTON_SIZE: (i32, i32) = ( 85, 30 );
const TEXTBOX_SIZE: (i32, i32) = ( 330, 338 );
const CVAR_LIST_SIZE: (i32, i32) = ( 330, 350 );
const BSP_LIST_SIZE: (i32, i32) = ( 160, 350 );
const UNCHECKED: &str = "☐";
const CHECKED:   &str = "✔";
const HELP_INFO: &str =
    r#"This is a simple application to create, append, remove, and delete CFG files based on the BSP files in the current directory.

    Controls:-
    - Enter the cvars you want to manage in the text box, by either:
        selecting CVar presets in the right list,
        dragging in an exiting CFG file into the box,
        or typing them in manually.

    - 'Create': create or overwrite CFG files.
    - 'Add': appends cvars to existing CFG files.
    - 'Remove': remove specified cvars from CFG files.
    - 'Delete': deletes all CFG files in the current directory.
    - 'Change': changes the current BSP folder

    Thank you for using this app!
    If you'd like to give feedback feel free to put them here: https://github.com/Outerbeast/CFGBeast/issues
    "#;


pub fn message_box(title: &str, content: &str, buttons: MessageButtons, icons: MessageIcons) -> MessageChoice
{
    message( &MessageParams
    {
        title,
        content,
        buttons,
        icons,
    })
}

pub fn launch_gui(bsp_path: &Path) -> Result<(), NwgError>
{
    init()?;
    let gui = build_main_window( bsp_path );
    events::setup_event_handlers( &gui );
    // Keep the window alive in heap so events can be handled
    alloc_leaked!( gui );
    dispatch_thread_events();

    Ok(())
}
