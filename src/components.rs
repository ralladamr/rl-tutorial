use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub(crate) struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Component)]
pub(crate) struct Renderable {
    pub(crate) glyph: rltk::FontCharType,
    pub(crate) fg: RGB,
    pub(crate) bg: RGB,
}

#[derive(Component, Debug)]
pub(crate) struct Player {}
