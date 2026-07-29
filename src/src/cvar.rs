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
    collections::HashSet,
    fs,
    io::
    {
        self,
        BufRead,
        BufReader,
        Write
    },
    path::
    {
        Path,
        PathBuf
    },
    sync::OnceLock
};

use rfd::
{
    MessageLevel,
    MessageButtons
};

use crate::
{
    app::popup,
    config::Config,
    current_dir_path,
    utils::HasExtension
};

const EXT_CFG: &str = "cfg";
pub const EXT_BSP: &str = "bsp";
pub const DEFAULT_MAP_SETTINGS: &str = "default_map_settings.cfg";
const SKILL_SETTINGS: &str = "skill.cfg";

static DEFAULT_CVARS: OnceLock<Vec<String>> = OnceLock::new();
static SKILL_CVARS: OnceLock<Vec<String>> = OnceLock::new();

static OTHER_CVARS: [&str; 50] =
[
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
    "mp_timeleft",
    "mp_timeleft_empty",
    "mp_survival_mode",
    "mp_survival_retries",
    "mp_survival_voteallow",
    "mp_classic_mode 0"
];

pub enum WriteType
{
    OVERWRITE,
    APPEND,
    REMOVE,
    DELETE
}

impl WriteType
{
    /// Executes the write operation on the given CFG file.
    ///
    /// | Variant    | Behavior |
    /// |------------|----------|
    /// | OVERWRITE  | Creates/overwrites `path` with `content` |
    /// | APPEND     | Appends `content` to `path` |
    /// | REMOVE     | Removes lines matching `content` from `path` |
    /// | DELETE     | Deletes `path` (content ignored) |
    pub(crate) fn execute(&self, path: &Path, content: &str) -> io::Result<()>
    {
        match self
        {
            WriteType::OVERWRITE => fs::File::create( path )?.write_all( content.as_bytes() ),
            WriteType::APPEND => 
            {
                fs::OpenOptions::new()
                    .append( true )
                    .create( true )
                    .open( path )?
                .write_all( content.as_bytes() )
            }

            WriteType::REMOVE =>
            {
                let buf = fs::read_to_string( path )?;
                let remove_lines: HashSet<_> = content.lines().collect();
                let result: String = buf
                    .lines()
                    .filter( |line| !remove_lines.contains( *line ) )
                    .collect::<Vec<_>>()
                .join( "\n" );

                fs::write( path, format!( "{result}\n" ) )
            }

            WriteType::DELETE =>
            {
                if path.try_exists()?
                {
                    fs::remove_file( path )?;
                }

                Ok( () )
            }
        }
    }
}

pub struct Cfg
{
    pub cvars: String,
    pub writetype: WriteType,
    pub is_skillcfg: bool,
    pub bspdir: PathBuf,
    pub bspwhitelist: Vec<String>
}

impl Cfg
{   /// Creates/Modifies/Deletes cfg files based on the Cfg struct data, returns number of files processed, -1 on error
    pub fn create(&self) -> i8
    {
        let writetype = &self.writetype;
        let whitelist = &self.bspwhitelist;

        if self.cvars.is_empty() && !matches!( writetype, WriteType::DELETE )
        {
            popup( "No CVars specified", 
                "You did not add in any CVars.\nEnter your CVars in the text box and try again.", 
                MessageLevel::Warning, MessageButtons::Ok );

            return -1;
        }

        let bsps = load_bsps( self.bspdir.as_path() );

        if bsps.is_empty()
        {
            popup( "No BSP files found", 
                "No BSP files found.\n\nPlease place the app executable in a map folder with valid BSPs and try again.", 
                MessageLevel::Warning, MessageButtons::Ok );
            
            return -1;
        }
        // If whitelist is not empty, filter BSPs
        let whitelist_stems: HashSet<_> = whitelist
            .iter()
            .filter_map( |w| Path::new( w ).file_stem()?.to_str() )
            .map( |s| s.to_ascii_lowercase() )
        .collect();

        let bsps =
        if whitelist_stems.is_empty()
        {
            bsps
        }
        else
        {
            bsps
                .into_iter()
                .filter( |path| path.file_stem()
                .and_then( |s| s.to_str() )
                .is_some_and( |s| whitelist_stems.contains( &s.to_ascii_lowercase() ) ) )
            .collect()
        };

        if bsps.is_empty()// But why is it empty?
        {
            popup( "No matching BSP files found", 
            "No matching BSP files found from the whitelist.\n\n
                Please adjust the whitelist or place the app executable in a map folder with valid BSPs and try again.", 
            MessageLevel::Warning, MessageButtons::Ok );

            return -1;
        }

        let mut count = 0;

        for file_path in bsps
        {
            let mut cfg_name = file_path.clone();

            if self.is_skillcfg
            {
                if let Some( stem ) = cfg_name.file_stem()
                {
                    let mut stem = stem.to_string_lossy().to_string();
                    stem.push_str( "_skl.cfg" );
                    cfg_name.set_file_name( stem );
                }
            }
            else
            {
                cfg_name.set_extension( EXT_CFG );
            }

            count += writetype.execute( &cfg_name, &format!( "{}\n", self.cvars ) ).is_ok() as u8;
        }

        match count
        {
            0 => 
            {
                popup( "No CFG files written", 
                    "No CFG files written.\n\n
                    Please place the app executable in a map folder with valid BSPs and try again.", 
                    MessageLevel::Warning, MessageButtons::Ok );
            }

            _ =>
            {
                popup( "Done", 
                    &format!( "Processed {count} CFG file(s)." ), 
                    MessageLevel::Info, MessageButtons::Ok );
            }
        }

        count as i8
    }
}
/// Reads CVars from a CFG file
pub(crate) fn parse_cfg(file_cvars: fs::File) -> Vec<String>
{
    let mut cvars: Vec<_> = BufReader::new( file_cvars )
        .lines()
        .map_while( Result::ok )// fingers crossed
        .map( |line| line.trim().to_string() )
        .filter( |line| !line.is_empty() && !line.starts_with( "//" ) && !line.starts_with( '#' ) )
    .collect();

    cvars.sort();
    cvars
}

pub fn get_default_cvars() -> &'static Vec<String>
{   
    DEFAULT_CVARS.get_or_init( ||
    {
        let cvar_path = Config::get().svencoopdir
            .clone()
            .unwrap_or_default()
        .join( DEFAULT_MAP_SETTINGS );

        let mut cvars =
        match fs::File::open( &cvar_path )
        {
            Ok( file ) => parse_cfg( file ),
            Err( e ) =>
            {
                eprintln!( "Failed to load default cvars from {}: {e}", cvar_path.display() );
                vec![]
            }
        };

        let other_cvars: Vec<_> = OTHER_CVARS.iter().map( |&s| s.to_owned() ).collect();
        cvars.extend( other_cvars );
        cvars.sort();

        cvars
    })
}

pub fn get_skill_cvars() -> &'static Vec<String>
{
    SKILL_CVARS.get_or_init( ||
    {
        let cvar_path = Config::get().svencoopdir
            .clone()
            .unwrap_or_default()
        .join( SKILL_SETTINGS );

        match fs::File::open( &cvar_path )
        {
            Ok( file ) => parse_cfg( file ),
            Err( e ) =>
            {
                eprintln!( "Failed to load skill cvars from {}: {e}", cvar_path.display() );
                vec![]
            }
        }
    })
}
/// Collects all BSP files in a given directory and returns their paths.
pub fn load_bsps(chosen_path: &Path) -> Vec<PathBuf>
{   // Use the chosen_dir if it exists, otherwise fall back to current_dir
    let chosen_path =
    if chosen_path.try_exists().is_ok()
    {
        chosen_path
    }
    else
    {
        &current_dir_path!()
    };
    // Read the directory, return empty vec on error
    if let Ok( rd ) = fs::read_dir( chosen_path )
    {
        rd
            .filter_map( Result::ok )
            .map( |e| e.path() )
            .filter( |p| p.has_extension( &[EXT_BSP] ) )
        .collect()
    }
    else
    {
        vec![]
    }

}
