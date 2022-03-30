#[derive(Clone, Debug)]
pub enum Command {
    List, // -l
    View, // -v
    Add, // -a
    AddNote, // -N
    Remove, // -r
    MarkDone, // -d
    MarkNotDone, // -n
    Edit, // -e
    //AddParents, // -p
    //AddChildren, // -c
    //Unlink, // -u
    Random, // -R
    RandomTopLevel, // -T
    RandomBottomLevel // -B
}
impl Default for Command {
    fn default() -> Self {Self::List}
}

pub mod colour {
    pub const COLOUR_RESET: &'static str = "\x1b[0m";
    pub const RED: &'static str = "\x1b[31;1m";
    pub const GREEN: &'static str = "\x1b[32;1m";
}

#[derive(Default, Debug)]
pub struct Options {
    pub command: Command,
    pub main_index: usize,
    pub unlink_tasks: Vec<usize>,
    pub parent_tasks: Vec<usize>,
    pub child_tasks: Vec<usize>,
    pub string: Vec<String>,
    pub todo_file_path: Option<String>,
    pub view_done: bool,
    pub view_undone: bool,
    pub colours: bool
}

pub fn collate_string_vec(v: &Vec<String>) -> String {
    let mut ret = String::new();
    for s in v {
        ret.push_str(s);
        ret.push(' ')
    }
    ret
}

pub fn will_display(done: bool, po: &Options) -> bool {
    (done && po.view_done) || (!done && po.view_undone)
}
