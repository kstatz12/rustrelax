use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use crate::args::*;

pub fn run(opts: Opts) {
   match opts.path_type {
       PathType::Default => {
           let paths = get_paths();
           let p = get_sound_path(paths, opts);
           play(p, opts);
       }
       PathType::Testing => {
           let paths = get_testing_paths();
           let p = get_sound_path(paths, opts);
           play(p, opts);
       }
   }
}

fn get_sound_path(map: HashMap<Type, &'static str>, opts: Opts) -> &'static str {
    return &map.get(&opts.sound_type).expect("Sound Type Not Found");
}

fn play(path: &str, opts: Opts) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
}

fn get_paths() -> HashMap<Type, &'static str> {
    let mut map: HashMap<Type, &'static str> = HashMap::new();

    map.insert(Type::Default, "/var/lib/rustrelax/sounds/thunder.ogg");
    map.insert(Type::Thunder, "/var/lib/rustrelax/sounds/thunder.ogg");
    map.insert(Type::Rain, "/var/lib/rustrelax/sounds/rain.ogg");
    map.insert(Type::CoffeeShop, "var/lib/rustrelax/sounds/coffee.ogg");

    return map;
}

fn get_testing_paths() -> HashMap<Type, &'static str> {
    let mut map: HashMap<Type, &'static str> = HashMap::new();

    map.insert(Type::Default, "src/data/storm.ogg");
    map.insert(Type::Thunder, "src/data/thunder.ogg");
    map.insert(Type::Rain, "src/data/rain.ogg");
    map.insert(Type::CoffeeShop, "src/data/coffee.ogg");

    return map;
}
