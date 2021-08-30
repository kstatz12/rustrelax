use ctrlc;
use std::{env, thread, time};
pub mod args;
pub mod engine;
fn main() {
    let args: Vec<String> = env::args().collect();

    let opts = args::handle_args(args);

    match opts {
        None => return,
        Some(o) => engine::run(o),
    }

    let mut running: bool = true;

    //there has to be a better way to do this
    ctrlc::set_handler(move || running = false);

    while running {
        //lets keep this from thrashing constantly
        let dur = time::Duration::from_millis(500);
        thread::sleep(dur)
    }
}
