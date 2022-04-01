use serde::{Serialize, Deserialize};
use chrono::prelude::*;

use crate::utils::{colour::*, Options, will_display};
use crate::web::Web;

#[derive(Serialize, Deserialize)]
pub struct ToDoOld {
    pub name: String,
    pub notes: Vec<String>,
    pub done: bool,
    pub children: Vec<usize>, // items required for this
}
impl ToDoOld { // just used to allow backwards compat
    pub fn to_new(self) -> ToDo {
        let time_completed = if self.done {
            Some(Utc::now())
        }
        else {
            None
        };
        ToDo {
            name: self.name,
            notes: self.notes,
            done: self.done,
            children: self.children,
            time_added: Utc::now(),
            time_completed
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ToDo {
    pub name: String,
    pub notes: Vec<String>,
    pub done: bool,
    pub children: Vec<usize>, // items required for this
    pub time_added: DateTime<Utc>,
    pub time_completed: Option<DateTime<Utc>>,
}

impl ToDo {
    pub fn display(&self, web: &Web, po: &Options) {
        self.display_short(web, po);
        let index = web.get_index_of_todo(self);

        let added_local = self.time_added.with_timezone(&Local);
        println!("\tadded: {}", added_local.format("%Y-%m-%d %H:%M:%S %:z")); // iso 8601 w/ no decimal seconds
        match self.time_completed {
            Some(t) => {
                let completed_local = t.with_timezone(&Local);
                println!("\tcompleted: {}", completed_local.format("%Y-%m-%d %H:%M:%S %:z"))
            }
            None => {}
        }

        for n in &self.notes {
            println!("\t- {}", n)
        }

        println!("\n\tprerequesites:");
        for c in &self.children {
            let t = &web.list[*c];
            if will_display(t.done, po) {
                print!("\t\t");
                t.display_short(web, po);
            };
        }
        println!("\n\tprerequesite of:");
        for p in web.get_indexes_of_parent_tasks(index) { // todo items don't directly store their own parents
            let t = &web.list[p];
            if will_display(t.done, po) { 
                print!("\t\t");
                t.display_short(web, po);
            };
        }
        println!("")
    }

    pub fn display_short(&self, web: &Web, po: &Options) {
        if po.colours {
            if self.done {
                print!("{}", GREEN)
            }
            else if self.all_children_done(web) || self.children.len() == 0 { // display as blue if bottom level task
                print!("{}", BLUE)
            }
            else {
                print!("{}", RED)
            }
        }
        print!("{}) {}{}", web.get_index_of_todo(self), self.name, COLOUR_RESET);
        if !po.colours {
            if self.done {
                print!(" (done)")
                
            }
        }
        println!("")
    }

    pub fn add_note(&mut self, s: String) {
        self.notes.push(s)
    }

    pub fn all_children_done(&self, web: &Web) -> bool {
        let mut ret = true;
        for c in &self.children {
            if !web.list[*c].done {
                ret = false
            }
        }
        ret
    }
}
