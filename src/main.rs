/*
 *  BoardTracker
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

#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate bt_lib;

use conrod::{
    color,
    Button,
    Canvas,
    Circle,
    Color,
    Colorable,
    DropDownList,
    EnvelopeEditor,
    Frameable,
    Labelable,
    NumberDialer,
    Point,
    Positionable,
    Slider,
    Sizeable,
    Text,
    TextBox,
    Theme,
    Toggle,
    Widget,
    WidgetMatrix,
    XYPad,
};
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, 
                    WindowSettings};
use bt_lib::entity::player::*;

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Board Tracker", [1280, 720])
            .exit_on_esc(true).vsync(true).samples(4).build().unwrap();

    window.set_ups(60);
    let player1 = Player::new(0);
    while let Some(event) = window.next() {
        
    }
    println!("Hello world!");
}
