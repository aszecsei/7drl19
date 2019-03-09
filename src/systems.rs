use super::components;
use super::keyboard;
use specs::prelude::*;
use std::sync::{Arc, Mutex};

pub struct RenderSystem {
    pub viewport: Arc<Mutex<web_sys::Element>>,
    pub half_width: i32,
    pub half_height: i32,
}

unsafe impl Send for RenderSystem {}

#[derive(SystemData)]
pub struct Renderable<'a> {
    pos: ReadStorage<'a, components::Pos>,
    tile: ReadStorage<'a, components::Tile>,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = Renderable<'a>;

    fn run(&mut self, data: Renderable) {
        let mut res =
            String::with_capacity(4 * self.half_width as usize * self.half_height as usize);
        for y in -self.half_height..self.half_height {
            for x in -self.half_width..self.half_width as i32 {
                let mut has_pushed = false;
                for (pos, tile) in (&data.pos, &data.tile).join() {
                    if pos.0.x == x && pos.0.y == y {
                        res.push(tile.ch);
                        has_pushed = true;
                    }
                }
                if !has_pushed {
                    res.push(' ');
                }
            }
            res.push('\n');
        }
        let vp = self.viewport.lock().unwrap();
        vp.set_inner_html(&res);
    }
}

pub struct PlayerControllerSystem;

impl<'a> System<'a> for PlayerControllerSystem {
    type SystemData = (
        Read<'a, super::Keyboards>,
        WriteStorage<'a, components::Pos>,
        ReadStorage<'a, components::Playable>,
    );

    fn run(&mut self, (kbs, mut pos, playable): Self::SystemData) {
        for (pos, _) in (&mut pos, &playable).join() {
            // update the player position
            if kbs.is_key_pressed(keyboard::Key::W) {
                pos.0.y -= 1;
            }
            if kbs.is_key_pressed(keyboard::Key::S) {
                pos.0.y += 1;
            }
            if kbs.is_key_pressed(keyboard::Key::A) {
                pos.0.x -= 1;
            }
            if kbs.is_key_pressed(keyboard::Key::D) {
                pos.0.x += 1;
            }
        }
    }
}
