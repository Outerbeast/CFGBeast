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
use std::path::{Path, PathBuf};
/// Returns the current directory path, or "." if it fails.
#[macro_export] macro_rules! current_dir_path
{
    () =>
    {
        cfg_select!
        {
            windows => std::env::current_dir().unwrap_or( std::path::PathBuf::from( "." ) ),
            target_os = "linux" =>
            {
                env::current_exe()
                    .ok()
                    .and_then( |p| p.parent().map( |p| p.to_path_buf() ) )
                    .filter( |p| p.is_dir() )
                .unwrap_or_else( || std::env::current_dir().unwrap_or( std::path::PathBuf::from( "." ) ) )
            }
        }
    };
}
/// Searches all drives for a specific filename, returns the path to that file
pub fn search_drives(file_name: &str) -> Option<PathBuf>
{
    if file_name.trim().is_empty()
    {
        return None;
    }

    let locations: Vec<_> =
    cfg_select!
    {
        windows =>
        {
            ( 'A'..='Z' )
                .map( |d| format!( "{d}:/" ) )
                .filter( |drive|
                {
                    let root = Path::new( drive );
                    root.try_exists().is_ok() && root.is_dir()
                })
            .collect()
        }

        target_os = "linux" =>
        {
            vec!
            [
                dirs::home_dir()
                    .map( |p| p.join( ".steam" )
                    .join( "steam" )
                    .to_string_lossy().to_string() )
                .unwrap_or_default(),
                dirs::home_dir()
                    .map( |p| p.join( ".steam" )
                    .join( "root" )
                    .join( "steamapps" )
                    .join( "common" )
                    .to_string_lossy().to_string() )
                .unwrap_or_default(),
                dirs::home_dir()
                    .map( |p| p.join( "Steam" )
                    .join( "steamapps" )
                    .join( "common" )
                    .to_string_lossy().to_string())
                .unwrap_or_default(),
                dirs::home_dir()
                    .map( |p| p.join( ".local" )
                    .join( "share" ).join( "Steam" )
                    .join( "steamapps" )
                    .join( "common" )
                    .to_string_lossy().to_string() )
                .unwrap_or_default(),
                dirs::home_dir()
                    .map( |p| p.join( ".var" )
                    .join( "app" )
                    .join( "com.valvesoftware.Steam" )
                    .join( ".steam" )
                    .join( "steam" )
                    .join( "steamapps" )
                    .join( "common" )
                    .to_string_lossy().to_string() )
                .unwrap_or_default(),
                "/mnt".to_string(),
                "/opt".to_string(),
                "/usr/games".to_string(),
                "/usr/local/games".to_string(),
            ]
            .into_iter()
            .filter( |root|
            {
                let p = Path::new( root );
                p.try_exists().is_ok() && p.is_dir()
            })
            .collect()
        }
    };

    for location in locations
    {
        for entry in jwalk::WalkDir::new( &location ).max_depth( 12 )
        {
            let Ok( entry ) = entry 
            else 
            { 
                continue
            };

            if entry.file_name != file_name
            {
                continue;
            }

            let path = entry.path();

            if path.has_extension( &["lnk"] )
            {
                continue;
            }

            #[cfg( target_os = "windows" )]
            {
                let path_lower = path.to_string_lossy().to_lowercase();
                if path_lower.contains( "\\$recycle.bin\\" ) || path_lower.contains( "/$recycle.bin/" )
                {
                    continue;
                }
            }

            return Some( path );
        }
    }

    None
}
pub trait HasExtension
{
    fn has_extension(&self, extensions: &[&str]) -> bool;
}

impl<T: AsRef<Path>> HasExtension for T
{   /// Checks if a string has any of the specified extensions
    fn has_extension(&self, extensions: &[&str]) -> bool
    {
        let Some( ext ) = self.as_ref().extension().and_then( |e| e.to_str() )
        else
        {
            return false;
        };

        extensions.iter().any( |e| ext.eq_ignore_ascii_case( e ) )
    }
}
/// Checks if a directory contains at least one file of the specified type
pub fn dir_contains_type(dir: &Path, ext: &str) -> bool
{
    if !dir.is_dir()
    {
        return false;
    }

    match std::fs::read_dir( dir )
    {
        Ok( entries ) =>
        {
            for entry in entries.flatten()
            {
                if entry.path().has_extension( &[ext] )
                {
                    return true;
                }
            }

            false
        }

        Err( _ ) => false
    }
}
/// Line-by-line reader
pub fn read_trimmed_lines(path: &Path) -> Option<Vec<String>>
{
    let file = std::fs::File::open( path ).ok()?;

    let lines = std::io::BufRead::lines( std::io::BufReader::new( file ) )
        .map_while( Result::ok )
        .map( |l| l.trim().to_owned() )
        .filter( |l| !l.is_empty() )
    .collect();

    Some( lines )
}
/// Line-by-line writer for a given filename and extension.
pub fn write_lines<L: std::fmt::Display>(filename: &str, ext: &str, lines: &[L]) -> std::io::Result<()>
{
    use std::io::Write;

    if lines.is_empty()
    {
        return Err( std::io::Error::new( std::io::ErrorKind::InvalidData, "Lines collection is empty." ) );
    }

    let mut file = std::fs::OpenOptions::new()
        .write( true )
        .create( true )
        .truncate( true )
    .open( format!( "{filename}.{ext}" ) )?;

    for l in lines
    {
        writeln!( file, "{l}" )?;// Write can fail here, throw to prevent corruption.
    }
    
    Ok( () )
}
