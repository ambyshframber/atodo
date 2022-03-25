# atodo

a simple command line linked todo list manager

this program needs to be built with cargo. read the [rust book](https://doc.rust-lang.org/book/title-page.html) to find out more

## usage

the program takes a command and some other parameters. commands are:

`-l`: list

`-v`: view

`-a`: add

`-N`: add note

`-r`: remove

`-d`: mark as done

`-n`: mark as not done

`-e`: edit

`-R`: view random

`-T`: view random top-level

`-B`: view random bottom-level

the available parameters are:

`-t TASK`: the target task to work with

`-p TASK`: add a parent task

`-c TASK`: add a child task

`-u TASK`: unlink a task

`-P PATH`: use a custom path, rather than `./.todo`

the program also collects all remaining arguments as a single string.

`-p` and `-c` can be used when adding or editing a task. `-u` can only be used while editing. all commands other than `-l`, `-R`, `-T` and `-B` require `-t` to be used.

## examples

`atodo -a -c 10 refactor non-repeating file name method`: add a task that requires task 10 to be completed

`atodo -a -p 4 -p 6 add proper multi-unit parsing`: add a task that is required to complete tasks 4 and 6

`atodo -v -t 0`: view details about task 0

`atodo -e -t 7 -c 3`: edit task 7 to note that it requires task 3 to be completed

`atodo -e -t 1 -p 9 -u 4`: edit task 1 to note that task 9 needs it to be completed, and to remove any links (parent or child) to task 4

`atodo -N -t 5 this may take some time`: add a note to task 5