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
    env,
    fs,
    io,
    path::PathBuf,
    sync::
    {
        mpsc,
        OnceLock
    },
    thread
};

use rfd::
{
    FileDialog,
    MessageButtons,
    MessageDialogResult,
    MessageLevel
};

use crate::
{
    APPNAME,
    app,
    current_dir_path,
    cvar,
    utils::search_drives
};

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive( Debug, serde::Serialize, serde::Deserialize, Default )]
pub struct Config
{
    pub svencoopdir: Option<PathBuf>,
    #[serde( default )]
    pub additional_cvars: Vec<String>
}

impl Config
{
    pub fn get() -> &'static Self
    {
        CONFIG.get().expect( "Config not initialized" )
    }

    fn read_store() -> Result<Self, io::Error>
    {
        let p = config_path();
        
        if !p.try_exists()?
        { 
            return Ok( Config::default() );
        }

        serde_json::from_str( &fs::read_to_string( &p )? )
            .map_err( |e| io::Error::new( io::ErrorKind::InvalidData, format!( "{e}: {}", p.display() ) ) )
    }

    fn write_store(&self) -> io::Result<()>
    {
        fs::create_dir_all( appdata_base() )?;

        let p = config_path();
        let tmp = p.with_extension( "json.tmp" );

        fs::write( &tmp, serde_json::to_string_pretty( &self ).map_err( io::Error::other )?.as_bytes() )?;
        fs::rename( &tmp, &p )?;
        
        Ok( () )
    }

    fn normalize(&mut self)
    {   // Old configs stored the full file path; normalize to directory
        if let Some( dir ) = &self.svencoopdir
        && ( dir.is_file() || dir.file_name().and_then( |n| n.to_str() ) == Some( cvar::DEFAULT_MAP_SETTINGS ) )
        {
            self.svencoopdir = dir.parent().map( |p| p.to_path_buf() );
        }
    }

}

fn appdata_base() -> PathBuf 
{
    #[cfg( target_os = "windows" )]
    {
        if let Ok( local ) = env::var( "LOCALAPPDATA" )
        {
            return PathBuf::from( local ).join( APPNAME );
        }

        if let Ok( appdata ) = env::var( "APPDATA" )
        {
            return PathBuf::from( appdata ).join( APPNAME );
        }
    }

    #[cfg( target_os = "linux" )]
    {
        if let Some( config ) = env::var( "XDG_CONFIG_HOME" )
            .ok()
            .map( PathBuf::from )
        .filter( |p| p.try_exists().is_ok() )
        {
            return config.join( APPNAME );
        }

        if let Some( home ) = dirs::home_dir()
        {
            return home.join( ".config" ).join( APPNAME );
        }
    }

    current_dir_path!().join( APPNAME )
}

#[inline] fn config_path() -> PathBuf
{
    appdata_base().join( format!( "{APPNAME}.json" ) )
}

pub fn init() -> io::Result<PathBuf>
{
    if let Ok( mut store ) = Config::read_store()
    {
        store.normalize();

        if let Some( dir ) = store.svencoopdir.clone()
        {
            let _ = CONFIG.set( store );
            return Ok( dir );
        }
    }

    let exe_path = current_dir_path!();
    if exe_path.join( cvar::DEFAULT_MAP_SETTINGS ).try_exists().unwrap_or( false )
    {
        return save_and_return( exe_path );
    }

    let ( tx, rx ) = mpsc::channel();

    thread::spawn( move ||
    {
        let _ = tx.send( search_drives( cvar::DEFAULT_MAP_SETTINGS ) );
    });
    
    let dialog_result = app::popup(
        "Initial Setup",
        "CFGBeast is starting for the first time and needs to search for default map settings.\n
        Click 'Ok' to close this window and wait for the search to complete.\n
        Otherwise click 'Cancel' to skip search and manually set the path to your default map settings file.",
        MessageLevel::Info,
        MessageButtons::OkCancel );

    match dialog_result
    {
        MessageDialogResult::Ok =>
        {
            let found = rx.recv().map_err( io::Error::other )?;

            match found.and_then( |p| p.parent().map( |p| p.to_path_buf() ) )
            {
                Some( dir ) if dir.try_exists().unwrap_or( false ) => save_and_return( dir ),
                _ => Err( io::Error::new
                (
                    io::ErrorKind::NotFound,
                    "Sven Co-op install not found.\n
                    Try installing CFGBeast directly to 'Sven Co-op\\svencoop' and try again."
                ))
            }
        }

        MessageDialogResult::Cancel =>
        {
            let Some( folder ) = FileDialog::new()
                .set_title( "Select your Sven Co-op installation folder" )
            .pick_folder()
            else
            {
                return Err( io::Error::new
                (
                    io::ErrorKind::NotFound,
                    "Setup cancelled. No folder was selected."
                ));
            };

            if folder.join( cvar::DEFAULT_MAP_SETTINGS ).try_exists().unwrap_or( false )
            {
                save_and_return( folder )
            }
            else
            {
                Err( io::Error::new
                (
                    io::ErrorKind::InvalidData,
                    "Selected folder does not contain default_map_settings.cfg.\n\
                    Please select your 'Sven Co-op\\svencoop' directory."
                ))
            }
        }

        _ => Err( io::Error::new( io::ErrorKind::NotFound, "Setup cancelled." ) )
    }
}

fn save_and_return( dir: PathBuf ) -> io::Result<PathBuf>
{
    let store = Config { svencoopdir: Some( dir.clone() ), ..Default::default() };
    Config::write_store( &store )?;
    let _ = CONFIG.set( store );
    Ok( dir )
}
/// Clear the app cookie
pub fn reset() -> io::Result<()>
{
    let p = config_path();
    
    if p.try_exists().is_ok()
    {
        fs::remove_file( &p )?;
        println!( "Config reset: removed {p:?}" );
    }
    else
    {
        println!( "Config already clear: {p:?} not found" );
    }

    Ok( () )
}
