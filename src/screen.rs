use std::time::Duration;

#[derive(Debug)]
pub enum Transition {
    None,
    Pop,
}

pub trait Screen: Debug {
    fn update(&mut self, context: &mut Context, dtime: Duration) -> bool;
    fn draw(&self, context: &mut Context) -> bool;
}

pub struct Screens {
    screens: Vec<Box<dyn Screen>>,
}
