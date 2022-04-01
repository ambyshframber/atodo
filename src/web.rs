use serde::{Serialize, Deserialize};
use std::fs;
use rand::{prelude::IteratorRandom, thread_rng};
use chrono::prelude::*;

use crate::todo::{ToDo, ToDoOld};
use crate::utils::{Options, collate_string_vec, will_display};

#[derive(Serialize, Deserialize)]
pub struct Web {
    pub list: Vec<ToDo>
}
#[derive(Serialize, Deserialize)]
pub struct WebOld {
    pub list: Vec<ToDoOld>
}
impl WebOld {
    pub fn load_from_file(filename: &str) -> Result<WebOld, String> {
        match fs::read_to_string(filename) {
            Ok(s) => WebOld::load_from_string(&s),
            Err(_) => panic!()
        }
    }
    pub fn load_from_string(s: &str) -> Result<WebOld, String> {
        match serde_json::from_str::<WebOld>(&s) {
            Ok(g) => Ok(g),
            Err(e) => Err(format!("json parse error for file! ({})", e)) // exit without panicking
        }
    }
    pub fn to_new(self) -> Web {
        let mut ret = Web::new();

        for t in self.list {
            ret.list.push(t.to_new())
        }

        ret
    }
}

impl Web {
    pub fn new() -> Web {
        Web {
            list: Vec::new()
        }
    }
    pub fn load_from_file(filename: &str) -> Result<Web, String> {
        match fs::read_to_string(filename) {
            Ok(s) => Web::load_from_string(&s),
            Err(_) => Ok(Web::new())
        }
    }
    pub fn load_from_string(s: &str) -> Result<Web, String> {
        match serde_json::from_str::<Web>(&s) {
            Ok(g) => Ok(g),
            Err(e) => Err(format!("json parse error for file! ({})", e)) // exit without panicking
        }
    }
    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        match fs::write(filename, serde_json::to_string(self).unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("failed to write file {}", filename))
        }
    }
    pub fn backup(&self, path: &str) -> Result<(), String> {
        let mut path_owned = String::from(path);
        path_owned.push_str("BAK");
        self.save_to_file(&path_owned)
    }

    pub fn get_index_of_todo(&self, target: &ToDo) -> usize { // panics if it can't find it
        for (i, t) in self.list.iter().enumerate() {
            if t.name == target.name {
                return i
            }
        }

        unreachable!()
    }
    pub fn get_indexes_of_parent_tasks(&self, index: usize) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();
        for (i, t) in self.list.iter().enumerate() {
            if t.children.contains(&index) {
                ret.push(i)
            }
        }
        ret
    }

    // non-mutating
    pub fn list(&self, po: &Options) {
        for t in &self.list {
            if will_display(t.done, po) {
                t.display_short(self, po)
            }
        }
    }
    pub fn view(&self, index: usize, po: &Options) -> i32 {
        if index < self.list.len() {
            self.list[index].display(self, po);
            0
        }
        else {
            println!("index out of range!");
            1
        }
    }
    pub fn random(&self, po: &Options) -> i32 {
        let mut eligible: Vec<usize> = Vec::new();
        for (i, t) in self.list.iter().enumerate() {
            if po.view_done && t.done {
                eligible.push(i)
            }
            else if po.view_undone && !t.done {
                eligible.push(i)
            }
            // these two ifs mean the program will say "no tasks in list"
            // if there are tasks but they wont be displayed
        }
        match eligible.iter().choose(&mut thread_rng()) {
            Some(t) => {
                self.list[*t].display(self, po);
                0
            }
            None => {
                println!("no tasks in list!");
                1
            }
        }
    }
    pub fn random_top(&self, po: &Options) -> i32 {
        let mut eligible: Vec<usize> = Vec::new();

        for (i, t) in self.list.iter().enumerate() { // select tasks with no parents
            if !self.has_parents(i) {
                if po.view_done && t.done {
                    eligible.push(i)
                }
                else if po.view_undone && !t.done {
                    eligible.push(i)
                }
                // these two ifs mean the program will say "no tasks in list"
                // if there are top level tasks but they wont be displayed
            }
        }
        
        match eligible.iter().choose(&mut thread_rng()) {
            Some(t) => {
                self.list[*t].display(self, po);
                0
            }
            None => {
                println!("no top level tasks in list!");
                1
            }
        }
    }
    pub fn random_bottom(&self, po: &Options) -> i32 {
        let mut eligible: Vec<usize> = Vec::new();

        for (i, t) in self.list.iter().enumerate() { // select tasks with no children
            if t.children.len() == 0 || t.all_children_done(self) {
                if po.view_done && t.done {
                    eligible.push(i)
                }
                else if po.view_undone && !t.done {
                    eligible.push(i)
                }
                // these two ifs mean the program will say "no tasks in list"
                // if there are bottom level tasks but they wont be displayed
            }
        }
        
        match eligible.iter().choose(&mut thread_rng()) {
            Some(t) => {
                self.list[*t].display(self, po);
                0
            }
            None => {
                println!("no bottom level tasks in list! (god help you)");
                1
            }
        }
    }

    // mutating
    pub fn add(&mut self, po: &Options) -> i32 {
        let name = collate_string_vec(&po.string);
        let mut children: Vec<usize> = Vec::new();
        for i in &po.child_tasks {
            if i < &self.list.len() { // check task exists
                children.push(*i)
            }
            else {
                println!("child task {} does not exist!", i);
                return 1
            }
        }
        //let mut parents: Vec<usize> = Vec::new();
        for i in &po.parent_tasks {
            if i < &self.list.len() { // check task exists
                let new_index = self.list.len();
                self.list[*i].children.push(new_index); // add new task as child task to all specified parent tasks
            }
            else {
                println!("parent task {} does not exist!", i);
                return 1
            }
        }
        let t = ToDo {
            name,
            notes: Vec::new(),
            children,
            done: false,
            time_added: Utc::now(),
            time_completed: None
        };

        self.list.push(t);

        0
    }
    pub fn mark(&mut self, po: &Options, done: bool) -> i32 {
        if po.main_index < self.list.len() {
            let t = &mut self.list[po.main_index];
            t.done = done;
            if done {
                t.time_completed = Some(Utc::now())
            }
            else {
                t.time_completed = None
            }
            0
        }
        else {
            println!("index out of range!");
            1
        }
    }
    pub fn add_note(&mut self, po: &Options) -> i32 {
        if po.main_index < self.list.len() {
            self.list[po.main_index].add_note(collate_string_vec(&po.string));
            0
        }
        else {
            println!("index out of range!");
            1
        }
    }
    pub fn remove(&mut self, po: &Options) -> i32 {
        let remove_index = po.main_index;
        if remove_index < self.list.len() {
            for t in &mut self.list {
                t.children.retain(|&x| x != remove_index);
                for i in 0..t.children.len() {
                    if t.children[i] > remove_index {
                        t.children[i] -= 1
                    }
                }
            }
            self.list.remove(remove_index);
            0
        }
        else {
            println!("index out of range!");
            1
        }
    }
    pub fn edit(&mut self, po: &Options) -> i32 {
        let index = po.main_index;
        if index < self.list.len() {
            for c in &po.child_tasks {
                if !self.list[index].children.contains(&c) {
                    self.list[index].children.push(*c)
                }
            }
            for p in &po.parent_tasks {
                if !self.list[*p].children.contains(&index) {
                    self.list[*p].children.push(index)
                }
            }
            for u in &po.unlink_tasks {
                self.list[index].children.retain(|&x| x != *u);
                self.list[*u].children.retain(|&x| x != index)
            }
            0
        }
        else {
            println!("index out of range!");
            1
        }
    }

    pub fn has_parents(&self, task: usize) -> bool {
        self.get_indexes_of_parent_tasks(task).len() != 0
    }
}
