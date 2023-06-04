use std::{time::Instant, collections::HashMap};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{graphics::state::State, InputState, InputWrapper};

pub async fn run<U>(title : String, width : i32, height: i32, mut update_handler: U)
where
    U : FnMut(&mut State, &mut ControlFlow, &InputWrapper, f32) + 'static,
{

    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height));

    let window = window_builder.build(&event_loop).unwrap();
    let mut state = State::new(window).await;
    let mut last_frame_rendered: Instant = Instant::now();

    //registry

    let mut inputs = InputWrapper::new();

    event_loop.run(move |event: Event<()>, _, control_flow| {
        
        control_flow.set_poll();

        match event {
            
            Event::WindowEvent { ref event, .. } => {
                
                match event {
                    
                    WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.set_exit(),

                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                        ..
                    } => {
                        
                        let pressed = if *state == ElementState::Pressed {true} else {false};
                        
                        if pressed {
                            match inputs.0.contains_key(keycode) {
                                false => {
                                    inputs.0.insert(*keycode, InputState::Pressed(true));
                                },
                                _ => ()
                            }
                        }else{
                            match inputs.0.contains_key(keycode) {
                                true => {
                                    inputs.0.insert(*keycode, InputState::Released(true));
                                },
                                _ => ()
                            }
                        }
                    },

                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }

            Event::MainEventsCleared =>{
                
                let delta_time = last_frame_rendered.elapsed().as_secs_f32();
                last_frame_rendered = Instant::now();

                update_handler(
                    &mut state,
                    control_flow,
                    &inputs,
                    delta_time
                );

                inputs.retain();
                
                state.update(delta_time);
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }

            _ => {}
        }
    });
}
