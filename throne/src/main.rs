use std::time::Instant;

use winit::{
    event::*,
    event_loop::ControlFlow,
};

use iron::graphics::state;

enum Actions{
    ZoomIn,
    ZoomOut,
    Left,
    Right,
}

struct Inputs{
    key : VirtualKeyCode,
    state : bool,
    action : Actions
}

impl Inputs{
    fn new(key : VirtualKeyCode, state : bool, action : Actions) -> Self
    {
        Inputs {
            key,
            state,
            action,
        }
    }
}

fn main() {

    let mut inputs = [
        Inputs::new(VirtualKeyCode::A, false, Actions::Left),
        Inputs::new(VirtualKeyCode::D, false, Actions::Right),
        Inputs::new(VirtualKeyCode::W, false, Actions::ZoomIn),
        Inputs::new(VirtualKeyCode::S, false, Actions::ZoomOut),
    ];

    let title = String::from("Throne");
    pollster::block_on(iron::graphics::window::run(title, 640, 480,
        move | render_state, event, control_flow, delta_time  |
        {
            match event {
                Some(window_event) => {
                    match window_event {

                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,

                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                            ..
                        } => {
                            
                            for (_i, elem) in inputs.iter_mut().enumerate() {
                                if elem.key == *keycode
                                {
                                    elem.state = if *state == ElementState::Pressed {true} else {false}
                                }
                            }
                            
                        }

                        _ => (),
                    }
                },
                None => (),
            }

            for (_i, elem) in inputs.iter_mut().enumerate() {
                match elem.action
                {
                    Actions::ZoomIn if elem.state => {render_state.camera.fovy -= delta_time * 10.0},
                    Actions::ZoomOut if elem.state => {render_state.camera.fovy += delta_time * 10.0},
                    Actions::Left if elem.state => {render_state.camera.target.x -= delta_time * 10.0},
                    Actions::Right if elem.state => {render_state.camera.target.x += delta_time * 10.0},
                    _ => ()
                }
            }

        }
    ));

}   