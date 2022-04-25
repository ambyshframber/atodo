use argparse::{ArgumentParser, StoreConst, Collect, StoreOption, StoreTrue, StoreFalse};
use std::process::exit;

use utils::{Options, Command};
use web::{Web};

mod todo;
mod web;
mod utils;

fn main() {
    exit(run())
}

fn run() -> i32 {
    let po = get_options();

    let path = match &po.todo_file_path {
        Some(p) => p,
        None => ".todo"
    };
    let mut web = match Web::load_from_file(&path) {
        Ok(w) => w,
        Err(e) => {
            println!("failed to load web file: {}", e);
            return 1
        }
    };

    type C = Command;
    let exit_code = match po.command { // HERE WE GO
        // non-mutating
        C::List => {
            web.list(&po);
            0
        }
        C::View => {
            match po.main_index {
                Some(i) => web.view(i, &po),
                None => {
                    println!("task index required!");
                    2
                }
            }
        }
        C::Random => {
            web.random(&po)
        }
        C::RandomTopLevel => {
            web.random_top(&po)
        }
        C::RandomBottomLevel => {
            web.random_bottom(&po)
        }

        // mutating
        _ => {
            match web.backup(path) { // backup todo file
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    return 1
                }
            }
            if let C::Add = po.command {
                web.add(&po)
            }
            else {
                if let None = po.main_index {
                    println!("task index required!");
                    2
                }
                else {
                    match po.command {
                        C::MarkDone => {
                            web.mark(&po, true) // mark just changes the done field
                        }
                        C::MarkNotDone => {
                            web.mark(&po, false)
                        }
                        C::AddNote => {
                            web.add_note(&po)
                        }
                        C::Remove => {
                            web.remove(&po)
                        }
                        C::Edit => {
                            web.edit(&po)
                        }
                        _ => unreachable!() // make the compiler shut up
                    }
                }
            }
        }
    };

    match exit_code {
        0 => { // only save if program doesn't error
            let _ = web.save_to_file(path); // MAKE THIS NOT SHIT
            0
        }
        _ => {
            exit_code
        }
    }
}

fn get_options() -> Options {
    let mut po = Options::default();
    po.view_undone = true;
    po.colours = true;
    
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut po.command) // main command
            .add_option(&["-l"], StoreConst(Command::List), "list all tasks")
            .add_option(&["-a"], StoreConst(Command::Add), "add a task")
            .add_option(&["-A"], StoreConst(Command::AddNote), "add a note to a task")
            .add_option(&["-v"], StoreConst(Command::View), "view a task in detail")
            .add_option(&["-e"], StoreConst(Command::Edit), "edit a task")
            //.add_option(&["-p"], StoreConst(Command::AddParents), "add parent tasks to a task")
            //.add_option(&["-c"], StoreConst(Command::AddChildren), "add child tasks to a task")
            //.add_option(&["-u"], StoreConst(Command::Unlink), "unlink tasks from a task")
            .add_option(&["-r"], StoreConst(Command::Remove), "remove a task")
            .add_option(&["-d"], StoreConst(Command::MarkDone), "mark a task as done")
            .add_option(&["-n"], StoreConst(Command::MarkNotDone), "mark a task as not done")
            .add_option(&["-R"], StoreConst(Command::Random), "select and display a random task")
            .add_option(&["-T"], StoreConst(Command::RandomTopLevel), "select and display a random top-level task")
            .add_option(&["-B"], StoreConst(Command::RandomBottomLevel), "select and display a random bottom-level task")
            ;
            
            ap.refer(&mut po.main_index).add_option(&["-t"], StoreOption, "the task index to work with");
            
            ap.refer(&mut po.parent_tasks).add_option(&["-p"], Collect, "a task to add as a parent (works with -a or -e)");
            ap.refer(&mut po.child_tasks).add_option(&["-c"], Collect, "a task to add as a child (works with -a or -e)");
            ap.refer(&mut po.unlink_tasks).add_option(&["-u"], Collect, "a task to unlink when using -e");
            
            ap.refer(&mut po.view_done).add_option(&["-D"], StoreTrue, "view tasks that are already completed");
            ap.refer(&mut po.view_undone).add_option(&["-N"], StoreFalse, "do not view tasks that are not already completed");
            
            ap.refer(&mut po.todo_file_path).add_option(&["-P"], StoreOption, "select a custom file path (by default uses ./.todo)");
            
            ap.refer(&mut po.colours).add_option(&["-C"], StoreFalse, "disable colours");
            
            ap.refer(&mut po.string).add_argument("string", Collect, "the string the program will use for -a and -A");
            
            ap.parse_args_or_exit()
        }
        
        #[cfg(windows)]
        {
            po.colours = false
        }
        po
    }
    