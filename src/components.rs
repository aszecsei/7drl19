use super::point::Point;
use specs::prelude::*;

#[derive(Clone, Debug)]
pub struct Pos(pub Point);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub ch: char,
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}

impl Tile {
    pub fn new(ch: char) -> Self {
        Tile { ch }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Playable;

impl Component for Playable {
    type Storage = NullStorage<Self>;
}
