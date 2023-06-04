pub mod custom_errors;
pub mod logger;
pub mod graphics;

use std::collections::HashMap;

pub use winit::{
    event::*,
    event_loop::ControlFlow,
};

#[derive(PartialEq, Clone, Debug)]
pub enum InputState{
    Pressed(bool),
    Released(bool),
}

pub struct InputWrapper(pub HashMap<VirtualKeyCode, InputState>);

impl InputWrapper {
    pub fn new() -> Self {
        InputWrapper(HashMap::new())
    }

    pub fn retain(&mut self) {
        self.0.retain(|_, state| {
            match state {
                InputState::Released(_) => false,
                InputState::Pressed(true) => {
                    *state = InputState::Pressed(false);
                    true
                }
                _ => true,
            }
        });
    }

    pub fn get_key(&self, key : VirtualKeyCode) -> InputState
    {
        match self.0.get(&key){
            Some(v) => (*v).clone(),
            None => InputState::Released(false),
        }
    }
}