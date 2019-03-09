#[macro_use]
extern crate shred_derive;

mod components;
mod keyboard;
mod point;
mod systems;

use cfg_if::cfg_if;
use specs::prelude::*;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

#[wasm_bindgen]
pub struct Game {
    last_time: f64,
    keyboards: Keyboards,
    world: World,
    game: Arc<Mutex<web_sys::Element>>,
}

#[derive(Debug, Default, Clone)]
pub struct Keyboards {
    keyboard_state: keyboard::KeyboardState,
    prev_keyboard_state: keyboard::KeyboardState,
}

impl Keyboards {
    pub fn is_key_pressed(&self, k: keyboard::Key) -> bool {
        self.keyboard_state.is_key_down(k) && self.prev_keyboard_state.is_key_up(k)
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        let window = web_sys::window().expect("unable to access window");
        let document = window.document().expect("unable to access document");
        let game = document
            .get_element_by_id("game")
            .expect("unable to access game");
        let gm = Arc::new(Mutex::new(game));
        let mut w = World::new();

        w.register::<components::Pos>();
        w.register::<components::Tile>();
        w.register::<components::Playable>();

        let k = Keyboards {
            keyboard_state: keyboard::KeyboardState::new(),
            prev_keyboard_state: keyboard::KeyboardState::new(),
        };
        w.add_resource(k.clone());

        w.create_entity()
            .with(components::Pos(point::Point::new(0, 0)))
            .with(components::Tile::new('@'))
            .with(components::Playable)
            .build();

        Game {
            last_time: 0.0,
            keyboards: k,
            world: w,
            game: gm,
        }
    }

    pub fn update(&mut self, time: f64) {
        if self.last_time == 0.0 {
            self.last_time = time;
        }
        let _delta = time - self.last_time;

        {
            let mut k = self.world.write_resource::<Keyboards>();
            *k = self.keyboards.clone();
        }
        let mut dispatcher = DispatcherBuilder::new()
            .with(
                systems::PlayerControllerSystem,
                "player_controller_system",
                &[],
            )
            .with_barrier()
            .with(
                systems::RenderSystem {
                    viewport: self.game.clone(),
                    half_height: 10,
                    half_width: 10,
                },
                "render_system",
                &[],
            )
            .build();
        dispatcher.setup(&mut self.world.res);
        dispatcher.dispatch(&mut self.world.res);

        self.keyboards
            .prev_keyboard_state
            .copy_from(&self.keyboards.keyboard_state);
    }

    pub fn key_down(&mut self, code: &str) {
        let x = keyboard::Key::from_str(code);
        if let Some(key) = x {
            self.keyboards.keyboard_state.set_key_down(key);
        }
    }

    pub fn key_up(&mut self, code: &str) {
        let x = keyboard::Key::from_str(code);
        if let Some(key) = x {
            self.keyboards.keyboard_state.set_key_up(key);
        }
    }
}

#[wasm_bindgen]
pub fn run() -> Result<Game, JsValue> {
    set_panic_hook();

    let game = Game::new();
    Ok(game)
}
