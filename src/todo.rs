use serde::{Serialize, Deserialize};

use crate::utils::{colour::*, Options};
use crate::web::Web;

#[derive(Default, Serialize, Deserialize)]
pub struct ToDo {
    pub name: String,
    pub notes: Vec<String>,
    pub done: bool,
    pub children: Vec<usize>, // items required for this
}

impl ToDo {
    pub fn display(&self, web: &Web, po: &Options) {
        if self.done {
            print!("{}", GREEN)
        }
        else {
            print!("{}", RED)
        }
        println!("{}) {}{}", web.get_index_of_todo(self), self.name, COLOUR_RESET);
        let index = web.get_index_of_todo(self);

        for n in &self.notes {
            println!("\t- {}", n)
        }

        println!("\n\tprerequesites:");
        for p in &self.children {
            print!("\t\t");
            web.list[*p].display_short(web, po);
        }
        println!("\n\tprerequesite of:");
        for p in web.get_indexes_of_parent_tasks(index) {
            print!("\t\t");
            web.list[p].display_short(web, po);
        }
        println!("")
    }

    pub fn display_short(&self, web: &Web, po: &Options) {
        if (self.done && po.view_done) || (!self.done && po.view_undone) {
            if self.done {
                print!("{}", GREEN)
            }
            else {
                print!("{}", RED)
            }
            println!("{}) {}{}", web.get_index_of_todo(self), self.name, COLOUR_RESET);
        }
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