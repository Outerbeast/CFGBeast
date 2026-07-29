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
    path::Path
};

use rfd::
{
    MessageButtons,
    MessageLevel
};

use crate::
{
    app,
    config,
    current_dir_path,
    prelude::*
};
/// Helper for duplicating a MOTD file for each BSP file in a given directory.
fn create_motd(motd_content: String, bsp_path: &Path) -> io::Result<u8>
{
    if motd_content.trim().is_empty()
    {
        return Err( io::Error::new( io::ErrorKind::InvalidData, "MOTD content cannot be empty." ) );
    }

    let bsps = load_bsps( bsp_path );

    if bsps.is_empty()
    {
        app::popup( "No BSP files found", 
            "Please place the app executable in a map folder with valid BSPs and try again.", 
            MessageLevel::Warning, 
            MessageButtons::Ok );

        return Ok( 0 );
    }

    let count = bsps
        .iter()
        .filter_map( |bsp| bsp.file_stem().and_then( |s| s.to_str() ) )
        .map( |base| fs::write( format!( "{base}_motd.txt" ), &motd_content ).is_ok() as u8 )
    .sum::<_>();

    match count
    {
        0 =>
        { 
            app::popup( "No MOTD files written", 
                "Please place the app executable in a map folder with valid BSPs and try again.", 
                MessageLevel::Warning, MessageButtons::Ok );
        }

        _ =>
        {
            app::popup( "Done", &format!( "Processed {count} MOTD file(s)." ), 
                MessageLevel::Info, 
                MessageButtons::Ok );
        }
    }

    Ok( count )
}

pub fn run() -> io::Result<()>
{
    if env::args().any( |a| a == "--reset-config" || a == "-reset" || a == "-r" )
    {
        config::reset()?;
    }

    config::init()?;

    if let args = env::args().skip( 1 ).filter( |a| a != "--reset-config" && a != "-r" ).collect::<Vec<_>>() 
    && !args.is_empty()
    {
        for file in &args
        {
            if file.has_extension( &["cfg"] )
            {
                if let Ok( content ) = fs::read_to_string( file )
                && !content.trim().is_empty()
                {
                    Cfg
                    {
                        cvars: content,
                        writetype: WriteType::OVERWRITE,
                        is_skillcfg: false,
                        bspdir: current_dir_path!(),
                        bspwhitelist: vec![]
                    }.create();
                }
            }
            else if file.ends_with( "_motd.txt" )
            && let Ok( content ) = fs::read_to_string( file )
            {
                create_motd( content, &current_dir_path!() )?;
            }
            else if file.has_extension( &[EXT_GMR, EXT_GSR] )
            && let Some( replacements ) = Replacement::from_file( file.as_ref() )
            {
                let filename = Path::new( file )
                    .file_stem()
                    .and_then( |s| s.to_str() )
                .unwrap_or( "output" );

                let ( models, sounds ) = Replacement::partition_replacements( &replacements );

                if !models.is_empty()
                {
                    let _ = Replacement::to_file( filename, &models );
                }

                if !sounds.is_empty()
                {
                    let _ = Replacement::to_file( filename, &sounds );
                }
            }
        }
    }
    else
    {
        app::launch_gui().map_err( io::Error::other )?
    }

    Ok( () )
}
