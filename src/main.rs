#![feature(seek_convenience)]
mod calculate;
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;


fn main() {
    // let board_array: [&str; 3] = ["As", "Ks", "Jd"];
    // let playerscards_array: [&str; 4] = ["8s", "7s", "?", "?"];
    // println!("{:?}", calculate::calc_hand(&board_array,&playerscards_array));

    notify_new_file();
    println!("toto")
}

fn read_log(file_name: &str, files_history: &mut HashMap<String, u64>) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::io::SeekFrom;

    println!("{:?}",files_history);

    let mut f = File::open(file_name)?;
    let next_start = f.stream_len()?;
    let mut reader = BufReader::new(f);

    println!("The file is currently {} bytes long", next_start);

    let start = files_history.entry(String::from(file_name)).or_insert(0);

    // 182
    reader.seek(SeekFrom::Start(*start))?;
    let lines = reader.lines();
    for line in lines {
        println!("{:?}", line.unwrap());
    }

    files_history.insert(String::from(file_name), next_start);

    Ok(())
}

fn notify_new_file() {
    extern crate notify;

    use notify::DebouncedEvent;
    use notify::{watcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;


    let mut files_history : HashMap<String, u64> = HashMap::new();
    
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("/home/emottet/.wine/drive_c/users/emottet/My Documents/Winamax Poker/accounts/emimo/history/", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) =>
            // read_log("/home/emottet/.wine/drive_c/users/emottet/My Documents/Winamax Poker/accounts/emimo/history/20200418_Last Chance BIG BANG(343766210)_real_holdem_no-limit.txt");
            {
                println!("{:?}", event);
                match event {
                    DebouncedEvent::Write(path) => {
                        read_log(path.as_path().to_str().expect("Testing expect"), &mut files_history);
                    }
                    _ => {}
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
