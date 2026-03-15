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
    cell::RefCell,
    path::{ Path, PathBuf },
    rc::Rc
};

use native_windows_gui::
{
    init,
    Button,
    CheckBox,
    CheckBoxFlags,
    CheckBoxState,
    Label,
    ListBox,
    ListBoxFlags,
    Monitor,
    TextBox,
    TextBoxFlags,
    Window,
    WindowFlags
};

use crate::{ alloc_shared, cvar };

use super::
{
    BSP_LIST_SIZE,
    TEXTBOX_SIZE,
    WINDOW_SIZE,
    BUTTON_SIZE,
    CHECKED,
    CVAR_LIST_SIZE
};

#[derive(Default)]
pub struct MainWindow
{
    pub window: Window,
    pub label: [Label; 2],
    pub buttons: [Button; 7],
    pub textbox: TextBox,
    pub listbox_cvar: ListBox<String>,
    pub listbox_bsp: ListBox<String>,
    pub checkbox: CheckBox,
    // The current directory containing the BSP files
    pub bsp_dir: PathBuf,
}

pub fn show_wait_splash() -> Window
{
    init().ok();

    let mut splash = Window::default();
    Window::builder()
        .size( ( 200, 0 ) )
        .position( ( Monitor::width() / 2 - 150, Monitor::height() / 2 - 50 ) )
        .title( "Initial setup, please wait..." )// no title bar text
        .flags
        (
            WindowFlags::WINDOW
            | WindowFlags::VISIBLE
            | WindowFlags::POPUP,// no system menu, no buttons
        )
    .build( &mut splash ).ok();
    
    splash
}
// All fugly boilerplate business for building the GUI
pub fn build_main_window(bsp_path: &Path) -> Rc<RefCell<MainWindow>>
{
    let window = alloc_shared!( MainWindow::default() );
    {
        let mut app_mut = window.borrow_mut();
        app_mut.bsp_dir = bsp_path.to_path_buf().clone();
        let bsp_paths = cvar::load_bsps( app_mut.bsp_dir.clone().as_path() );
        
        let bsp_filenames: Vec<String> = bsp_paths.iter()
            .filter_map( |p| p.file_name() )
            .map( |name| name.to_string_lossy().into_owned() )
        .collect();
    
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
            .accept_files( true )
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
            .flags( TextBoxFlags::VISIBLE | TextBoxFlags::VSCROLL )
        .build( &mut app_mut.textbox ).unwrap_or_default();
        // BSP Listbox
        let display_names: Vec<String> = bsp_filenames
            .iter()
            .map( |name| format!( "{CHECKED}\t{name}" ) )
        .collect();

        ListBox::builder()
            .collection( display_names )
            .size( BSP_LIST_SIZE )
            .position( ( 10, 40 ) )
            .parent( &app_mut.window )
            .flags( ListBoxFlags::VISIBLE ) // ensure it's interactive (not DISABLED)
        .build( &mut app_mut.listbox_bsp ).unwrap_or_default();
        // CVar Listbox
        ListBox::builder()
            .collection( cvar::get_default_cvars() )
            .size( CVAR_LIST_SIZE ) // visible area; scrollbar appears if items overflow
            .position( ( 520, 40 ) )
            .parent( &app_mut.window )
            .flags( ListBoxFlags::VISIBLE )
        .build( &mut app_mut.listbox_cvar ).unwrap_or_default();
        // Skill CFG checkbox
        CheckBox::builder()
            .text( "Skill CFG" )
            .size( ( 120, 25 ) )
            .position( ( 520, 10 ) )
            .parent( &app_mut.window )
            .flags( CheckBoxFlags::VISIBLE )
            .check_state( CheckBoxState::Unchecked )
        .build( &mut app_mut.checkbox ).unwrap_or_default();
        // Buttons
        Button::builder()
            .text( "Create" )
            .parent( &app_mut.window )
            .position( ( 180, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[0] ).unwrap_or_default();
        // Add button
        Button::builder()
            .text( "Add" )
            .parent( &app_mut.window )
            .position(( 275, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[1] ).unwrap_or_default();
        // Remove button
        Button::builder()
            .text( "Remove" )
            .parent( &app_mut.window )
            .position( ( 370, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[2] ).unwrap_or_default();
        // Delete button
        Button::builder()
            .text( "Delete" )
            .parent( &app_mut.window )
            .position( ( 670, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[3] ).unwrap_or_default();
        // Change Folder button
        Button::builder()
            .text( "Change" )
            .parent( &app_mut.window )
            .position( ( 10, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[6] ).unwrap_or_default();
        // Cancel button
        Button::builder()
            .text( "Cancel" )
            .parent( &app_mut.window )
            .position( ( 765, 390 ) )
            .size( BUTTON_SIZE )
        .build( &mut app_mut.buttons[4] ).unwrap_or_default();
        // Help button
        Button::builder()
            .text( "?" )
            .parent( &app_mut.window )
            .position( ( 820, 5 ) )
            .size( ( 30, 30 ) )
        .build( &mut app_mut.buttons[5] ).unwrap_or_default();
    }

    window
}