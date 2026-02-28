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
extern crate native_windows_gui as nwg;

use std::
{
    path::Path,
    rc::Rc
};

use native_windows_gui::
{
    dispatch_thread_events,
    init,
    stop_thread_dispatch,
    CheckBoxState,
    Event,
    EventData,
    ListBox,
    MessageButtons,
    MessageIcons
};

use crate::
{
    alloc_leaked,
    utils,
    APPNAME,
    gui::window::
    {
        build_main_window,
        message_box,
        HELP_INFO,
        CHECKED,
        UNCHECKED
    },
    cvar::
    {
        get_skill_cvars,
        get_default_cvars,
        load_bsps,
        Cfg,
        WriteType
    }
};

pub fn GUI(bsp_path: &Path)
{
    if let Err( e ) = init()
    {
        message_box( "FATAL ERROR",
        format!( "Failed to initialise window.\nError code: {}", e ).as_str(),
        MessageButtons::Ok,
        MessageIcons::Error );

        panic!( "{} panicked: {}", APPNAME, e );
    }

    let gui = build_main_window( bsp_path );
    let cloned_gui = Rc::clone( &gui );
    let window_handle = gui.borrow().window.handle;

    nwg::bind_event_handler( &window_handle, &window_handle, move |evt, evt_data, handle|
    {
        let gui = cloned_gui.borrow();

        match evt
        {
            Event::OnButtonClick
            if handle == gui.checkbox.handle =>
            {
                let is_checked = gui.checkbox.check_state() == CheckBoxState::Checked;

                match is_checked
                {
                    true => { gui.listbox_cvar.set_collection( get_skill_cvars() ); }
                    false => { gui.listbox_cvar.set_collection( get_default_cvars() ); }
                }
            }

            Event::OnButtonClick =>
            {
                let cvars = gui.textbox.text();
                let bspwhitelist = current_bsp_whitelist( &gui.listbox_bsp );
                let is_skillcfg = gui.checkbox.check_state() == CheckBoxState::Checked;
                let bspdir = gui.bsp_dir.clone();

                if handle == gui.buttons[0].handle
                {
                    Cfg
                    { 
                        cvars,
                        writetype: WriteType::OVERWRITE,
                        is_skillcfg,
                        bspdir,
                        bspwhitelist: bspwhitelist.clone() 
                    }.create();
                }
                else if handle == gui.buttons[1].handle
                {
                    Cfg
                    { 
                        cvars,
                        writetype: WriteType::APPEND,
                        is_skillcfg,
                        bspdir,
                        bspwhitelist: bspwhitelist.clone()
                    }.create();
                }
                else if handle == gui.buttons[2].handle
                {
                    Cfg
                    { 
                        cvars, 
                        writetype: WriteType::REMOVE, 
                        is_skillcfg,
                        bspdir,
                        bspwhitelist: bspwhitelist.clone() 
                    }.create();
                }
                else if handle == gui.buttons[3].handle
                {
                    Cfg
                    {
                        cvars,
                        writetype: WriteType::DELETE,
                        is_skillcfg,
                        bspdir,
                        bspwhitelist: current_bsp_whitelist( &gui.listbox_bsp )
                    }.create();
                }
                else if handle == gui.buttons[6].handle
                {
                    let selected_bsp_folder =
                    match utils::select_folder_dialogue( &gui.window )
                    {
                        Some( path ) => path,
                        None =>
                        {   // A folder wasn't picked
                            return;
                        }
                    };

                    if selected_bsp_folder.exists() && utils::dir_contains_type( &selected_bsp_folder, "bsp" )
                    {   
                        drop( gui );// Release the immutable borrow before taking a mutable one
                        let mut gui_mut = cloned_gui.borrow_mut();
                        gui_mut.bsp_dir = selected_bsp_folder;
                        let bsp_paths = load_bsps( gui_mut.bsp_dir.as_path() );
                        // Extract filenames and mark them all as checked
                        let bsp_filenames: Vec<String> = bsp_paths.iter()
                            .filter_map( |p| p.file_name() )
                            .map( |name| format!( "{CHECKED}\t{}", name.to_string_lossy() ) )
                        .collect();

                        gui_mut.listbox_bsp.set_collection( bsp_filenames );
                    }
                    else
                    {
                        message_box( "Invalid folder",
                        "The selected folder does not contain any BSP files.",
                        MessageButtons::Ok,
                        MessageIcons::Error );
                    }
                }
                else if handle == gui.buttons[4].handle
                {
                    stop_thread_dispatch();
                }
                else if handle == gui.buttons[5].handle
                {
                    message_box( "Help", HELP_INFO, MessageButtons::Ok, MessageIcons::Question );
                }
            }

            Event::OnFileDrop =>
            {   // Release the long-lived immutable borrow taken before the match
                drop( gui );

                if let EventData::OnFileDrop( drop ) = evt_data
                {
                    let mut combined = String::new();

                    for path in drop.files()
                    {
                        if Path::new( &path ).extension().and_then( |s| s.to_str() ) != Some( "cfg" )
                        {   // Not a .cfg file, skip
                            continue;
                        }

                        if let Ok( content ) = std::fs::read_to_string(&path)
                        {
                            combined.push_str( &content );
                            combined.push( '\n' );
                        }
                    }

                    {
                        let gui_mut = cloned_gui.borrow_mut();
                        gui_mut.textbox.set_text( &combined );
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
                        gui.listbox_bsp.set_collection( items );
                    }
                }
                else if handle == gui.listbox_cvar.handle
                && let Some( idx ) = gui.listbox_cvar.selection()
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

            Event::OnWindowClose =>
            {
                stop_thread_dispatch();
            }

            _ => { }
        }
    });
    // Keep the window alive in heap so events can be handled
    alloc_leaked!( gui );
    dispatch_thread_events();
}

fn current_bsp_whitelist(listbox: &ListBox<String>) -> Vec<String>
{
    let checked = CHECKED;
    listbox.collection()
        .iter()
        .filter_map( |s| 
        {
            match s.starts_with( checked )
            {
                true => Some( s[checked.len() + 1..].to_string() ),// strip "☑ "
                false => None
            }
        })
    .collect()
}
