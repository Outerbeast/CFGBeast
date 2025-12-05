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

use nwg::*;
use std::
{
    env,
    rc::Rc,
    cell::RefCell
};

use crate::cfg;

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
    pub window: Window,
    pub label: [Label; 2],
    pub buttons: [Button; 6],
    pub textbox: TextBox,
    pub listbox_cvar: ListBox<String>,
    pub listbox_bsp: ListBox<String>,
    pub checkbox: CheckBox,
}

pub fn message_box(title: &str, content: &str, buttons: MessageButtons, icons: MessageIcons) -> MessageChoice
{
    nwg::message( &MessageParams
    {
        title,
        content,
        buttons,
        icons,
    })
}
// All fugly boilerplate business for building the GUI
pub fn build_main_window() -> Rc<RefCell<MainWindow>>
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
            .title( crate::APPNAME )
            .flags( WindowFlags::WINDOW | WindowFlags::VISIBLE )
        .build( &mut app_mut.window ).unwrap_or_default();

        Label::builder()
            .text( "BSPs:" )
            .parent( &app_mut.window )
            .position( ( 10, 12 ) )
            .size( ( 100, 25 ) )
        .build( &mut app_mut.label[0] ).unwrap_or_default();
        
        Label::builder()
            .text( "Input CVars:" )
            .parent( &app_mut.window )
            .position( ( 180, 12 ) )
            .size( ( 300, 25 ) )
        .build( &mut app_mut.label[1] ).unwrap_or_default();
        // Textbox
        TextBox::builder()
            .text( "" )
            .parent( &app_mut.window )
            .position( ( 180, 40 ) )
            .size( TEXTBOX_SIZE )
            .flags( TextBoxFlags::VISIBLE )
        .build( &mut app_mut.textbox ).unwrap_or_default();
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
        .build( &mut app_mut.listbox_bsp ).unwrap_or_default();
        // CVar Listbox
        ListBox::builder()
            .collection( cfg::get_default_cvars() )
            .size( CVAR_LIST_SIZE ) // visible area; scrollbar appears if items overflow
            .position(( 520, 40 ) )
            .parent( &app_mut.window )
            .flags( nwg::ListBoxFlags::VISIBLE )
        .build( &mut app_mut.listbox_cvar ).unwrap_or_default();

        CheckBox::builder()
            .text( "Skill CFG" )
            .size( ( 120, 25 ) )
            .position( ( 520, 10 ) )
            .parent( &app_mut.window )
            .flags( nwg::CheckBoxFlags::VISIBLE )
            .check_state( nwg::CheckBoxState::Unchecked )
        .build( &mut app_mut.checkbox ).unwrap_or_default();
        // Buttons
        Button::builder()
            .text( "Create" )
            .parent( &app_mut.window )
            .position(( 180, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[0] ).unwrap_or_default();

        Button::builder()
            .text( "Add" )
            .parent( &app_mut.window )
            .position(( 275, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[1] ).unwrap_or_default();

        Button::builder()
            .text( "Remove" )
            .parent( &app_mut.window )
            .position( ( 370, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[2] ).unwrap_or_default();

        Button::builder()
            .text( "Delete" )
            .parent( &app_mut.window )
            .position( ( 670, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[3] ).unwrap_or_default();

        Button::builder()
            .text( "Cancel" )
            .parent( &app_mut.window )
            .position( ( 765, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[4] ).unwrap_or_default();

        Button::builder()
            .text( "?" )
            .parent( &app_mut.window )
            .position(( 820, 5 ) )
            .size( ( 30, 30 ) )
        .build( &mut app_mut.buttons[5] ).unwrap_or_default();
    }

    window
}

pub fn show_wait_splash() -> nwg::Window
{
    nwg::init().unwrap();

    let mut splash = nwg::Window::default();
    nwg::Window::builder()
        .size( ( 200, 0 ) )
        .position( ( nwg::Monitor::width() / 2 - 150, nwg::Monitor::height() / 2 - 50 ) )
        .title( "Initial setup, please wait..." ) // no title bar text
        .flags
        (
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

