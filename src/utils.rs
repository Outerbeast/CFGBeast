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
extern crate native_windows_gui as nwg;
use std::
{
    fs,
    path::
    {
        Path,
        PathBuf
    }
};
// Searches all drives for a specific filename, returns the path to that file
pub fn search_drives(file_name: &str) -> PathBuf
{
    if file_name.trim().is_empty()
    {
        return PathBuf::new();
    }

    let mut results: Vec<PathBuf> = Vec::new();

    for c in 'A'..='Z'
    {
        let drive = format!( "{}:/", c );
        let root = Path::new( &drive );

        if root.exists() && root.is_dir()
        {
            let walker = 
            walkdir::WalkDir::new( root )
                .max_depth( 10 )
                .into_iter()
                .filter_entry( |e|
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
// Opens a folder selection dialogue, returns the selected folder path
pub fn select_folder_dialogue(parent: &nwg::Window) -> Option<PathBuf>
{
    let mut dlg = nwg::FileDialog::default();

    nwg::FileDialog::builder()
        .title( "Select a folder" )
        .action( nwg::FileDialogAction::OpenDirectory )
        .build( &mut dlg )
    .unwrap_or_default();

    if dlg.run( Some( &parent.handle ) )
    {
        dlg.get_selected_item().ok().map( PathBuf::from )
    }
    else
    {
        None
    }
}
// Checks if a directory contains at least one file of the specified type
pub fn dir_contains_type(dir: &Path, ext: &str) -> bool
{
    if !dir.is_dir()
    {
        return false;
    }

    match fs::read_dir(dir)
    {
        Ok( entries ) =>
        {
            for entry in entries.flatten()
            {
                let path = entry.path();
                if path.extension()
                    .map( |e| e.eq_ignore_ascii_case( ext ) )
                .unwrap_or(false)
                {
                    return true;
                }
            }

            false
        }

        Err( _ ) => false,
    }
}
