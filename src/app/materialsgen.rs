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
use std::
{
    cell::Cell,
    path::Path,
};

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
    ModelRc,
    StandardListViewItem
};

use super::
{
    MainWindow,
    make_row,
    popup
};

use crate::with_controller;

use crate::prelude::*;

#[derive( Default )]
pub(crate) struct Controller
{
    material_rows: Vec<(String, String)>,
    wad_textures: Vec<String>
}

thread_local!
{
    static CTRL: Cell<Option<Controller>> = const { Cell::new( None ) };
}

fn sync_materials(rows: &[(String, String)], app: &MainWindow)
{
    let items: Vec<_> = rows.iter()
        .map( |( texture, kind )| make_row( texture, kind ) )
    .collect();

    app.set_material_rows( ModelRc::from( items.as_slice() ) );
}

fn sync_wad_textures(textures: &[String], app: &MainWindow)
{
    let items: Vec<_> = textures.iter()
        .map( |n| StandardListViewItem::from( n.as_str() ) )
    .collect();

    app.set_material_wad_textures( ModelRc::from( items.as_slice() ) );
}

impl Controller
{
    pub fn new(app: &MainWindow) -> Self
    {
        app.set_material_rows( ModelRc::default() );
        app.set_material_current_row( -1 );
        app.set_material_selected_kind( 0 );
        app.set_material_wad_textures( ModelRc::default() );
        // ========== Materials Replacer Callback Bindings ==========
        let app_weak = app.as_weak();
        app.on_load_materials( move ||
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_load_materials( app ) );
        });

        let app_weak = app.as_weak();
        app.on_material_dropped( move |path|
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_dropped( app, path.as_str() ) );
        });

        let app_weak = app.as_weak();
        app.on_load_wad( move ||
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_load_wad( app ) );
        });

        let app_weak = app.as_weak();
        app.on_wad_texture_selected( move |idx|
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_wad_texture_selected( app, idx ) );
        });

        let app_weak = app.as_weak();
        app.on_kind_changed( move |idx|
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_kind_changed( app, idx ) );
        });

        let app_weak = app.as_weak();
        app.on_remove_material( move ||
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_remove_material( app ) );
        });

        let app_weak = app.as_weak();
        app.on_create_materials( move ||
        {
            with_controller!( app_weak, CTRL, |ctrl, app| ctrl.on_create_materials( app ) );
        });

        Self::default()
    }

    pub fn register(self)
    {
        CTRL.set( Some( self ) );
    }

    fn populate_materials(&mut self, entries: &[MaterialEntry])
    {
        for e in entries
        {
            self.material_rows.push( ( e.texture.clone().unwrap_or_default(), e.kind.to_string() ) );
        }
    }

    fn on_dropped(&mut self, app: &MainWindow, path: &str)
    {
        if path.has_extension( &["txt"] )
        {
            if let Some( entries ) = MaterialEntry::from_file( Path::new( path ) )
            {
                if entries.is_empty()
                {
                    popup( "No entries", "The file contains no valid material entries.",
                        MessageLevel::Error, MessageButtons::Ok );
                    return;
                }

                self.material_rows.clear();
                self.populate_materials( &entries );
                sync_materials( &self.material_rows, app );
            }
            else
            {
                popup( "Error loading file", "Could not load the material file. Check that the file exists and has valid entries.",
                    MessageLevel::Error, MessageButtons::Ok );
            }

            return;
        }

        if path.has_extension( &["wad"] )
        {
            match read_texture_names( path )
            {
                Ok( names ) =>
                {
                    self.wad_textures = names;
                    sync_wad_textures( &self.wad_textures, app );

                    let filename = Path::new( path ).file_name()
                        .map( |n| n.to_string_lossy().to_string() )
                        .unwrap_or_default();
                    app.set_material_wad_button_text( format!( "Set WAD: {filename}" ).into() );
                }

                Err( e ) =>
                {
                    popup( "Error loading WAD", &format!( "Could not read texture names from WAD.\nReason: {e}" ),
                        MessageLevel::Error, MessageButtons::Ok );
                }
            }

            return;
        }

        popup( "Invalid file",
            "The dropped file is not a recognized format (.txt, .wad).",
            MessageLevel::Warning, MessageButtons::Ok );
    }

    fn on_load_materials(&mut self, app: &MainWindow)
    {
        let Some( file ) = FileDialog::new().add_filter( "Material Files", &["txt"] ).pick_file()
        else
        {
            return;
        };

        let Some( entries ) = MaterialEntry::from_file( &file )
        else
        {
            popup( "Error loading file", "Could not load the material file. Check that the file exists and has valid entries.", 
                MessageLevel::Error, MessageButtons::Ok );

            return;
        };

        if entries.is_empty()
        {
            popup( "No entries", "The file contains no valid material entries.", 
                MessageLevel::Error, MessageButtons::Ok );

            return;
        }

        self.material_rows.clear();
        self.populate_materials( &entries );
        sync_materials( &self.material_rows, app );
    }

    fn on_load_wad(&mut self, app: &MainWindow)
    {
        let Some( file ) = FileDialog::new().add_filter( "WAD Files", &["wad"] ).pick_file()
        else
        {
            return;
        };

        match read_texture_names( &file )
        {
            Ok( names ) =>
            {
                self.wad_textures = names;
                sync_wad_textures( &self.wad_textures, app );

                let filename = file
                    .file_name()
                    .map( |n| n.to_string_lossy().to_string() )
                .unwrap_or_default();

                app.set_material_wad_button_text( format!( "Set WAD: {filename}" ).into() );
            }

            Err( e ) =>
            {
                popup( "Error loading WAD", &format!( "Could not read texture names from WAD.\nReason: {e}" ),
                    MessageLevel::Error, MessageButtons::Ok );
            }
        }
    }

    fn on_wad_texture_selected(&mut self, app: &MainWindow, idx: i32)
    {
        if idx < 0 || ( idx as usize ) >= self.wad_textures.len()
        {
            return;
        }

        let kind = MaterialKind::index_to_kind( app.get_material_selected_kind() ).to_string();
        let texture = self.wad_textures[ idx as usize ].clone();

        self.material_rows.push( ( texture, kind ) );
        sync_materials( &self.material_rows, app );
    }

    fn on_kind_changed(&mut self, app: &MainWindow, kind_idx: i32)
    {
        let row = app.get_material_current_row();

        if row < 0 || ( row as usize ) >= self.material_rows.len()
        {
            return;
        }

        self.material_rows[row as usize].1 = MaterialKind::index_to_kind( kind_idx ).to_string();
        sync_materials( &self.material_rows, app );
    }

    fn on_remove_material(&mut self, app: &MainWindow)
    {
        let row = app.get_material_current_row();

        if row >= 0 && ( row as usize ) < self.material_rows.len()
        {
            self.material_rows.remove( row as usize );
            app.set_material_current_row( -1 );
            sync_materials( &self.material_rows, app );
        }
        else
        {
            let confirm_remove = popup( "Remove all entries", 
                "Are you sure you want to remove all material entries?\nThis cannot be undone.", 
                MessageLevel::Warning, MessageButtons::YesNo );

            if confirm_remove == MessageDialogResult::Yes
            {
                self.material_rows.clear();
                sync_materials( &self.material_rows, app );
            }
        }
    }

    fn on_create_materials(&self, _app: &MainWindow)
    {
        if self.material_rows.is_empty()
        {
            popup( "No entries", "Add at least one material entry.", 
                MessageLevel::Warning, MessageButtons::Ok );

            return;
        }

        let mut entries = vec![];
        for ( texture, kind_str ) in &self.material_rows
        {
            let texture =
            if texture.trim().is_empty() 
            { 
                None
            } 
            else 
            { 
                Some( texture.clone() )
            };

            entries.push( MaterialEntry::new( kind_str.parse::<_>().unwrap_or( MaterialKind::Concrete ), texture ) );
        }

        if entries.is_empty()
        {
            return;
        }

        if let Some( save_path ) = FileDialog::new().add_filter( "Save Materials file", &["txt"] ).save_file()
        {
            let filename = save_path.with_extension( "" ).to_string_lossy().to_string();
            match MaterialEntry::to_file( &filename, &entries )
            {
                Ok( () ) =>
                {
                    popup( "Done", &format!( "Material file '{filename}.txt' created successfully." ), 
                        MessageLevel::Info, MessageButtons::Ok );
                }

                Err( e ) =>
                {
                    popup( "Error", &format!( "Failed to save material '{filename}.txt'.\nReason: {e}" ), 
                        MessageLevel::Error, MessageButtons::Ok );
                }
            }
        }
    }
}
