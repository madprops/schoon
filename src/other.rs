use crate::
{
    p,
};

use std::process;

// Centralized function to exit the program
pub fn exit() -> !
{
    process::exit(0)
}

// Show a message before exit
pub fn exit2(s: &str)
{
    p!(s); exit()
}