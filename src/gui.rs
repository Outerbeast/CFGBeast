use std::env;
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
use std::rc::Rc;
use std::{cell::RefCell};
use native_windows_gui as nwg;
use nwg::*;

use crate::cfg::{self, create_cfg, WriteType};

pub const APPNAME: &str = "CFGBeast";
const WINDOW_SIZE: (i32, i32) = ( 860, 440 );
const BUTTON_SIZE: (i32, i32) = ( 85, 30 );
const TEXTBOX_SIZE: (i32, i32) = ( 330, 338 );
const CVAR_LIST_SIZE: (i32, i32) = ( 330, 350 );
const BSP_LIST_SIZE: (i32, i32) = ( 160, 350 );
pub const UNCHECKED: &str = "☐";
pub const CHECKED:   &str = "✔";

#[derive(Default)]
pub struct MainWindow
{
    window: Window,
    label: [Label; 2],
    buttons: [Button; 6],
    textbox: TextBox,
    listbox_cvar: ListBox<String>,
    listbox_bsp: ListBox<String>,
    checkbox: CheckBox,
}

pub fn message_box(title: &str, body: &str, buttons: MessageButtons, icons: MessageIcons) -> MessageChoice
{
    let choice = nwg::message( &MessageParams
    {
        title: title,
        content: body,
        buttons: buttons,
        icons: icons,
    });

    choice
}
// All fugly boilerplate business for building the GUI
fn build_main_window() -> Rc<RefCell<MainWindow>>
{
    let window = Rc::new( RefCell::new( MainWindow::default() ) );
    let bsp_paths = cfg::load_bsps( env::current_dir().unwrap() );
    let bsp_filenames: Vec<String> = bsp_paths.iter()
        .filter_map( |p| p.file_name() )
        .map( |name| name.to_string_lossy().into_owned() )
        .collect();
    
    {
        let mut app_mut = window.borrow_mut();

        Window::builder()
            .size( ( WINDOW_SIZE.0, WINDOW_SIZE.1 ) )
            .position(
        {
                let center_x = ( Monitor::width() - WINDOW_SIZE.0 ) / 2;
                let center_y = ( Monitor::height() - WINDOW_SIZE.1 ) / 2;

                ( center_x, center_y )
            })
            .title( APPNAME )
            .flags( WindowFlags::WINDOW | WindowFlags::VISIBLE )
        .build( &mut app_mut.window ).unwrap();

        Label::builder()
            .text( "BSPs:" )
            .parent( &app_mut.window )
            .position( ( 10, 12 ) )
            .size( ( 100, 25 ) )
        .build( &mut app_mut.label[0] ).unwrap();
        
        Label::builder()
            .text( "Input CVars:" )
            .parent( &app_mut.window )
            .position( ( 180, 12 ) )
            .size( ( 300, 25 ) )
        .build( &mut app_mut.label[1] ).unwrap();
        // Textbox
        TextBox::builder()
            .text( "" )
            .parent( &app_mut.window )
            .position( ( 180, 40 ) )
            .size( TEXTBOX_SIZE )
            .flags( TextBoxFlags::VISIBLE )
        .build( &mut app_mut.textbox ).unwrap();
        // BSP Listbox
        let display_names: Vec<String> = bsp_filenames
            .iter()
            .map( |name| format!( "{CHECKED}\t{name}" ) )
        .collect();

        ListBox::builder()
            .collection( display_names )
            .size( BSP_LIST_SIZE )
            .position(( 10, 40 ) )
            .parent( &app_mut.window )
            .flags( nwg::ListBoxFlags::VISIBLE ) // ensure it's interactive (not DISABLED)
        .build( &mut app_mut.listbox_bsp ).unwrap();
        // CVar Listbox
        ListBox::builder()
            .collection( cfg::get_default_cvars() )
            .size( CVAR_LIST_SIZE ) // visible area; scrollbar appears if items overflow
            .position(( 520, 40 ) )
            .parent( &app_mut.window )
            .flags( nwg::ListBoxFlags::VISIBLE )
        .build( &mut app_mut.listbox_cvar ).unwrap();

        CheckBox::builder()
            .text( "Skill CFG" )
            .size( ( 120, 25 ) )
            .position( ( 520, 10 ) )
            .parent( &app_mut.window )
            .flags( nwg::CheckBoxFlags::VISIBLE )
            .check_state( nwg::CheckBoxState::Unchecked )
        .build( &mut app_mut.checkbox ).expect( "Failed to build checkbox" );
        // Buttons
        Button::builder()
            .text( "Create" )
            .parent( &app_mut.window )
            .position(( 180, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[0] ).unwrap();

        Button::builder()
            .text( "Add" )
            .parent( &app_mut.window )
            .position(( 275, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[1] ).unwrap();

        Button::builder()
            .text( "Remove" )
            .parent( &app_mut.window )
            .position( ( 370, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[2] ).unwrap();

        Button::builder()
            .text( "Delete" )
            .parent( &app_mut.window )
            .position( ( 670, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[3] ).unwrap();

        Button::builder()
            .text( "Cancel" )
            .parent( &app_mut.window )
            .position(( 765, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[4] ).unwrap();

        Button::builder()
            .text( "?" )
            .parent( &app_mut.window )
            .position(( 820, 5 ) )
            .size( ( 30, 30 ) )
        .build( &mut app_mut.buttons[5] ).unwrap();
    }

    window
}

pub fn GUI()
{
    nwg::init().unwrap();
    let gui = build_main_window();
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
                    show_help();
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

                        items[idx] = if current.starts_with( CHECKED )
                        {
                            format!( "{UNCHECKED}\t{}", &current[CHECKED.len() + 1..] )
                        }
                        else
                        {
                            format!( "{CHECKED}\t{}", &current[UNCHECKED.len() + 1..] )
                        };
                        // Replace the collection in one go
                        gui.listbox_bsp.set_collection(items);
                    }
                }
                else if handle == gui.listbox_cvar.handle
                {
                    if let Some( idx ) = gui.listbox_cvar.selection()
                    {
                        if let Some( selected ) = gui.listbox_cvar.collection().get( idx )
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

pub fn show_wait_splash() -> nwg::Window
{
    nwg::init().unwrap();

    let mut splash = nwg::Window::default();
    nwg::Window::builder()
        .size( ( 200, 0 ) )
        .position( ( nwg::Monitor::width() / 2 - 150, nwg::Monitor::height() / 2 - 50 ) )
        .title( "Initial setup, please wait..." ) // no title bar text
        .flags(
            nwg::WindowFlags::WINDOW
            | nwg::WindowFlags::VISIBLE
            | nwg::WindowFlags::POPUP, // no system menu, no buttons
        )
    .build( &mut splash ).unwrap();
    // !-UNDONE-!: Label doesn't show up for some reason
/*     let mut label = nwg::Label::default();
    nwg::Label::builder()
        .text( "Doing initial setup, please wait…" )
        .parent( &splash )
        .position( ( 20, 40 ) )
        .size( ( 260, 20 ) )
    .build( &mut label ).unwrap(); */

    splash
}


fn show_help()
{
    message_box( "Help", 
    "This is a simple application to create, append, remove, and delete CFG files based on the BSP files in the current directory.\n\n
        1. Enter the cvars you want to manage in the text box, by selecting cvar presets or typing them in.\n
        2. Click 'Create' to create or overwrite CFG files.\n
        3. Click 'Add' to append cvars to existing CFG files.\n
        4. Click 'Remove' to remove specified cvars from CFG files.\n
        5. Click 'Delete' to delete all CFG files in the current directory.\n
        6. Click 'Cancel' or 'x' to exit the application.\n\n
        Thank you for using this app!\nIf you'd like to give feedback feel free to put them here: https://github.com/Outerbeast/CFGBeast/issues", 
        MessageButtons::Ok, 
        MessageIcons::Question );
}
