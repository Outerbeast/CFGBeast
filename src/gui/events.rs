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
extern crate native_windows_gui as nwg;

use std::
{
    env,
    rc::Rc
};

use nwg::*;
use super::window::*;
use crate::
{
    APPNAME,
    cfg::
    {
        self,
        *
    }
};

const HELP_INFO: &str =
"This is a simple application to create, append, remove, and delete CFG files based on the BSP files in the current directory.\n\n
    1. Enter the cvars you want to manage in the text box, by selecting cvar presets or typing them in.\n
    2. Click 'Create' to create or overwrite CFG files.\n
    3. Click 'Add' to append cvars to existing CFG files.\n
    4. Click 'Remove' to remove specified cvars from CFG files.\n
    5. Click 'Delete' to delete all CFG files in the current directory.\n
    6. Click 'Cancel' or 'x' to exit the application.\n\n
    Thank you for using this app!\nIf you'd like to give feedback feel free to put them here: https://github.com/Outerbeast/CFGBeast/issues";

pub fn GUI()
{
    if let Err( e ) = init()
    {
        message_box( "FATAL ERROR",
        format!( "Failed to initialise window.\nError code: {}", e ).as_str(),
        MessageButtons::Ok,
        MessageIcons::Error );

        panic!( "{} panicked: {}", APPNAME, e );
    }

    let gui = crate::gui::window::build_main_window();
    let cloned_gui = Rc::clone( &gui );
    let window_handle = gui.borrow().window.handle;

    nwg::bind_event_handler( &window_handle, &window_handle, move |evt, evt_data, handle|
    {
        let gui = cloned_gui.borrow();

        match evt
        {
            Event::OnButtonClick if handle == gui.checkbox.handle =>
            {
                let is_checked = gui.checkbox.check_state() == nwg::CheckBoxState::Checked;

                match is_checked
                {
                    true => { gui.listbox_cvar.set_collection( cfg::get_skill_cvars() ); }
                    false => { gui.listbox_cvar.set_collection( cfg::get_default_cvars() ); }
                }
            }

            Event::OnButtonClick =>
            {
                let contents = gui.textbox.text();
                let bspwhitelist = current_bsp_whitelist( &gui.listbox_bsp );

                if handle == gui.buttons[0].handle
                {
                    create_cfg( cfg::Cfg
                    { 
                        cvars: contents, writetype:
                        WriteType::OVERWRITE, skillcfg:
                        gui.checkbox.check_state() == nwg::CheckBoxState::Checked,
                        bspdir: env::current_dir().unwrap(),
                        bspwhitelist: bspwhitelist.clone() 
                    });
                }
                else if handle == gui.buttons[1].handle
                {
                    create_cfg( cfg::Cfg
                    { 
                        cvars: contents,
                        writetype: WriteType::APPEND,
                        skillcfg: gui.checkbox.check_state() == nwg::CheckBoxState::Checked,
                        bspdir: env::current_dir().unwrap(),
                        bspwhitelist: bspwhitelist.clone()
                    });
                }
                else if handle == gui.buttons[2].handle
                {
                    create_cfg( cfg::Cfg
                    { 
                        cvars: contents, 
                        writetype: WriteType::REMOVE, 
                        skillcfg: gui.checkbox.check_state() == nwg::CheckBoxState::Checked,
                        bspdir: env::current_dir().unwrap(),
                        bspwhitelist: bspwhitelist.clone() 
                    });
                }
                else if handle == gui.buttons[3].handle
                {
                    create_cfg( cfg::Cfg
                    {
                        cvars: contents,
                        writetype: WriteType::DELETE,
                        skillcfg: gui.checkbox.check_state() == nwg::CheckBoxState::Checked,
                        bspdir: env::current_dir().unwrap(),
                        bspwhitelist: current_bsp_whitelist( &gui.listbox_bsp )
                    });
                }
                else if handle == gui.buttons[4].handle
                {
                    nwg::stop_thread_dispatch();
                }
                else if handle == gui.buttons[5].handle
                {
                    message_box( "Help", HELP_INFO, MessageButtons::Ok, MessageIcons::Question );
                }
            }
            // !-ISSUE-!: Doesn't do anything - flags can't be set on textbox for some reason to allow drops
            Event::OnFileDrop =>
            {
                if let EventData::OnFileDrop( drop) = evt_data
                {
                    for file in drop.files()
                    {
                        if let Ok( content ) = std::fs::read_to_string( file )
                        {
                            gui.textbox.set_text( &content );
                        }
                    }
                }
            }

            Event::OnListBoxSelect =>
            {   // BSP whitelist listbox → toggle checkmark
                if handle == gui.listbox_bsp.handle
                {
                    if let Some(idx) = gui.listbox_bsp.selection()
                    {   // Clone the entire vector so we don’t mix borrows
                        let mut items = gui.listbox_bsp.collection().clone();
                        // Toggle the leading glyph on the cloned item
                        let current = items[idx].clone();

                        items[idx] =
                        match current.starts_with( CHECKED )
                        {
                            true => format!( "{UNCHECKED}\t{}", &current[CHECKED.len() + 1..] ),
                            false => format!( "{CHECKED}\t{}", &current[UNCHECKED.len() + 1..] )
                        };
                        // Replace the collection in one go
                        gui.listbox_bsp.set_collection(items);
                    }
                }
                else if handle == gui.listbox_cvar.handle
                {
                    if let Some( idx ) = gui.listbox_cvar.selection()
                    && let Some( selected ) = gui.listbox_cvar.collection().get( idx )
                    {
                        let mut current = gui.textbox.text();

                        if !current.trim().is_empty() && !current.ends_with( '\n' )
                        {
                            current.push_str( "\r\n" );
                        }

                        current.push_str( selected );
                        gui.textbox.set_text( &current );
                    }
                }
            }

            Event::OnWindowClose =>
            {
                nwg::stop_thread_dispatch();
            }

            _ => { }
        }
    });
    // Keep the window alive in heap so events can be handled
    Box::leak( Box::new( gui ) );
    nwg::dispatch_thread_events();
}

fn current_bsp_whitelist(listbox: &nwg::ListBox<String>) -> Vec<String>
{
    let checked = CHECKED;
    listbox.collection()
        .iter()
        .filter_map( |s| 
        {
            match s.starts_with( checked )
            {
                true => Some( s[checked.len() + 1..].to_string() ), // strip "☑ "
                false => None
            }
        })
    .collect()
}
