pub const VERSION: &str = "v1.1.0";

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

use std::
{
    fs::
    {
        self, File,
    },
    path::PathBuf,
};

use zip::ZipArchive;

// Start of the program
fn main() 
{
    schoon(check_args());
}

// Main operation
fn schoon(args: Args)
{
    let mut files: Vec<(String, bool)> = vec![];

    if args.is_zip
    {
        // Get zip root files
        let file = File::open(args.template).unwrap();
        let mut zip = ZipArchive::new(file).unwrap();

        for i in 0..zip.len()
        {
            let zfile = zip.by_index(i).unwrap();
            files.push((s!(zfile.name()), zfile.is_dir()));
        }
    }

    else
    {
        // Get dir root files
        for dfile in fs::read_dir(args.template).unwrap()
        {
            let path = dfile.unwrap().path();
            files.push((s!(path.file_name().unwrap().to_str().unwrap()), path.is_dir()));
        }
    }

    let tdir = PathBuf::from(args.target);

    for file in files
    {
        let tfile = tdir.join(file.0);

        // Remove dir
        if file.1 && tfile.is_dir()
        {
            fs::remove_dir_all(tfile).unwrap();
        }

        // Or remove file
        else if !file.1 && tfile.is_file()
        {
            fs::remove_file(tfile).unwrap();
        }
    }
}