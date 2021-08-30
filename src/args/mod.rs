use string_builder::Builder;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Type {
    Default,
    Thunder,
    Rain,
    CoffeeShop,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Default,
    Loop,
    Single,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PathType {
    Default,
    Testing,
}

#[derive(Copy, Clone)]
pub struct Opts {
    pub sound_type: Type,
    pub mode: Mode,
    pub path_type: PathType,
}

pub fn handle_args(args: Vec<String>) -> Option<Opts> {
    if args.len() == 1 {
        print_help();
        return None;
    } else {
        let parse_result = parse_args(args).expect("cannot parse args");
        return Some(parse_result);
    }
}

fn print_help() {
    let mut b: Builder = Builder::default();
    b.append("\nUsage: rustrelax [options]");
    b.append("\nPlay Relaxing Sounds, But With Rust");
    b.append("\nOptions: ");
    b.append("\n -h, --help display this help");
    b.append("\n -t, --thunder play thunder sounds");
    b.append("\n -r, --rain play rain sounds");
    b.append("\n -c, --coffee play coffee shop sounds");
    b.append("\n -l, --loop loop mode");
    b.append("\n -s, --single single play mode");
    b.append("\n --testing to load testing sounds");

    let s = b.string().unwrap();
    print!("{}", s);
}

fn parse_args<'a>(args: Vec<String>) -> Result<Opts, &'a str> {
    let mut opt = Opts {
        sound_type: Type::Default,
        mode: Mode::Default,
        path_type: PathType::Default,
    };

    for a in args {
        match &a as &str {
            "-h" | "--help" => print_help(),
            "-t" | "--thunder" => opt.sound_type = Type::Thunder,
            "-r" | "--rain" => opt.sound_type = Type::Rain,
            "-c" | "--coffee" => opt.sound_type = Type::CoffeeShop,
            "-l" | "--loop" => opt.mode = Mode::Loop,
            "-s" | "--single" => opt.mode = Mode::Single,
            "--testing" => opt.path_type = PathType::Testing,
            _ => return Err("Invalid Arg"),
        }
    }
    return Ok(opt);
}
