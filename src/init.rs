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
use std::fs;
use std::env;
use std::path::*;

use crate::gui;

const DEFAULT_MAP_SETTINGS: &str = "default_map_settings.cfg";
const SKILL_SETTINGS: &str = "skill.cfg";

fn get_localappdata_path() -> PathBuf
{
    let base = env::var( "LOCALAPPDATA" ).expect( "LOCALAPPDATA not set" );
    PathBuf::from( base ).join( crate::gui::APPNAME )
}

pub fn get_config(filename: &str) -> String
{
    if filename.trim().is_empty()
    {
        return String::new();
    }
    
    let path = get_localappdata_path().to_path_buf().join( filename );

    match path.exists()
    {
        true => fs::read_to_string( &path ).unwrap_or_default(),
        false => String::new(),
    }
}

pub fn write_config(filename: &str, contents: &str)
{
    if filename.trim().is_empty() || contents.trim().is_empty()
    {
        return;
    }

    let file_path = get_localappdata_path().join( filename );
    // Ensure only the APPNAME directory exists
    if let Some( parent ) = file_path.parent()
    {
        fs::create_dir_all( parent ).expect("Failed to create config directory" );
    }

    fs::write( &file_path, contents ).expect("Failed to write file" );
}

fn search_drives(file_name: &str) -> PathBuf
{
    if file_name.trim().is_empty()
    {
        return PathBuf::new();
    }

    let mut results: Vec<PathBuf> = Vec::new();

    for drive in vec!["A:/", "B:/", "C:/", "D:/", "E:/",]
    {
        let root = Path::new( drive );

        if root.exists() && root.is_dir()
        {
            let walker = walkdir::WalkDir::new( root )
                .max_depth( 10 )
                .into_iter()
                .filter_entry(|e|
                {
                    let name = e.file_name().to_string_lossy();
                    !name.eq_ignore_ascii_case( "$Recycle.Bin" )
                })
                .filter_map( Result::ok )
                .filter( |e| e.file_name().to_string_lossy().eq_ignore_ascii_case( file_name ) );

            for entry in walker
            {
                results.push( entry.path().to_path_buf() );
            }
        }
    }

    match results.is_empty()
    {
        true => PathBuf::new(),
        false => results[0].clone(),
    }
}

pub fn init()
{
    if !get_config( "default_map_settings_path.txt" ).is_empty()
    {
        return;
    }

    let splash = gui::show_wait_splash();
    let default_path = search_drives( DEFAULT_MAP_SETTINGS );

    if default_path.exists()
    {
        write_config( "default_map_settings_path.txt", default_path.to_string_lossy().as_ref() );
        // skill.cfg is in the same dir
        if default_path.with_file_name( SKILL_SETTINGS ).exists()
        {
            write_config( "skill_path.txt", default_path.with_file_name( SKILL_SETTINGS ).to_string_lossy().as_ref() );
        }
    }

    splash.close();
}
