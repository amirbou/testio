use std::vec;

use fuser::MountOption;
use testio::{testfs::{TestFs, FsFile}, files::EmptyROFile, files::{PrepopulatedFile, ReadOneFile, WriteOneFile}};
use clap::{arg, Command};



fn create_files() -> Vec<Box<dyn FsFile>>
{
    let mut files = Vec::with_capacity(5);
    let empty_names = vec!["empty1".into()];
    for name in empty_names {
        files.push(Box::new(EmptyROFile::new(name)) as Box<dyn FsFile>);
    }
    
    let full_names = vec!["full1", "full2"];
    for name in full_names {
        let data = String::from(name) + "\n".into();
        files.push(Box::new(PrepopulatedFile::new(name.into(), data)) as Box<dyn FsFile>);
    }

    let readone_names = vec!["readone1", "readone2"];
    for name in readone_names {
        let data = String::from(name) + "\n".into();
        files.push(Box::new(ReadOneFile::new(name.into(), data)) as Box<dyn FsFile>);
    }

    let writeone_names = vec!["writeone1", "writeone2"];
    for name in writeone_names {
        files.push(Box::new(WriteOneFile::new(name.into())) as Box<dyn FsFile>);
    }

    files
    
    // let full_names = vec![("full1".into(), "")]
    // , "f3".into(), "f4".into(), "f5".into()];
}

fn main() {
    let matches = Command::new("TestFs")
        .version("wip")
        .author("AmirB")
        .about("Mounts a fuse that produces edge cases for simple io functions on linux")
        .arg(arg!(<path> "The path to mount the fuse on"))
        .get_matches();

    let path: String = matches.value_of("path").expect("required").into();

    let mut fs = TestFs::new();
    let files = create_files();
    for file in files {
        fs.add_file(file);
    }
    env_logger::init();
    let options  = vec![
        MountOption::FSName("testfs".into()),
        MountOption::AllowOther,
        MountOption::NoAtime,
        MountOption::AutoUnmount,
        MountOption::DefaultPermissions,
    ];
    fuser::mount2(fs, path, &options).unwrap();
}
