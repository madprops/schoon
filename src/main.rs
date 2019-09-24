pub const VERSION: &str = "v1.0.0";

mod macros;
mod args;
mod other;

use crate::
{
    args::
    {
        check_args,
        Args,
    },
};

use std::fs;

// Start of the program
fn main() 
{
    schoon(check_args());
}

// Main operation
fn schoon(args: Args)
{
    for file in fs::read_dir(args.template).unwrap()
    {
        let path1 = file.unwrap().path();
        let path2 = args.target.join(path1.file_name().unwrap());

        if path1.is_file() && path2.is_file()
        {
            fs::remove_file(path2).unwrap();
        }

        else if path1.is_dir() && path2.is_dir()
        {
            fs::remove_dir_all(path2).unwrap();
        }
    }
}