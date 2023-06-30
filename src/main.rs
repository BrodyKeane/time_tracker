use dbus::{
    Message,
    arg::ReadAll, ffidisp::Connection,
};

use std::{time::Duration, env};

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

    let mut args = env::args().skip(1);
    let mut primary_arg = args.next().unwrap_or("help".into());

    if primary_arg.contains("help") {
        primary_arg = "help".into();
    }
    
    let result = match primary_arg.as_str() {
        "start" => start(), 
        "end" => todo!(),
        "pause" => todo!(),
        "resume" => todo!(),
        "display" => todo!(),
        "config" => todo!(),
        "add" => add_app(),
        "help" => todo!(),
        _ => todo!()
    }; 

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }
}

fn start() -> Result<(), Box<dyn std::error::Error>> {
    let db = tables::get_database()?;

    let conn = Connection::new_session()?;

    let match_rule = format!(
        "type='signal',interface='org.freedesktop.DBus',member='NameOwnerChanged',arg0='{}'",
        "org.mpris.MediaPlayer2.spotify" // Replace with the application name you want to detect
    );

    conn.add_match(&match_rule)?;
    
    loop {
        let mut msgs = conn.incoming(60000);
        if let Some(msgs) = msgs.next() {
            read_msgs(msgs);
        }
    }
}

fn read_msgs(mut msg: Message) {
    todo!()
}


fn add_app() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
//    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
//
//    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;
//    for name in names { println!("{}", name); }
    todo!();
}
