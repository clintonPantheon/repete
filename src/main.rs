use clap::{App, Arg};
use core::sync::atomic::{AtomicBool, Ordering};
use failure::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::{thread, time};

fn do_main(runnable: Arc<AtomicBool>) -> Result<(), Error> {
    let mut sleep_interval: u64 = 0;
    let mut file_contents: String = String::new();
    let matches = App::new("string_mem_test")
        .about("prints file every n seconds")
        .long_about(
            "examples:
./string_mem_test -t 181 -f ./file_name   # print file contents of file_name every 181 seconds
",
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .help("integer of seconds")
                .value_name("SECONDS")
                .number_of_values(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("file path")
                .value_name("FILE_PATH")
                .number_of_values(1)
                .takes_value(true),
        )
        .get_matches();
    // handle time
    if let Some(t) = matches.value_of("time") {
        let interval = t.parse::<u64>().unwrap();
        sleep_interval = interval * 1000u64; // interval in milliseconds
    }
    // handle file path
    if let Some(file_path) = matches.value_of("file") {
        let path = Path::new(file_path);
        let mut file_ptr = File::open(path)?;
        file_ptr.read_to_string(&mut file_contents)?;
    }
    let sleepy = time::Duration::from_millis(sleep_interval);
    while runnable.load(Ordering::SeqCst) {
        println!("{:?}", file_contents);
        thread::sleep(sleepy);
    }
    println!("Got it! Exiting...");
    Ok(())
}

fn main() {
    let runnable = Arc::new(AtomicBool::new(true));
    let r = runnable.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set handler for SIGINT / SIGTERM");

    match do_main(runnable) {
        Err(x) => {
            eprintln!("Error: {}", x);
            eprintln!("{}", x.backtrace());
            std::process::exit(1);
        }
        _ => {}
    }
}
