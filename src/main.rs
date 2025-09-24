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
#![windows_subsystem = "windows"]

use std::env;
use std::fs;

mod init;
mod gui;
mod cfg;
mod motd;

fn main()
{
    init::init();
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
                        cfg::create_cfg( cfg::Cfg 
                        { 
                            cvars: content, 
                            writetype: cfg::WriteType::OVERWRITE, 
                            skillcfg: false, 
                            bspdir: env::current_dir().unwrap(), 
                            bspwhitelist: vec![] 
                        });
                    }
                }
                else if file.ends_with( "_motd.txt" )
                {
                    if let Ok( content ) = fs::read_to_string( file )
                    {
                        motd::create_motd( content );
                    }
                }
            }
        }
        // Nothing was dragged, launch application
        _ => gui::GUI(),
    }

}
