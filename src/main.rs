extern crate getopts;
use getopts::Options;
use my_help_handler::{open_my_help_file_with, read_content};
use std::env;
use std::io;

mod my_help_handler;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn setup_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("e", "edit", "open .my_help file with vim editor");
    opts.optflag("a", "all", "display all my_help content");
    return opts;
}

fn get_home_dir() -> String {
    if let Ok(home_dir) = env::var("HOME") {
        return home_dir;
    } else {
        eprintln!("Could not get home directory.");
        return String::from("/home");
    }
}

fn confirm_read_all() -> bool {
    println!("Do you want to see all my_help content? [y/n]");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_lowercase() == "y";
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts: Options = setup_opts();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f)
        }
    };

    let my_help_file_path = format!("{}/.my_help", get_home_dir());
    let app = "vim";

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("e") {
        open_my_help_file_with(&my_help_file_path, app);
        return;
    }

    let mut keywords = matches.free.clone();
    if !keywords.is_empty() {
        // remove command name itself
        keywords.remove(0);
    }

    if !matches.opt_present("a") && keywords.len() == 0 {
        if !confirm_read_all() {
            return;
        }
    }

    read_content(&my_help_file_path, keywords);
}
