use std::time::Instant;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::graphics::state::State;

pub async fn run<F>(title : String, width : i32, height: i32, mut state_handler: F)
where
    F: FnMut(&mut State , Option<&WindowEvent>, &mut ControlFlow, f32) + 'static,
{

    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height));

    let window = window_builder.build(&event_loop).unwrap();
    let mut state = State::new(window).await;
    
    let mut now: Instant = Instant::now();


    event_loop.run(move |event: Event<()>, _, control_flow| {
        
        state_handler(
            &mut state,
            match event { 
                Event::WindowEvent { ref event, window_id } => Some(event),
                _ => None
            },
            control_flow,
            now.elapsed().as_secs_f32()
        );

        now = Instant::now();

        match event {
            
            Event::WindowEvent { ref event, window_id } if window_id == state.window().id() => {
                
                match event {
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }

            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }

            Event::RedrawEventsCleared => {
                state.window().request_redraw();
            }

            _ => {}
        }
    });
}
