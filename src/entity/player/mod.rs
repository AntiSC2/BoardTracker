/*
 *  DM-lib
 *  Copyright (C) 2016 Jakob Sinclair
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Jakob Sinclair <sinclair.jakob@openmailbox.org>
 */

use entity::statistics::Statistics;

pub struct Player {
    pub stats: Statistics,
}

impl Player {
    pub fn new(x: i32) -> Player {
        Player {
            stats: Statistics {
                hp: x,
                max_hp: x,

                strenght: x,
                dexterity: x,
                constitution: x,
                intelligence: x,
                wisdom: x,
                charisma: x,
            }
        }
    }
}
