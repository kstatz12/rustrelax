use std::env;
pub mod args;
pub mod engine;
fn main() {
    let args: Vec<String> = env::args().collect();

    let opts = args::handle_args(args);

    match opts {
        None => return,
        Some(o) => engine::run(o),
    }
}
