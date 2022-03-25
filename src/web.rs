use serde::{Serialize, Deserialize};
use std::fs;
use rand::{prelude::IteratorRandom, thread_rng};

use crate::todo::ToDo;
use crate::utils::{Options, collate_string_vec};

#[derive(Serialize, Deserialize)]
pub struct Web {
    pub list: Vec<ToDo>
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
    pub fn list(&self) {
        for t in &self.list {
            t.display_short(self)
        }
    }
    pub fn view(&self, index: usize) -> i32 {
        if index < self.list.len() {
            self.list[index].display(self);
            0
        }
        else {
            println!("index out of range!");
            1
        }
    }
    pub fn random(&self) -> i32 {
        match self.list.iter().choose(&mut thread_rng()) {
            Some(t) => {
                t.display(self);
                0
            }
            None => {
                println!("no tasks in list!");
                1
            }
        }
    }
    pub fn random_top(&self) -> i32 {
        let mut eligible: Vec<usize> = Vec::new();

        for (i, _) in self.list.iter().enumerate() {
            if self.get_indexes_of_parent_tasks(i).len() == 0 {
                eligible.push(i)
            }
        }
        
        match eligible.iter().choose(&mut thread_rng()) {
            Some(t) => {
                self.list[*t].display(self);
                0
            }
            None => {
                println!("no top level tasks in list!");
                1
            }
        }
    }
    pub fn random_bottom(&self) -> i32 {
        let mut eligible: Vec<usize> = Vec::new();

        for (i, t) in self.list.iter().enumerate() {
            if t.children.len() == 0 {
                eligible.push(i)
            }
        }
        
        match eligible.iter().choose(&mut thread_rng()) {
            Some(t) => {
                self.list[*t].display(self);
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
            if i < &self.list.len() {
                children.push(*i)
            }
            else {
                println!("child task {} does not exist!", i);
                return 1
            }
        }
        let mut parents: Vec<usize> = Vec::new();
        for i in &po.parent_tasks {
            if i < &self.list.len() {
                parents.push(*i)
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
            done: false
        };

        self.list.push(t);

        0
    }
    pub fn mark(&mut self, po: &Options, done: bool) -> i32 {
        if po.main_index < self.list.len() {
            self.list[po.main_index].done = done;
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
                for i in 0..t.children.len() {
                    if t.children[i] > remove_index {
                        t.children[i] -= 1
                    }
                    if t.children[i] == remove_index {
                        t.children.remove(i);
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

}