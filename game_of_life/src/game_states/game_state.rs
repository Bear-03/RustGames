use std::fmt;

use super::SharedState;

pub trait GameState: fmt::Debug {
    fn start(&mut self) {}
    #[allow(unused_variables)]
    fn update(&mut self, shared_state: &mut SharedState) -> Option<Box<dyn GameState>> {
        None
    }
    fn draw(&self) {}
    fn exit(&mut self) {}
}
