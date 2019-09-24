use crate::
{
    s,
    VERSION,
    other::
    {
        exit2,
    },
};

use std::
{
    fs,
    path::PathBuf, 
};

use clap::{App, Arg};

// Starts the argument system
pub fn check_args() -> Args
{
    let matches = App::new("schoon")
    .version(VERSION)
    .about("Removes files on a dir based on the files of another dir")
    .arg(Arg::with_name("template_path")
        .help("Dir to use as base to delete files")
        .required(true)
        .index(1))
    .arg(Arg::with_name("target_path")
        .help("Dir where files will be removed")
        .required(true)
        .index(2))
    .get_matches();

    let stemplate = match matches.value_of("template_path")
    {
        Some(path) => s!(path),
        None => s!()
    };

    let starget = match matches.value_of("target_path")
    {
        Some(path) => s!(path),
        None => s!()
    };

    if stemplate.is_empty() {exit2("Template not provided.")}
    if starget.is_empty() {exit2("Target not provided.")}
    if stemplate == starget {exit2("Template and Target are the same.")}

    let template = PathBuf::from(stemplate);
    if !template.is_dir() {exit2("Template is not a directory.")};
    let files = fs::read_dir(&template).unwrap();
    if files.count() == 0 {exit2("Template directory is empty.")}

    let target = PathBuf::from(starget);
    if !target.is_dir() {exit2("Target is not a directory.")};
    let files = fs::read_dir(&target).unwrap();
    if files.count() == 0 {exit2("Target directory is empty.")}

    Args
    {
        template, target,
    }
}

// Struct for arguments
pub struct Args
{
    pub template: PathBuf,
    pub target: PathBuf,
}