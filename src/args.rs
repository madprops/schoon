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

    let template = match matches.value_of("template_path")
    {
        Some(path) => s!(path),
        None => s!()
    };

    let target = match matches.value_of("target_path")
    {
        Some(path) => s!(path),
        None => s!()
    };

    if template.is_empty() {exit2("Template not provided.")}
    if target.is_empty() {exit2("Target not provided.")}
    if template == target {exit2("Template and Target are the same.")}

    let is_zip = template.to_lowercase().ends_with(".zip");

    if !is_zip
    {
        let dir = PathBuf::from(&template);
        if !dir.is_dir() {exit2("Template is not a directory.")};
        let files = fs::read_dir(&dir).unwrap();
        if files.count() == 0 {exit2("Template directory is empty.")}
    }

    let dir = PathBuf::from(&target);
    if !dir.is_dir() {exit2("Target is not a directory.")};
    let files = fs::read_dir(&dir).unwrap();
    if files.count() == 0 {exit2("Target directory is empty.")}

    Args
    {
        template, target, is_zip,
    }
}

// Struct for arguments
pub struct Args
{
    pub template: String,
    pub target: String,
    pub is_zip: bool,
}