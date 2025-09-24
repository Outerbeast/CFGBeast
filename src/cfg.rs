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
#[allow(non_snake_case)]

use std::fs;
use std::env;
use std::fs::DirEntry;
use std::path::*;
use std::io::*;

use crate::init::*;
use crate::gui::message_box;

#[derive(PartialEq)]
pub enum WriteType
{
    OVERWRITE,
    APPEND,
    REMOVE,
    DELETE
}

pub struct Cfg
{
    pub cvars: String,
    pub writetype: WriteType,
    pub skillcfg: bool,
    pub bspdir: PathBuf,
    pub bspwhitelist: Vec<String>,
}

const OTHER_CVARS: &[&str] =
&[
    "map_script",
    "globalmodellist",
    "globalsoundlist",
    "sentence_file",
    "materials_file",
    "forcepmodels",
    "as_command",
    "nomaptrans",
    // Equipment
    "nomedkit",
    "nosuit",
    "item_longjump",
    // Ammo
    "ammo_9mm",
    "ammo_buckshot",
    "ammo_gaussclip",
    "ammo_crossbow",
    "ammo_556",
    "ammo_rpg",
    // Weapons
    "weapon_357",
    "weapon_eagle",
    "weapon_uzi",
    "weapon_uziakimbo",
    "weapon_mp5",
    "weapon_shotgun",
    "weapon_m16",
    "weapon_crossbow",
    "weapon_sniperrifle",
    "weapon_m249",
    "weapon_rpg",
    "weapon_minigun",
    "weapon_gauss",
    "weapon_egon",
    "weapon_displacer",
    "weapon_tripmine",
    "weapon_handgrenade",
    "weapon_satchel",
    "weapon_hivehand",
    "weapon_snark",
    "weapon_grapple",
    "weapon_sporelauncher",
    // Additional mp_ cvars that don't exist in the default map cfg (why?)
    "mp_allowmodelselection",
    "mp_telefrag 0",
    "mp_monsterpoints 1",
    "mp_teamlist 0",
    "mp_teamoverride 1",
    "mp_teamplay 0",
    "mp_timeleft",
    "mp_timeleft_empty",
    "mp_survival_retries",
    "mp_survival_voteallow",
    "mp_classic_mode 0"
];

pub fn parse_cfg(file_cvars: fs::File) -> Vec<String>
{
    let mut cvars : Vec<String> = BufReader::new( file_cvars )
        .lines()
        .filter_map( Result::ok )
        .map( |line| line.trim().to_string() )
        .filter( |line| !line.is_empty() && !line.starts_with( "//" ) && !line.starts_with( "#" ) )
    .collect();

    cvars.sort();
    cvars
}

pub fn get_default_cvars() -> Vec<String>
{
    let file_default_cvars = 
    match fs::File::open( get_config( "default_map_settings_path.txt" ).trim() )
    {
        Ok( f ) => f,
        Err( _ ) => return vec!["default_setting 1".to_string()],
    };
    // Append hardcoded CVars
    let mut default_cvars = parse_cfg( file_default_cvars );
    let other_cvars: Vec<String> = OTHER_CVARS.iter().map( |&s| s.to_owned() ).collect();
    default_cvars.extend( other_cvars );
    default_cvars.sort();

    default_cvars
}

pub fn get_skill_cvars() -> Vec<String>
{
    let file_skill_cvars = 
    match fs::File::open( get_config( "skill_path.txt" ).trim() )
    {
        Ok( f ) => f,
        Err( _ ) => return vec!["skill 1".to_string()],
    };

    parse_cfg( file_skill_cvars )
}

pub fn load_bsps(chosen_dir: PathBuf) -> Vec<PathBuf>
{
    let current_dir =
    match chosen_dir.exists()
    {
        true => env::current_dir().expect( "Failed to get dir." ),
        false => chosen_dir
    };

    let entries: Vec<DirEntry> = fs::read_dir( Path::new( &current_dir ) )
        .unwrap()
        .filter_map( Result::ok )
        .filter( |e| e.path().extension().map_or( false, |ext| ext.eq_ignore_ascii_case( "bsp" ) ) )
    .collect();

    return entries.iter().map( |e| e.path() ).collect();
}

pub fn create_cfg(cfg: Cfg)
{
    let writetype = cfg.writetype;
    let whitelist = cfg.bspwhitelist;

    if cfg.cvars == ""
    {
        if writetype != WriteType::DELETE
        {
            message_box( "No CVars specified", "You did not add in any CVars.\nEnter your CVars in the text box and try again.", native_windows_gui::MessageButtons::Ok, native_windows_gui::MessageIcons::Warning );
            return;
        }    
    }

    let cvars_in = format!( "// CFGBeast generated config file\n{}\n", cfg.cvars );
    let mut count = 0u8;

    let bsps = load_bsps( cfg.bspdir );

    if bsps.is_empty()
    {
        message_box( "No BSP files found", "No BSP files found.\n\nPlease place the app executable in a map folder with valid BSPs and try again.", native_windows_gui::MessageButtons::Ok, native_windows_gui::MessageIcons::Warning );
        return;
    }
    // If whitelist is not empty, filter BSPs
    let bsps: Vec<PathBuf> =
    match !whitelist.is_empty()
    {
        true =>
        {
            bsps.into_iter()
                .filter( |path| 
                {
                    if let Some( stem ) = path.file_stem().and_then( |s| s.to_str() ) 
                    {
                        whitelist.iter().any( |w| 
                        {
                            // Strip extension from whitelist entry if present
                            let w_stem = Path::new( w )
                                .file_stem()
                                .and_then( |s| s.to_str() )
                                .unwrap_or( w );
                            w_stem.eq_ignore_ascii_case( stem )
                        })
                    }
                    else
                    {
                        false
                    }
                })
            .collect()
        }
        
        false => bsps
    };

    if bsps.is_empty()// But why is it empty?
    {
        message_box( "No matching BSP files found", "No matching BSP files found from the whitelist.\n\nPlease adjust the whitelist or place the app executable in a map folder with valid BSPs and try again.", native_windows_gui::MessageButtons::Ok, native_windows_gui::MessageIcons::Warning );
        return;
    }

    for file_path in bsps
    {
        let mut cfg_name = file_path.clone();

        match cfg.skillcfg
        {
            true =>
            {
                if let Some( stem ) = cfg_name.file_stem()
                {
                    let mut stem = stem.to_string_lossy().to_string();
                    stem.push_str( "_skl.cfg" );
                    cfg_name.set_file_name( stem );
                }
            }
            false => 
            {
                cfg_name.set_extension( "cfg" );
            }
        }

        match writetype
        {
            WriteType::OVERWRITE =>
            {
                let mut file = fs::File::create( &cfg_name ).expect( "Failed to create file." );
                count += file.write_all( cvars_in.as_bytes() ).is_ok() as u8;
            }

            WriteType::APPEND =>
            {
                let mut file = fs::OpenOptions::new()
                    .append( true )
                    .create( true )
                    .open( &cfg_name )
                    .expect( "Failed to open file." );

                count += file.write_all( cvars_in.as_bytes() ).is_ok() as u8;
            }

            WriteType::REMOVE =>
            {
                if let Ok( mut lines ) = fs::read_to_string( &cfg_name )
                {
                    for line in cvars_in.lines()
                    {
                        lines = lines.replace( line, "" );
                    }

                    count += fs::write( &cfg_name, lines ).is_ok() as u8;
                }
            }

            WriteType::DELETE =>
            {
                if cfg_name.exists()
                {
                    count += fs::remove_file( &cfg_name ).is_ok() as u8;
                }
            }
        }
    }

    match count
    {
        0 => { message_box( "No CFG files written", "No CFG files written.\n\nPlease place the app executable in a map folder with valid BSPs and try again.", native_windows_gui::MessageButtons::Ok, native_windows_gui::MessageIcons::Warning ); },
        _ => { message_box( "Done", &format!( "Processed {} .cfg file(s).", count ), native_windows_gui::MessageButtons::Ok, native_windows_gui::MessageIcons::Info ); }
    }
}
