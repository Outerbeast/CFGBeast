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
    fmt::
    {
        self,
        Display
    },
    io,
    path::Path
};

use crate::utils::
{
    HasExtension,
    read_trimmed_lines,
    write_lines
};

pub const EXT_GSR: &str = "gsr";
pub const EXT_GMR: &str = "gmr";

pub static EXTS_MODELS: [&str; 2] = ["mdl", "spr"];
pub static EXTS_SOUNDS: [&str; 16] =
[
    "aiff", "asf", "dls", "flac", "it", "m3u", "mid", "mod", "mp2", "mp3", "ogg", "s3m", "vag", "wav", "wma", "xm"
];

#[derive( Clone )]
pub enum Replacement
{
    Models { original: String, new: String },
    Sounds { original: String, new: String }
}

impl Replacement
{   /// Classifies the file based on its extension
    pub fn classify(item: &str) -> Option<Self>
    {
        if item.has_extension( &EXTS_MODELS )
        {
            Some( Self::Models { original: String::new(), new: String::new() } )
        }
        else if item.has_extension( &EXTS_SOUNDS )
        {
            Some( Self::Sounds { original: String::new(), new: String::new() } )
        }
        else
        {
            None
        }
    }
    /// Attempts to create a new Replacement instance from two paths.
    pub fn try_new(from: &str, to: &str) -> Option<Self>
    {
        let ( from, to ) = ( Self::truncate_path( from ), Self::truncate_path( to ) );

        match Self::classify( &from )?
        {
            Self::Models { .. } => Some( Self::Models { original: from, new: to } ),
            Self::Sounds { .. } => Some( Self::Sounds { original: from, new: to } )
        }
    }
    /// Gets the original item.
    pub fn get_original(&self) -> &str
    {
        match self
        {
            Self::Models { original, .. } | 
            Self::Sounds { original, .. } => original,
        }
    }
    /// Gets the new item.
    pub fn get_new(&self) -> &str
    {
        match self
        {
            Self::Models { new, .. } | 
            Self::Sounds { new, .. } => new,
        }
    }
    /// Gets replacement mappings from a file.
    pub fn from_file(path: &Path) -> Option<Vec<Self>>
    {
        let mut replacements = vec![];

        for mapping in read_trimmed_lines( path )?
        {
            let parts: Vec<_> = mapping.split( '"' ).collect();
            let mut quoted = parts
                .iter()
                .enumerate()
                .filter( |( i, _ )| i % 2 == 1 )
                .map( |( _, s )| s.trim() )
            .filter( |s| !s.is_empty() );

            let Some( from ) = quoted.next()
            else
            {
                continue
            };

            let Some( to ) = quoted.next()
            else
            {
                continue
            };

            if let Some( replacement ) = Self::try_new( from, to )
            && !replacement.is_redundant()
            {
                replacements.push( replacement );
            }
        }

        Some( replacements )
    }
    /// Chops off the absolute path of a model/sprite/sound file to only include the relative path from the models/sprites/sound directory.
    pub fn truncate_path(path: &str) -> String
    {
        if let Some( split ) = path.split_once( "models" )
        {
            format!( "models{}", split.1 )
        }
        else if let Some( split ) = path.split_once( "sprites" )
        {
            format!( "sprites{}", split.1 )
        }
        else if let Some( split ) = path.split_once( "sound" )
        {
            split.1.to_string()
        }
        else// File is not inside expected directories, return the filename and parent directory name.
        {
            let file_path = Path::new( path );

            let filename = file_path
                .file_name()
                .and_then( |s| s.to_str() )
            .unwrap_or( path );

            let parent = file_path
                .parent()
                .and_then( |p| p.file_name() )
                .and_then( |s| s.to_str() )
            .unwrap_or( "" );

            let ext = file_path.extension()
                .and_then( |s| s.to_str() )
            .unwrap_or( "" );

            let combined = format!( "{parent}/{filename}" );
            match ext
            {
                "mdl" => format!( "models/{combined}" ),
                "spr" => format!( "sprites/{combined}" ),
                _ if EXTS_SOUNDS.contains( &ext ) =>
                {
                    if parent.is_empty()
                    { 
                        filename.to_string()
                    }
                    else
                    {
                        combined
                    }
                }

                _ => filename.to_string()
            }
        }
    }
    /// Splits the collected replacements into models and sounds
    pub fn partition_replacements(replacements: &[Self]) -> (Vec<Self>, Vec<Self>)
    {
        replacements
            .iter()
            .cloned()
        .partition( |r| matches!( r, Replacement::Models { .. } ) )
    }
    /// Writes the replacements to a file in txt format
    pub fn to_file(filename: &str, replacements: &[Self]) -> io::Result<()>
    {
        let ext = 
        match replacements.first()
        {
            Some( Self::Models { .. } ) => EXT_GMR,
            Some( Self::Sounds { .. } ) => EXT_GSR,
            None => return Err( io::Error::new( io::ErrorKind::InvalidFilename, "Invalid extension." ) )
        };

        write_lines( filename, ext, replacements )
    }
    /// Checks whether both the original and new is identical.
    /// Returns true if they match.
    pub fn is_redundant(&self) -> bool
    {
        self.get_original() == self.get_new()
    }
}

impl Display for Replacement
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!( f, "\"{}\" \"{}\"", self.get_original(), self.get_new() )
    }
}
