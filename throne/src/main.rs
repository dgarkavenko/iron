fn main() {

    let title = String::from("Throne");
    pollster::block_on(iron::graphics::window::run(title, 640, 480));
}
