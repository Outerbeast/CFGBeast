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
pub const PRODUCT_NAME: &str = env!( "CARGO_PKG_NAME" );
const AUTHOR: &str = env!( "CARGO_PKG_AUTHORS" );
const VERSION: &str = env!( "CARGO_PKG_VERSION" );
const DESCRIPTION: &str = env!( "CARGO_PKG_DESCRIPTION" );

#[cfg(windows)]
fn main() -> std::io::Result<()>
{
    winresource::WindowsResource::new()
        .set_icon( "icon.ico" )
        .set( "ProductName", PRODUCT_NAME )
        .set( "ProductVersion", VERSION )
        .set( "FileDescription", DESCRIPTION )
        .set( "FileVersion", VERSION )
        .set( "LegalCopyright", AUTHOR )
        .set( "OriginalFilename", format!( "{}.exe", PRODUCT_NAME ).as_str() )
        .set( "InternalName", PRODUCT_NAME )
        .set( "CompanyName", AUTHOR )
        .set( "LegalTrademarks", AUTHOR )
        .set( "Comments", DESCRIPTION )
    .compile()?;

    Ok(())
}

#[cfg(not(windows))]
fn main() -> std::io::Result<()>
{
    compile_error!( "This application only supports Windows targets" );
}
