use crate::Global;
use std::fmt;

pub trait GameState: fmt::Debug {
    fn start(&mut self) {}
    #[allow(unused_variables)]
    fn update(&mut self, global: &mut Global) -> Option<Box<dyn GameState>> {
        None
    }
    fn draw(&self) {}
    fn exit(&mut self) {}
}
