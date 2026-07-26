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
/// Takes the controller out of the thread-local Cell, runs the body, puts it back.
/// Use inside a Slint callback closure that captured `app_weak`.
#[macro_export] macro_rules! with_ctrl
{
    ( $weak:expr, $handle:ident, | $ctrl:ident | $body:expr ) =>
    {
        {
            let Some( $handle ) = $weak.upgrade()
            else
            {
                return;
            };
            CTRL.with( |c|
            {
                let Some( mut $ctrl ) = c.take()
                else
                {
                    return;
                };

                $body;
                c.set( Some( $ctrl ) );
            });
        }
    };
}

mod cfggen;
mod materialsgen;
mod replacegen;

use std::
{
    path::Path,
    sync::mpsc,
    time::Duration,
    thread,
};

use rfd::
{
    MessageButtons,
    MessageDialog,
    MessageDialogResult,
    MessageLevel
};

slint::include_modules!();
use slint::
{
    ComponentHandle,
    Model,
    ModelRc,
    PlatformError,
    SharedString,
    StandardListViewItem,
    winit_030::
    {
        CustomApplicationHandler,
        EventResult,
        WinitWindowAccessor,
        winit::
        {
            self as winit,
            dpi::PhysicalPosition,
            event::WindowEvent,
            event_loop::ActiveEventLoop,
            window::WindowId
        }
    }
};

use crate::prelude::*;

pub const CHECKED: &str = "✔";
pub const UNCHECKED: &str = "☐";

/// Shows a popup message dialog with the given title, description, returns MessageDialogResult of what the user clicked.
/// Blocks the calling thread until the dialog is dismissed. The dialog runs on its own OS thread
/// so Slint's event loop continues rendering in the background.
pub fn popup(title: &str, message: &str, level: MessageLevel, buttons: MessageButtons) -> MessageDialogResult
{
    let (title, desc) = ( title.to_string(), message.to_string() );
    let (tx, rx) = mpsc::channel();

    thread::spawn( move ||
    {
        let result = MessageDialog::new()
            .set_title( &title )
            .set_description( &desc )
            .set_level( level )
            .set_buttons( buttons )
        .show();

        let _ = tx.send( result );
    });

    rx.recv().unwrap_or( MessageDialogResult::Cancel )
}

pub fn load_cvar_presets(is_skill: bool) -> Vec<StandardListViewItem>
{
    let mut cvars =
    if is_skill
    {
        get_skill_cvars().clone()
    }
    else
    {
        get_default_cvars().clone()
    };

    let custom_cvars = &Config::get().additional_cvars;
    if !custom_cvars.is_empty()
    {
        cvars.extend( custom_cvars.iter().cloned() );
        cvars.sort();
        cvars.dedup();
    }

    cvars
        .iter()
        .map( |c| StandardListViewItem::from( c.as_str() ) )
    .collect()
}

fn current_bsp_whitelist(ui: &MainWindow) -> Vec<String>
{
    let mut whitelist = vec![];
    let items = ui.get_bsp_items();

    for item in items.iter()
    {
        if item.starts_with( CHECKED )
        {
            whitelist.push( item[CHECKED.len() + 1..].to_string() );
        }
    }

    whitelist
}

fn collect_bsp_items(bsp_path: &Path) -> Vec<SharedString>
{
    load_bsps( bsp_path )
        .iter()
        .filter_map( |p| p.file_name() )
        .map( |n| SharedString::from( format!( "{CHECKED} {}", n.to_string_lossy().as_ref() ) ) )
    .collect()
}

fn make_row(from: &str, to: &str) -> ModelRc<StandardListViewItem>
{
    ModelRc::from
    ([
        StandardListViewItem::from( from ),
        StandardListViewItem::from( to )
    ])
}

type DropEvent = (bool, String);

struct DragDropHandler
{
    sender: mpsc::Sender<DropEvent>
}

impl DragDropHandler
{   /// Register winit backend. Call BEFORE MainWindow::new().
    pub fn new() -> Result<(Self, mpsc::Receiver<DropEvent>), PlatformError>
    {
        let (sender, receiver) = mpsc::channel();

        slint::BackendSelector::new()
            .backend_name( "winit".into() )
            .with_winit_custom_application_handler( Self { sender: sender.clone() } )
        .select()?;

        Ok( ( Self { sender }, receiver ) )
    }
    /// Start polling the drop queue. Call AFTER MainWindow::new().
    /// Returns the Timer — must be kept alive for the duration of the app.
    pub fn start_polling(receiver: mpsc::Receiver<DropEvent>, app: &MainWindow) -> slint::Timer
    {
        let app_weak = app.as_weak();
        let timer = slint::Timer::default();
        // Have to poll the queue every 33ms to see if there are any new drag-n-drop events.
        timer.start( slint::TimerMode::Repeated, Duration::from_millis( 33 ), move ||
        {
            let Some( app ) = app_weak.upgrade()
            else
            {
                return;
            };

            let current_tab = app.get_current_tab();

            for ( hovering, path ) in receiver.try_iter()
            {
            if hovering
            {
                match current_tab
                {
                    0 => app.set_cfg_is_dragging( true ),
                    2 => app.set_material_is_dragging( true ),
                    _ => app.set_replace_is_dragging( true )
                }
            }
            else if path.is_empty()
            {
                app.set_cfg_is_dragging( false );
                app.set_replace_is_dragging( false );
                app.set_material_is_dragging( false );
            }
            else
            {
                app.set_cfg_is_dragging( false );
                app.set_replace_is_dragging( false );
                app.set_material_is_dragging( false );

                match current_tab
                {
                    0 => app.invoke_cfg_dropped( path.into() ),
                    2 => app.invoke_material_dropped( path.into() ),
                    _ => app.invoke_dropped( path.into() )
                }
                }
            }
        });
        timer
    }
}

impl CustomApplicationHandler for DragDropHandler
{   // Event handler for drag-and-drop functionality
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _winit_window: Option<&winit::window::Window>,
        _slint_window: Option<&slint::Window>,
        event: &WindowEvent,
    ) -> EventResult
    {
        match event
        {
            WindowEvent::HoveredFile( path ) => { let _ = self.sender.send( ( true, path.to_string_lossy().into_owned() ) ); }
            WindowEvent::DroppedFile( path ) => { let _ = self.sender.send( ( false, path.to_string_lossy().into_owned() ) ); }
            WindowEvent::HoveredFileCancelled => { let _ = self.sender.send( ( false, String::new() ) ); }
            _ => { }
        }

        EventResult::Propagate
    }
}
/// Set up event controllers and launch the app window.
/// Window position is also defaulted to the centre of the screen.
pub fn launch_gui() -> Result<(), PlatformError>
{
    let (_handler, drop_rx) = DragDropHandler::new()?;
    let app = MainWindow::new()?;
    let _timer = DragDropHandler::start_polling( drop_rx, &app );
    cfggen::setup( &app );
    replacegen::setup( &app );
    materialsgen::setup( &app );

    let app_weak = app.as_weak();
    // Position window in the centre of the screen
    slint::spawn_local( async move
    {
        let app = app_weak.unwrap();
        
        if let Ok( winit_window ) = app.window().winit_window().await
        && let Some( monitor ) = winit_window.primary_monitor()
        {
            let monitor_size = monitor.size();
            let window_size = winit_window.inner_size();
            let x = ( monitor_size.width as i32 - window_size.width as i32 ) / 2;
            let y = ( monitor_size.height as i32 - window_size.height as i32 ) / 2;
            
            winit_window.set_outer_position( PhysicalPosition::new( x, y ) );
        }
    }).map_err( |e|
    match e
    {
        slint::EventLoopError::NoEventLoopProvider => PlatformError::NoEventLoopProvider,
        _ => PlatformError::Other( e.to_string() )
    })?;

    app.run()
}
