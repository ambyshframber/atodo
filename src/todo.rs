use serde::{Serialize, Deserialize};

use crate::utils::colour::*;
use crate::web::Web;

#[derive(Default, Serialize, Deserialize)]
pub struct ToDo {
    pub name: String,
    pub notes: Vec<String>,
    pub done: bool,
    pub children: Vec<usize>, // items required for this
}

impl ToDo {
    pub fn display(&self, web: &Web) {
        self.display_short(web);
        let index = web.get_index_of_todo(self);

        for n in &self.notes {
            println!("\t- {}", n)
        }

        println!("\n\tprerequesites:");
        for p in &self.children {
            print!("\t\t");
            web.list[*p].display_short(web);
        }
        println!("\n\tprerequesite of:");
        for p in web.get_indexes_of_parent_tasks(index) {
            print!("\t\t");
            web.list[p].display_short(web);
        }
    }

    pub fn display_short(&self, web: &Web) {
        if self.done {
            print!("{}", GREEN)
        }
        else {
            print!("{}", RED)
        }
        println!("{}) {}{}", web.get_index_of_todo(self), self.name, COLOUR_RESET);
    }

    pub fn add_note(&mut self, s: String) {
        self.notes.push(s)
    }
}