use serde::{Serialize, Deserialize};

use crate::utils::{colour::*, Options, will_display};
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
        self.display_short(web, po);
        let index = web.get_index_of_todo(self);

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
            else if self.all_children_done(web) || self.children.len() == 0 {
                print!("{}", BLUE)
            }
            else {
                print!("{}", RED)
            }
        }
        println!("{}) {}{}", web.get_index_of_todo(self), self.name, COLOUR_RESET);
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
