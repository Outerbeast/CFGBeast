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
    fs,
    path::PathBuf,
};

use rfd::
{
    FileDialog,
    MessageButtons,
    MessageLevel
};

use slint::
{
    ComponentHandle,
    Model,
    ModelRc,
    SharedString,
};

use super::
{
    MainWindow,
    CHECKED,
    UNCHECKED,
    popup
};

use crate::
{
    app::
    {
        collect_bsp_items,
        current_bsp_whitelist,
        load_cvar_presets,
    },
    current_dir_path,
    cvar::parse_cfg,
    prelude::*,
    utils
};

pub(crate) struct Controller
{
    bsp_dir: PathBuf,
    default_cvar_cache: SharedString,
    skill_cvar_cache: SharedString
}

thread_local!
{
    static CTRL: Cell<Option<Controller>> = const { Cell::new( None ) };
}

#[allow( unused_mut )]
pub(crate) fn setup(app: &MainWindow)
{
    let bsp_path = current_dir_path!();
    app.set_bsp_items( ModelRc::from( collect_bsp_items( &bsp_path ).as_slice() ) );
    // --- Load CVar presets ---
    app.set_cvar_presets( ModelRc::from( load_cvar_presets( false ).as_slice() ) );
    app.set_cvar_current( -1 );

    CTRL.set( Some( Controller
    {
        bsp_dir: bsp_path,
        default_cvar_cache: SharedString::new(),
        skill_cvar_cache: SharedString::new()
    }));
    // ========== CFG Generator Callback Bindings ==========
    let app_weak = app.as_weak();
    app.on_load_cfg( move ||
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.on_load_cfg( &app ) );
    });

    let app_weak = app.as_weak();
    app.on_cfg_dropped( move |path|
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.on_dropped( path.as_str(), &app ) );
    });

    let app_weak = app.as_weak();
    app.on_bsp_toggled( move |idx|
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.on_bsp_toggled( &app, idx ) );
    });

    let app_weak = app.as_weak();
    app.on_cvar_selected( move |idx|
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.on_cvar_selected( &app, idx ) );
    });

    let app_weak = app.as_weak();
    app.on_skill_cfg_changed( move ||
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.on_skill_cfg_changed( &app ) );
    });

    let app_weak = app.as_weak();
    app.on_change_folder( move ||
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.on_change_folder( &app ) );
    });

    let app_weak = app.as_weak();
    app.on_create_cfg( move ||
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.write_cfg( &app, WriteType::OVERWRITE ) );
    });

    let app_weak = app.as_weak();
    app.on_add_cfg( move ||
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.write_cfg( &app, WriteType::APPEND ) );
    });

    let app_weak = app.as_weak();
    app.on_remove_cfg( move ||
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.write_cfg( &app, WriteType::REMOVE ) );
    });

    let app_weak = app.as_weak();
    app.on_delete_cfg( move ||
    {
        with_ctrl!( app_weak, app, |ctrl| ctrl.write_cfg( &app, WriteType::DELETE ) );
    });
}
/// ========== CFG Generator Handlers ==========
impl Controller
{
        fn on_load_cfg(&self, app: &MainWindow)
    {
        let Some( selected ) = FileDialog::new()
            .set_title( "Select a CFG file" )
            .add_filter( "CFG files", &["cfg"] )
            .pick_file()
        else
        {
            return;
        };

        let path = selected.to_string_lossy().to_string();
        self.on_dropped( &path, app );
    }

    fn on_dropped(&self, path: &str, app: &MainWindow)
    {
        if !path.has_extension( &["cfg"] )
        {
            popup( "Invalid file",
                "The dropped file is not a .cfg file.",
                MessageLevel::Warning, MessageButtons::Ok );

            return;
        }

        match fs::File::open( path )
        {
            Ok( file ) =>
            {
                let cvars = parse_cfg( file );
                app.set_cvar_text( SharedString::from( cvars.join( "\n" ) ) );
            }
            Err( e ) =>
            {
                popup( "Failed to read file",
                    &format!( "Could not read '{path}':\n{e}" ),
                    MessageLevel::Warning, MessageButtons::Ok );
            }
        }
    }

    fn on_bsp_toggled(&mut self, app: &MainWindow, index: i32)
    {
        let mut vec: Vec<_> = app.get_bsp_items().iter().collect();
        let idx = index as usize;

        if let Some( item ) = vec.get_mut( idx )
        {
            let name = &item[CHECKED.len() + 1..];

            vec[idx] =
            if item.starts_with( CHECKED )
            {
                format!( "{UNCHECKED} {name}" ).into()
            }
            else
            {
                format!( "{CHECKED} {name}" ).into()
            };
        }

        app.set_bsp_items( ModelRc::from( vec.as_slice() ) );
    }

    fn on_cvar_selected(&mut self, app: &MainWindow, index: i32)
    {
        if let Some( item ) = app.get_cvar_presets().iter().nth( index as usize )
        {
            let mut current = app.get_cvar_text();
            if !current.trim().is_empty() && !current.ends_with( '\n' )
            {
                current.push_str( "\n" );
            }

            current.push_str( &item.text );
            app.set_cvar_text( current );
        }

        app.set_cvar_current( -1 );
    }

    fn on_skill_cfg_changed(&mut self, app: &MainWindow)
    {
        let is_skill = app.get_skill_cfg();
        let current_text = app.get_cvar_text();
        // Toggle between regular CVars and skill cvars
        if is_skill
        {
            self.default_cvar_cache = current_text;
            app.set_cvar_text( self.skill_cvar_cache.clone() );
        }
        else
        {
            self.skill_cvar_cache = current_text;
            app.set_cvar_text( self.default_cvar_cache.clone() );
        }

        app.set_cvar_presets( ModelRc::from( load_cvar_presets( is_skill ).as_slice() ) );
        app.set_cvar_current( -1 );
    }

    fn on_change_folder(&mut self, app: &MainWindow)
    {
        let Some( selected ) = FileDialog::new().set_title( "Select a BSP folder" ).pick_folder()
        else
        {
            return
        };

        if selected.try_exists().is_err() || !utils::dir_contains_type( &selected, EXT_BSP )
        {
            popup( "Invalid folder", "The selected folder does not contain any BSP files.", 
                MessageLevel::Warning, MessageButtons::Ok );
                
            return;
        }

        self.bsp_dir = selected.clone();
        app.set_bsp_items( ModelRc::from( collect_bsp_items( &selected ).as_slice() ) );
        app.set_cvar_text( SharedString::new() );
    }

    fn write_cfg(&self, app: &MainWindow, wt: WriteType)
    {
        Cfg
        {
            cvars: app.get_cvar_text().to_string(),
            writetype: wt,
            is_skillcfg: app.get_skill_cfg(),
            bspdir: self.bsp_dir.clone(),
            bspwhitelist: current_bsp_whitelist( app ),
        }.create();
    }
}
