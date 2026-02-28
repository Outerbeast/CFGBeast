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
    env,
    fs,
    io,
    path::PathBuf
};

use crate::
{
    gui::window::message_box,
    utils::search_drives
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Config
{
    pub svencoopdir: Option<String>
}

fn appdata_base() -> PathBuf 
{
    if let Ok( local ) = env::var( "LOCALAPPDATA" ) 
    {
        PathBuf::from( local ).join( crate::APPNAME )
    } 
    else if let Ok( appdata ) = env::var( "APPDATA" ) 
    {
        PathBuf::from( appdata ).join( crate::APPNAME )
    } 
    else 
    {
        env::current_dir().unwrap_or_default().join( crate::APPNAME )
    }
}

fn config_path() -> PathBuf
{
    appdata_base().join( format!( "{}.toml", crate::APPNAME ) )
}

pub fn read_store() -> Result<Config, io::Error>
{
    let p = config_path();
    
    if !p.exists()
    { 
        return Ok( Config::default() );
    }

    let s = fs::read_to_string( p )?;
    let conf: Config = toml::from_str( &s ).map_err( io::Error::other )?;
    
    Ok( conf )
}

fn write_store(st: &Config) -> Result<(), io::Error>
{
    fs::create_dir_all( appdata_base() )?;

    let p = config_path();
    let tmp = p.with_extension( "toml.tmp" );
    let s = toml::to_string_pretty( st ).map_err( io::Error::other )?;

    fs::write( &tmp, s.as_bytes() )?;
    fs::rename( &tmp, &p )?;
    
    Ok(())
}

pub fn init() -> io::Result<PathBuf>
{   // Load config first if its exists
    if let Ok( store ) = read_store() && let Some( dir ) = store.svencoopdir
    {
        return Ok( PathBuf::from( dir ) );
    }
    // Initial setup
    let splash = crate::gui::window::show_wait_splash();
    let exe_path = env::current_dir().unwrap_or_default();// If the default cfg file exists in the current dir, just use that.
    let default_cfg_dir =
    match exe_path.join( crate::cvar::DEFAULT_MAP_SETTINGS ).exists()
    {
        true => exe_path,
        false =>// Doesn't exist, look for it
        {
            search_drives( crate::cvar::DEFAULT_MAP_SETTINGS ).unwrap_or_default()
        }
    };

    if !default_cfg_dir.exists()
    {
        splash.close();
        return Err( io::Error::new( io::ErrorKind::NotFound, "No directory exists." ) );
    }//... from this point we just assume skill.cfg is also here along with default_map_settings.cfg
    // Save folder path into TOML
    if let Err( e ) = write_store( &Config { svencoopdir: Some( default_cfg_dir.to_string_lossy().into_owned() ) } )
    {
        message_box( "Error",
            format!( "Failed to save config.\nReason: {}", e ).as_str(),
            native_windows_gui::MessageButtons::Ok,
            native_windows_gui::MessageIcons::Error );
    };
    
    splash.close();
    Ok( default_cfg_dir )
}