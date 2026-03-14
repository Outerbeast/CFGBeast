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
    path::Path,
    rc::Rc,
    cell::RefCell
};

use native_windows_gui::
{
    bind_event_handler,
    stop_thread_dispatch,
    CheckBoxState,
    Event,
    EventData,
    ListBox,
    MessageButtons,
    MessageIcons,
};

use crate::
{
    utils,
    cvar::
    {
        get_skill_cvars,
        get_default_cvars,
        load_bsps,
        Cfg,
        WriteType
    }
};

use super::
{
    CHECKED,
    HELP_INFO,
    UNCHECKED,
    MainWindow,
    message_box
};
// Checkbox handler
fn on_checkbox_toggled(gui: &mut MainWindow)
{
    let is_checked = gui.checkbox.check_state() == CheckBoxState::Checked;

    match is_checked
    {
        true => { gui.listbox_cvar.set_collection( get_skill_cvars() ); }
        false => { gui.listbox_cvar.set_collection( get_default_cvars() ); }
    }
}
// Button handlers
fn on_create_button(gui: &mut MainWindow)
{
    let cvars = gui.textbox.text();
    let bspwhitelist = current_bsp_whitelist( &gui.listbox_bsp );
    let is_skillcfg = gui.checkbox.check_state() == CheckBoxState::Checked;
    let bspdir = gui.bsp_dir.clone();

    Cfg
    {
        cvars,
        writetype: WriteType::OVERWRITE,
        is_skillcfg,
        bspdir,
        bspwhitelist
    }.create();
}

fn on_add_button(gui: &mut MainWindow)
{
    let cvars = gui.textbox.text();
    let bspwhitelist = current_bsp_whitelist( &gui.listbox_bsp );
    let is_skillcfg = gui.checkbox.check_state() == CheckBoxState::Checked;
    let bspdir = gui.bsp_dir.clone();

    Cfg
    {
        cvars,
        writetype: WriteType::APPEND,
        is_skillcfg,
        bspdir,
        bspwhitelist
    }.create();
}

fn on_remove_button(gui: &mut MainWindow)
{
    let cvars = gui.textbox.text();
    let bspwhitelist = current_bsp_whitelist( &gui.listbox_bsp );
    let is_skillcfg = gui.checkbox.check_state() == CheckBoxState::Checked;
    let bspdir = gui.bsp_dir.clone();

    Cfg
    {
        cvars,
        writetype: WriteType::REMOVE,
        is_skillcfg,
        bspdir,
        bspwhitelist
    }.create();
}

fn on_delete_button(gui: &mut MainWindow)
{
    let cvars = gui.textbox.text();
    let bspwhitelist = current_bsp_whitelist( &gui.listbox_bsp );
    let is_skillcfg = gui.checkbox.check_state() == CheckBoxState::Checked;
    let bspdir = gui.bsp_dir.clone();

    Cfg
    {
        cvars,
        writetype: WriteType::DELETE,
        is_skillcfg,
        bspdir,
        bspwhitelist
    }.create();
}

fn on_cancel_button()
{
    stop_thread_dispatch();
}

fn on_help_button()
{
    message_box( "Help", HELP_INFO, MessageButtons::Ok, MessageIcons::Question );
}

fn on_change_folder_button(gui: &mut MainWindow)
{
    let selected_bsp_folder =
    match utils::select_folder_dialogue( &gui.window )
    {
        Some( path ) => path,
        None => return
    };

    if selected_bsp_folder.exists() && utils::dir_contains_type( &selected_bsp_folder, "bsp" )
    {
        gui.bsp_dir = selected_bsp_folder;
        let bsp_paths = load_bsps( gui.bsp_dir.as_path() );
        let bsp_filenames: Vec<String> = bsp_paths.iter()
            .filter_map( |p| p.file_name() )
            .map( |name| format!( "{CHECKED}\t{}", name.to_string_lossy() ) )
        .collect();

        gui.listbox_bsp.set_collection( bsp_filenames );
    }
    else
    {
        message_box( "Invalid folder",
            "The selected folder does not contain any BSP files.",
            MessageButtons::Ok,
            MessageIcons::Error );
    }
}

// File drop handler
fn on_file_drop(gui: &mut MainWindow, evt_data: &EventData)
{
    if let EventData::OnFileDrop( drop ) = evt_data
    {
        let mut combined = String::new();

        for path in drop.files()
        {
            if Path::new( &path ).extension().and_then( |s| s.to_str() ) != Some( "cfg" )
            {
                continue;
            }

            if let Ok( content ) = std::fs::read_to_string( &path )
            {
                combined.push_str( &content );
                combined.push( '\n' );
            }
        }

        gui.textbox.set_text( &combined );
    }
}

// ListBox handlers
fn on_bsp_listbox_select(gui: &mut MainWindow)
{
    if let Some( idx ) = gui.listbox_bsp.selection()
    {
        let mut items = gui.listbox_bsp.collection().clone();
        let current = items[idx].clone();

        items[idx] =
        match current.starts_with( CHECKED )
        {
            true => format!( "{UNCHECKED}\t{}", &current[CHECKED.len() + 1..] ),
            false => format!( "{CHECKED}\t{}", &current[UNCHECKED.len() + 1..] )
        };

        gui.listbox_bsp.set_collection( items );
    }
}

fn on_cvar_listbox_select(gui: &mut MainWindow)
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
// Window handler
fn on_window_close()
{
    stop_thread_dispatch();
}
// Helper function
fn current_bsp_whitelist(listbox: &ListBox<String>) -> Vec<String>
{
    let checked = CHECKED;
    listbox.collection()
        .iter()
        .filter_map( |s| 
        {
            match s.starts_with( checked )
            {
                true => Some( s[checked.len() + 1..].to_string() ),
                false => None
            }
        })
    .collect()
}

pub(super) fn setup_event_handlers(gui: &Rc<RefCell<MainWindow>>)
{
    let gui_weak = Rc::downgrade( gui );
    let window_handle = gui.borrow().window.handle;

    bind_event_handler( &window_handle, &window_handle, move |evt, evt_data, handle|
    {
        let gui_rc =
        match gui_weak.upgrade()
        {
            Some( rc ) => rc,
            None => return// GUI was dropped
        };

        match evt
        {
            Event::OnButtonClick =>
            {
                let mut gui = gui_rc.borrow_mut();

                if handle == gui.checkbox.handle
                {
                    on_checkbox_toggled( &mut gui );
                    return;
                }

                if let Some( index ) = gui.buttons.iter().position( |b| b.handle == handle )
                {
                    match index
                    {
                        0 => on_create_button( &mut gui ),
                        1 => on_add_button( &mut gui ),
                        2 => on_remove_button( &mut gui ),
                        3 => on_delete_button( &mut gui ),
                        4 => on_cancel_button(),
                        5 => on_help_button(),
                        6 => on_change_folder_button( &mut gui ),
                        _ => { }
                    }
                }
            }

            Event::OnFileDrop =>
            {
                let mut gui = gui_rc.borrow_mut();
                on_file_drop( &mut gui, &evt_data );
            }

            Event::OnListBoxSelect =>
            {
                let mut gui = gui_rc.borrow_mut();

                if handle == gui.listbox_bsp.handle
                {
                    on_bsp_listbox_select( &mut gui );
                }
                else if handle == gui.listbox_cvar.handle
                {
                    on_cvar_listbox_select( &mut gui );
                }
            }

            Event::OnWindowClose => on_window_close(),

            _ => { }
        }
    });
}