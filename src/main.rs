/* Copyright (c) 2016 Mike	Lubinets
 * Originally written by Mike Lubinets
 *
 * See LICENSE (MIT) */

extern crate time;
extern crate rustc_serialize;
extern crate ansi_term;
extern crate docopt;

#[macro_use]
extern crate log;
extern crate env_logger;

mod todolist;

use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;
use docopt::Docopt;
use rustc_serialize::json;



const USAGE: &'static str = "
Todo List.

Usage:
    todo <list>
    todo <list> new <task> <description>
    todo <list> done <num>
    todo <list> del  <num>
    todo <list> clear
    todo <list> [options]

Comands:
    new <task> <description>    Add new task
    done <num>                  Complete task

Options:
    -h --help                   Display this message
    -n --flat                   Disable coloured output
    -a --all                    Display task and description
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_flat: bool,
    flag_all: bool,
    cmd_new: bool,
    cmd_done: bool,
    cmd_del: bool,
    cmd_clear: bool,
    arg_list: Option<String>,
    arg_task: Option<String>,
    arg_description: Option<String>,
    arg_num: Option<u16>,
}

fn main() {
    env_logger::init().unwrap();
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());


    let listname = args.arg_list.expect("No list passed");

    // Open requested TodoList
    let mut list = open_list_read(&listname);

    // Read file to buffer
    let mut buffer = String::with_capacity(list.metadata().unwrap().len() as usize);
    list.read_to_string(&mut buffer).unwrap();

    // Decode Json
    let mut tasklist: todolist::TaskList = json::decode(&buffer).unwrap_or_else(|e| {
        warn!("Failed to load list {}: {}", listname, e);
        todolist::TaskList::new()
    });

    // Main logic
    if args.cmd_new {
        let task        = args.arg_task.expect("No task passed");
        let description = args.arg_description.expect("No description passed");
        tasklist.new_task(
            &task,
            &description,
        )
    } else
    if args.cmd_done {
        let number = args.arg_num.expect("No number passed");
        tasklist.complete(number);
    } else
    if args.cmd_del {
        let number = args.arg_num.expect("No number passed");
        tasklist.delete(number);
    } else
    if args.cmd_clear {
        tasklist.clear();
    } else {
        if args.flag_flat {
            if args.flag_all {
                println!("{}", tasklist.to_string());
            } else {
                println!("{}", tasklist.to_short_string());
            }
        } else {
            if args.flag_all {
                println!("{}", tasklist.to_colored());
            } else {
                println!("{}", tasklist.to_short_colored());
            }
        }
    }

    // If tasklist was changed, write it down
    if args.cmd_new || args.cmd_done || args.cmd_del || args.cmd_clear {
        let mut list = open_list_write(&listname);
        let encoded = json::encode(&tasklist).unwrap();
        list.write_all(encoded.as_bytes()).unwrap();
        list.flush().unwrap();
    }

}

fn open_list_read(filename: &str) -> std::fs::File {
    let path = prepare_filesystem(filename);
    File::open(path).unwrap()
}

fn open_list_write(filename: &str) -> std::fs::File {
    let path = prepare_filesystem(filename);
    File::create(path).unwrap()
}

fn prepare_filesystem(filename: &str) -> PathBuf {
    use std::fs;
    let mut path = config_folder();

    for _ in 0..1 {
        fs::create_dir_all(path.as_path()).unwrap();

        if !path.is_dir() {
            panic!(
                format!("{} is not a directory.", path.display())
            );
        }
        path.push("list");
    }

    path.push(filename);
    if !path.exists() {
        OpenOptions::new().create(true).write(true).open(&path).unwrap();
    }

    path
}

fn config_folder() -> PathBuf {
    use std::env::var_os;

    let mut pb = {
        // On Unix, try to find XDG_CONFIG_HOME or fall back to HOME/.config
        if cfg!(target_family = "unix") {
            match var_os("XDG_CONFIG_HOME") {
                Some(path) => PathBuf::from(path),
                None => match var_os("HOME") {
                    Some(path) => {
                        let mut pb = PathBuf::from(path);
                        pb.push(".config");
                        pb
                    }
                    None => panic!("No $HOME directory found in env!")
                }
            }
        } else
        // On Windows, try find APPDATA or panic
        if cfg!(target_family = "windows") {
            // Get AppData from os env
            match var_os("APPDATA") {
                Some(path) => PathBuf::from(path),
                None       => panic!("No %APPDATA% directory found in env!")
            }
        } else {
        // Who the fuck needs console todolist without OS?
            panic!("Unsupported environment")
        }
    };

    // Append application folder
    pb.push("todolist");
    pb
}
