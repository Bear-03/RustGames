use std::fmt;

pub trait GameState: fmt::Debug {
    type Shared;

    fn start(&mut self) {}
    #[allow(unused_variables)]
    fn update(
        &mut self,
        shared_state: &mut Self::Shared,
    ) -> Option<Box<dyn GameState<Shared = Self::Shared>>> {
        None
    }
    fn draw(&self) {}
    fn exit(&mut self) {}
}
