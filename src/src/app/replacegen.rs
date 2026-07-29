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
use std::cell::Cell;

use rfd::
{
    FileDialog, 
    MessageButtons,
    MessageDialogResult,
    MessageLevel
};

use slint::
{
    ComponentHandle,
    ModelRc
};

use crate::prelude::*;
use crate::with_controller;

use super::
{
    MainWindow,
    make_row,
    popup
};

#[derive( Default )]
pub(crate) struct Controller
{
    replace_rows: Vec<(String, String)>,
    pending_from: Option<String>
}

thread_local!
{
    static CTRL: Cell<Option<Controller>> = const { Cell::new( None ) };
}

fn sync_ui(rows: &[(String, String)], app: &MainWindow)
{
    let items: Vec<_> = rows.iter()
        .map( |( from, to )| make_row( from, to ) )
    .collect();

    app.set_replacement_rows( ModelRc::from( items.as_slice() ) );
}

impl Controller
{
    pub fn new(app: &MainWindow) -> Self
    {
        app.set_replacement_rows( ModelRc::default() );
        app.set_replace_current_row( -1 );
        // ========== Resource Replacer Callback Bindings ==========
        let app_weak = app.as_weak();
        app.on_load_replacements( move ||
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_load_replacements( app ) );
        });

        let app_weak = app.as_weak();
        app.on_dropped( move |path|
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_dropped( path.as_str(), app ) );
        });

        let app_weak = app.as_weak();
        app.on_add_replacement( move ||
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_add_replacement( app ) );
        });

        let app_weak = app.as_weak();
        app.on_remove_replacement( move ||
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_remove_replacement( app ) );
        });

        let app_weak = app.as_weak();
        app.on_create_replacements( move ||
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_create_replacements( app ) );
        });

        Self::default()
    }

    pub fn register(self)
    {
        CTRL.set( Some( self ) );
    }
    // ========== Resource Replacer Handlers ==========
    fn on_load_replacements(&mut self, app: &MainWindow)
    {
        let Some( file ) = FileDialog::new()
            .add_filter( "Replacements", &[EXT_GMR, EXT_GSR] )
        .pick_file()
        else
        {
            return;
        };

        let Some( replacements ) = Replacement::from_file( &file )
        else
        {
            popup( "Error loading file",
                "Could not load the replacement file. Check that the file exists and has valid entries.",
                MessageLevel::Error, MessageButtons::Ok );

            return;
        };

        if replacements.is_empty()
        {
            popup( "No replacements",
                "The file contains no valid replacements.",
                MessageLevel::Warning, MessageButtons::Ok );

            return;
        }

        self.replace_rows.clear();

        for r in &replacements
        {
            self.replace_rows.push( ( r.get_original().to_string(), r.get_new().to_string() ) );
        }

        sync_ui( &self.replace_rows, app );
    }

    fn on_dropped(&mut self, path: &str, app: &MainWindow)
    {
        if path.has_extension( &[EXT_GMR, EXT_GSR] )
        {
            self.pending_from.take();
            self.on_load_replacements( app );

            return;
        }

        if Replacement::classify( path ).is_none()
        {
            popup( "Invalid file", 
                "The dropped file does not have a recognized model or sound extension.", 
                MessageLevel::Warning, MessageButtons::Ok );

            return;
        }

        match self.pending_from.take()
        {
            None =>// Nothing was dragged here before, this is the first time i.e original
            {
                let path = path.to_string();
                let confirm = 
                popup( "File stored", &format!( "Now drop the replacement file for:\n{path}" ),
                    MessageLevel::Info, MessageButtons::OkCancel );
                
                if confirm == MessageDialogResult::Ok
                {
                    self.pending_from = Some( path );
                }
            }

            Some( from ) =>// Something was already dragged before, this is the replacement
            {
                if path == from
                {
                    popup( "Same file",
                        &format!( "The selected replacement file '{path}' is the same as the original.\nPlease select a different file." ),
                        MessageLevel::Warning,
                        MessageButtons::Ok );

                    return;
                }

                match ( Replacement::classify( &from ), Replacement::classify( path ) )
                {
                    ( Some( Replacement::Models { .. } ), Some( Replacement::Models { .. } ) ) |
                    ( Some( Replacement::Sounds { .. } ), Some( Replacement::Sounds { .. } ) ) =>
                    {
                        self.replace_rows.push( ( Replacement::truncate_path( &from ).to_string(), Replacement::truncate_path( path ).to_string() ) );
                        sync_ui( &self.replace_rows, app );
                    }

                    _ =>
                    {
                        popup( "Type mismatch", 
                            "'From' and 'To' must both be models or both be sounds.", 
                            MessageLevel::Warning, MessageButtons::Ok );

                        self.pending_from = Some( from );
                    }
                }
            }
        }
    }

    fn on_add_replacement(&mut self, app: &MainWindow)
    {
        let mut extensions = EXTS_MODELS.to_vec();
        extensions.extend_from_slice( &EXTS_SOUNDS );
        
        let from =
        match FileDialog::new()
            .set_title( "Select original model/sound" )
            .add_filter( "Models/Sounds", &extensions )
        .pick_file()
        {
            Some( path ) => path.to_string_lossy().into_owned(),
            None => return
        };
        // Constrain "to" file type towards the "from" file type
        let extensions = 
        if let Some( types ) = Replacement::classify( &from )
        {
            match types
            {
                Replacement::Models { .. } => EXTS_MODELS.to_vec(),
                Replacement::Sounds { .. } => EXTS_SOUNDS.to_vec()
            }
        }
        else
        {
            popup( "Invalid file",
                "The selected file does not have a recognized model or sound extension.",
                MessageLevel::Warning, MessageButtons::Ok );

            return;
        };

        let to = 
        loop
        {
            let Some( path ) = FileDialog::new()
                .set_title( "Select replacement file" )
                .add_filter( "", &extensions )
            .pick_file()
            else
            {
                return;
            };

            let path_str = path.to_string_lossy().into_owned();

            if path_str != from
            {
                break path_str;
            }

            popup( "Same file", 
                &format!( "The selected replacement file '{path_str}' is the same as the original.\nPlease select a different file." ),
                MessageLevel::Warning,
                MessageButtons::Ok );
        };

        if Replacement::classify( &to ).is_none()
        {
            popup( "Invalid file", 
                "The selected file does not have a recognized model or sound extension.",
                MessageLevel::Warning, MessageButtons::Ok );

            return;
        }

        match ( Replacement::classify( &from ), Replacement::classify( &to ) )
        {
            ( Some( Replacement::Models { .. } ), Some( Replacement::Models { .. } ) ) |
            ( Some( Replacement::Sounds { .. } ), Some( Replacement::Sounds { .. } ) ) =>
            {
                self.replace_rows.push( ( Replacement::truncate_path( &from ).to_string(), Replacement::truncate_path( &to ).to_string() ) );
                sync_ui( &self.replace_rows, app );
            }

            _ =>
            {
                popup( "Type mismatch", "'From' and 'To' must both be models or both be sounds.", 
                    MessageLevel::Warning, 
                    MessageButtons::Ok );
            }
            
        }
    }
    /// Removes a replacement entry
    fn on_remove_replacement(&mut self, app: &MainWindow)
    {
        let row = app.get_replace_current_row();

        if self.replace_rows.is_empty()
        {
            return;
        }

        if row >= 0 && ( row as usize ) < self.replace_rows.len()
        {
            self.replace_rows.remove( row as usize );
            app.set_replace_current_row( -1 );
            sync_ui( &self.replace_rows, app );
        }
        else// Remove all replacements
        {
            let confirm_remove = popup( "Remove all replacements", 
                "Are you sure you want to remove all replacements?\nThis cannot be undone.", 
                MessageLevel::Warning, MessageButtons::YesNo );

            if confirm_remove == MessageDialogResult::Yes
            {
                self.replace_rows.clear();
                sync_ui( &self.replace_rows, app );
            }
        }
    }

    fn on_create_replacements(&self, _app: &MainWindow)
    {
        if self.replace_rows.is_empty()
        {
            popup( "No replacements", 
                "Add at least one replacement pair.", 
                MessageLevel::Warning, MessageButtons::Ok );
            return;
        }

        let mut replacements: Vec<_> = vec![];
        for ( from, to ) in &self.replace_rows
        {
            if let Some( r ) = Replacement::try_new( from, to ) && !r.is_redundant()
            {
                replacements.push( r );
            }
        }

        let ( models, sounds ) = Replacement::partition_replacements( &replacements );

        if models.is_empty() && sounds.is_empty()
        {
            return;
        }

        let mut errors = vec![];

        if !models.is_empty() 
        && let Some( save_path ) = FileDialog::new().add_filter( "Save as GMR", &[EXT_GMR] ).save_file()
        {
            let gmr_filename = save_path.with_extension( "" ).to_string_lossy().to_string();
            if let Err( e ) = Replacement::to_file( &gmr_filename, &models )
            { 
                errors.push( format!( "Model: {e}" ) );
            }
        }

        if !sounds.is_empty() 
        && let Some( save_path ) = FileDialog::new().add_filter( "Save as GSR", &[EXT_GSR] ).save_file()
        {
            let gsr_filename = save_path.with_extension( "" ).to_string_lossy().to_string();
            if let Err( e ) = Replacement::to_file( &gsr_filename, &sounds )
            { 
                errors.push( format!( "Sound: {e}" ) );
            }
        }

        let ( title, msg, level ) =
        if errors.is_empty()
        {
            ( "Done", "Replacements created successfully.".to_owned(), MessageLevel::Info )
        }
        else
        {
            ( "Error", format!( "Failed:\n{}", errors.join( "\n" ) ), MessageLevel::Error )
        };

        popup( title, &msg, level, MessageButtons::Ok );
    }
}
