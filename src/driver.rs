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
    io,
    fs
};

use native_windows_gui::
{
    MessageButtons,
    MessageIcons
};

use crate::
{
    APPNAME,
    config,
    cvar,
    gui,
    motd
};

pub fn run() -> Result<(), io::Error>
{
    let _ =
    match config::init()
    {
        Ok( dir ) =>
        {
            dir
        }

        Err( e ) =>
        {
            gui::window::message_box( "Sven Co-op install Not Found",
                format!( "Could not find a valid Sven Co-op installation.
                    \nReason:\n{}
                    \n\nTry installing {} directly to 'Sven Co-op\\svencoop' and try again.", e, APPNAME ).as_str(), 
                MessageButtons::Ok,
                MessageIcons::Error );

            return Err( e );
        }
    };

    let args: Vec<String> = env::args().collect();

    match args.len()
    {
        n if n > 1 =>
        {
            for file in &args[1..]
            {
                if file.ends_with( ".cfg" )
                {
                    if let Ok( content ) = fs::read_to_string( file )
                    {
                        cvar::Cfg 
                        { 
                            cvars: content, 
                            writetype: cvar::WriteType::OVERWRITE, 
                            is_skillcfg: false, 
                            bspdir: env::current_dir().unwrap_or_default(), 
                            bspwhitelist: vec![] 
                        }.create();
                    }
                }
                else if file.ends_with( "_motd.txt" ) && let Ok( content ) = fs::read_to_string( file )
                {
                    motd::create_motd( content );
                }
            }
        }
        // Nothing was dragged, launch application
        _ => gui::events::GUI( env::current_dir().unwrap_or_default().as_path() ),
    }

    Ok( () )

}
