use iron::*;

fn main() {

    //ecs -- transfrom mesh render
    //read entities from files

    let mut current_measurment = 0;
    let mut measurements : [f32; 10] = [1.0 / 60.0; 10];

    let title = String::from("Throne");

    pollster::block_on(iron::graphics::flow::run(title, 640, 480,

        move | render_state, control_flow, inputs, delta_time  |
        {
            for (key, state) in &inputs.0 {
                match (key, state)
                {
                    (VirtualKeyCode::E, InputState::Pressed(true)) => {render_state.camera.fovy += 10.0},
                    (VirtualKeyCode::A, InputState::Pressed(_)) =>  {render_state.camera.target.x -= delta_time * 10.0},
                    (VirtualKeyCode::D, InputState::Pressed(_)) =>  {render_state.camera.target.x += delta_time * 10.0},
                    (VirtualKeyCode::W, InputState::Pressed(_)) =>  {render_state.camera.eye.z -= delta_time * 10.0},
                    (VirtualKeyCode::S, InputState::Pressed(_)) =>  {render_state.camera.eye.z += delta_time * 10.0},
                    (VirtualKeyCode::Space, InputState::Pressed(_)) =>  {render_state.camera.eye.y += delta_time * 10.0},

                    _ => ()
                }
            }

            if inputs.get_key(VirtualKeyCode::Q) == InputState::Released(true){
                render_state.camera.fovy -= 10.0;
            }

            current_measurment = (current_measurment + 1) % measurements.len();
            measurements[current_measurment] = delta_time;
            let mut av : f32 = measurements.iter().sum();
            av = av / measurements.len() as f32;
            println!("fps: {:.1}", 1.0 / av );
        },
    ));
}   