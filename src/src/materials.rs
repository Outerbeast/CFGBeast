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
#![allow( unreachable_patterns )]
use std::
{
    fmt::Display, 
    fs::File,
    io::
    {
        self,
        Read,
        Seek,
        SeekFrom
    },
    path::Path
};

use strum::
{
    EnumString,
    EnumIter, 
    Display,
    IntoEnumIterator
};

use crate::utils::
{
    read_trimmed_lines,
    write_lines
};

#[repr( u8 )]
#[derive( Copy, Clone, Display, EnumString, EnumIter, PartialEq )]
pub enum MaterialKind
{
    #[strum( serialize = "C", serialize = "Concrete", to_string = "Concrete" )]
    Concrete = b'C',

    #[strum( serialize = "M", serialize = "Metal", to_string = "Metal" )]
    Metal = b'M',

    #[strum( serialize = "V", serialize = "Ventilation", to_string = "Ventilation" )]
    Ventilation = b'V',

    #[strum( serialize = "D", serialize = "Dirt", to_string = "Dirt" )]
    Dirt = b'D',

    #[strum( serialize = "S", serialize = "Slosh Liquid", to_string = "Slosh Liquid" )]
    SloshLiquid = b'S',

    #[strum( serialize = "T", serialize = "Tile", to_string = "Tile" )]
    Tile = b'T',

    #[strum( serialize = "G", serialize = "Grate", to_string = "Grate" )]
    Grate = b'G',

    #[strum( serialize = "W", serialize = "Wood", to_string = "Wood" )]
    Wood = b'W',

    #[strum( serialize = "P", serialize = "Computer", to_string = "Computer" )]
    Computer = b'P',

    #[strum( serialize = "Y", serialize = "Glass", to_string = "Glass" )]
    Glass = b'Y',

    #[strum( serialize = "F", serialize = "Flesh", to_string = "Flesh" )]
    Flesh = b'F'
}

impl MaterialKind
{
    pub fn index_to_kind(idx: i32) -> Self
    {
        Self::iter()
            .filter( |k| !matches!( k, Self::Concrete ) )
            .collect::<Vec<_>>()
            .get( idx as usize )
            .copied()
        .unwrap_or( Self::Metal )
    }
}

#[derive( PartialEq )]
pub struct MaterialEntry
{
    pub kind: MaterialKind,
    pub texture: Option<String>
}

impl MaterialEntry
{
    pub fn new(kind: MaterialKind, texture: Option<String>) -> Self
    {
        Self { kind, texture }
    }
    /// Converts a character to a MaterialKind and creates a new MaterialEntry.
    fn from_char(c: char, texture: &str) -> Self
    {
        let kind = c.to_string().parse::<_>().unwrap_or( MaterialKind::Concrete );
        let texture =
        if texture.is_empty()
        {
            None
        }
        else
        {
            Some( texture.to_owned() )
        };

        Self { kind, texture }
    }
    /// Reads a material file and returns a vector of MaterialEntries.
    pub fn from_file(path: &Path) -> Option<Vec<Self>>
    {
        let mut materials = vec![];

        for line in read_trimmed_lines( path )?
        {
            if let Some( ( c, tex ) ) = line.split_once( ' ' )
            && let Some( c ) = c.chars().next()
            && let new_entry = MaterialEntry::from_char( c, tex )
            && !materials.contains( &new_entry )
            {
                materials.push( new_entry );
            }
        }

        if materials.is_empty() { None } else { Some( materials ) }
    }
    /// Writes the material entries to a file in txt format
    pub fn to_file(filename: &str, entries: &[Self]) -> io::Result<()>
    {
        write_lines( filename, "txt", entries )
    }
    /// Strips the texture identifier from the start of a texture name.
    pub fn without_tex_iden(texture_name: &str) -> String
    {
        if texture_name.to_lowercase().starts_with( "scroll" )
        {
            return texture_name.replace( "SCROLL", "" );
        }

        match texture_name.chars().next() 
        {   // liquid, masked, light and opaque
            Some( '!' | '{' | '~' | '@' ) => texture_name.chars().skip( 1 ).collect(),
            // textures that use some index for toggle/random tiling
            Some( '+' | '-' )  => texture_name.chars().skip( 2 ).collect(),//... since its followed by an additional alphanumeric char
            _ => texture_name.to_string()
        }
    }
}

impl Display for MaterialEntry
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match &self.texture
        {
            Some( tex ) => write!( f, "{} {}", self.kind as u8 as char, Self::without_tex_iden( tex ) ),
            None => write!( f, "{}", self.kind as u8 as char )
        }
    }
}
/// Reads WAD for entire list of texture names.
pub fn read_texture_names(wad_path: impl AsRef<Path>) -> io::Result<Vec<String>>
{
    let mut file = File::open( wad_path )?;
    let mut header = [0u8; 12];
    file.read_exact( &mut header )?;

    if &header[0..4] != b"WAD3"
    {
        return Err( io::Error::new( io::ErrorKind::InvalidInput, "Not a valid WAD3 file." ) );
    }

    let num_items = i32::from_le_bytes( header[4..8].try_into().map_err( |e| io::Error::new( io::ErrorKind::InvalidData, e ) )? ) as usize;
    let dir_offset = u32::from_le_bytes( header[8..12].try_into().map_err( |e| io::Error::new( io::ErrorKind::InvalidData, e ) )? ) as u64;

    file.seek( SeekFrom::Start( dir_offset ) )?;

    let mut names = Vec::with_capacity( num_items );
    let mut entry = [0u8; 32];
    
    for _ in 0..num_items
    {
        file.read_exact( &mut entry )?;
        let name = entry[16..32]
            .iter()
            .take_while( |&&b| b != 0 )
            .copied()
        .collect::<Vec<_>>();

        if let Ok( tex ) = String::from_utf8( name )
        {
            names.push( tex );
        }
    }

    Ok( names )
}
