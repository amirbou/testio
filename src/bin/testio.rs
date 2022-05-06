use std::vec;

use fuser::MountOption;
use testio::{testfs::{TestFs, FsFile}, files::EmptyROFile, files::{PrepopulatedFile, ReadX, WriteX}};
use clap::{arg, Command};

fn create_files() -> Vec<Box<dyn FsFile>>
{
    let mut files : Vec<Box<dyn FsFile>> = Vec::with_capacity(20);
    
    let empty_file = String::from("readempty");
    files.push(Box::new(EmptyROFile::new(empty_file)));
    
    let name = String::from("readregular");
    let data = name.clone() + "\n".into();
    files.push(Box::new(PrepopulatedFile::new(name.into(), data)));

    let name = String::from("readone");
    let data = "a".repeat(10_000);
    files.push(Box::new(ReadX::new(name, data, |_| 1)));

    for i in 2..10 {
        let data = "a".repeat(100_000);
        files.push(
            Box::new(
                ReadX::new(
                    format!("readX{}", i),
                    data,
                    move |size| {
                        std::cmp::max(size / i, 1)
                    }
                )
            )
        );
    }

    files.push(Box::new(WriteX::new("writeone".into(), |data| &data[..1])));

    for i in 2..10 {
        files.push(
            Box::new(
                WriteX::new(
                    format!("writeX{}", i),
                    move |data| {
                        let size = std::cmp::max(data.len() / i, 1);
                        &data[..size]
                    }
                )
            )
        );
    }

    files
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
