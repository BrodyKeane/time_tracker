use std::env;

mod tables;

enum OS {
    Linux,
    Windows
}

fn main() {
    let os = match env::consts::OS {
        "linux" => OS::Linux,
        "windows" => OS::Windows,
        os => panic!("unsuported OS: {}", os),
    };

    let mut args = env::args();
    let mut primary_arg = args.next().unwrap_or("help".into());

    if primary_arg.contains("help") {
        primary_arg = "help".into();
    }
    
    match primary_arg.as_str() {
        "start" => todo!(),
        "end" => todo!(),
        "pause" => todo!(),
        "resume" => todo!(),
        "display" => todo!(),
        "help" => todo!(),
        _ => todo!()
    }
}

fn start() {}
