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
pub use crate::
{
    config::Config,
    cvar::
    {
        Cfg,
        WriteType,
        EXT_BSP,
        DEFAULT_MAP_SETTINGS,
        get_default_cvars,
        get_skill_cvars,
        load_bsps
    },
    replacements::
    {
        Replacement,
        EXT_GMR,
        EXT_GSR,
        EXTS_MODELS,
        EXTS_SOUNDS
    },
    materials::
    {
        MaterialEntry,
        MaterialKind,
        read_texture_names
    },
    utils::HasExtension
};
