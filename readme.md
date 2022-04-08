# atodo

a simple command line linked todo list manager. every task in the list can have any amount of parent tasks (tasks it is required for) and child tasks (tasks required for it to be completed). i made atodo because a list usually isn't enough for a large project, as some things need other things to be done first. even a list ordered by priority isn't perfect.

this program needs to be built with cargo. read the [rust book](https://doc.rust-lang.org/book/title-page.html) to find out more

## usage

the program takes a command and some other parameters. commands are:

`-l`: list

`-v`: view

`-a`: add

`-A`: add note

`-r`: remove

`-d`: mark as done

`-n`: mark as not done

`-e`: edit

`-R`: view random

`-T`: view random top-level

`-B`: view random bottom-level

`-R` selects a random task and displays it. `-T` selects a task that is not a requirement for anything else. `-B` selects a task with no unfinished requirements. all 3 of these options are affected by `-D` and `-N`. running the program with no arguments is equivalent to `atodo -l`

the available parameters are:

`-t TASK`: the target task to work with

`-p TASK`: add a parent task

`-c TASK`: add a child task

`-u TASK`: unlink a task

`-P PATH`: use a custom path, rather than `./.todo`

`-C`: disable colour

`-D`: view tasks that are already completed

`-N`: do not view tasks that are not completed

by default, non-completed tasks are displayed, and completed tasks are not. bottom-level tasks are displayed in blue, non-completed tasks are red, and completed tasks are green. when in colourless mode, completed tasks are postfixed by `(done)` and bottom-level tasks are postfixed with `(ready)`. on windows builds, colours are disabled entirely

the program also collects all remaining arguments as a single string.

`-p` and `-c` can be used when adding or editing a task. `-u` can only be used while editing. all commands other than `-l`, `-R`, `-T` and `-B` require `-t` to be used.

## examples

`atodo -a -c 10 refactor non-repeating file name method`: add a task that requires task 10 to be completed

`atodo -a -p 4 -p 6 add proper multi-unit parsing`: add a task that is required to complete tasks 4 and 6

`atodo -v -t 0`: view details about task 0

`atodo -e -t 7 -c 3`: edit task 7 to note that it requires task 3 to be completed

`atodo -e -t 1 -p 9 -u 4`: edit task 1 to note that task 9 needs it to be completed, and to remove any links (parent or child) to task 4

`atodo -A -t 5 this may take some time`: add a note to task 5

you may also find it useful to go `atodo > todo.txt`. if you do, i recommend adding `-C` so you don't end up with misbehaving ansi colours
