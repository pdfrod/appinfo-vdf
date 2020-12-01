mod vdf;

use std::fs::File;
use std::io::Read;
use std::env;
use std::io;
use vdf::VDF;
use vdf::printer::print;
use vdf::reader::read;
use vdf::updater::update;
use vdf::writer::write;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cmd = if args.len() > 1 { &args[1] } else { "" };
    let mut success = false;

    match cmd {
        "fix" => {
            if args.len() == 4 {
                let vdf = read_file(&args[2])?;
                let new_vdf = update(&vdf);
                write_file(&args[3], &new_vdf)?;
                success = true;
            }
        },
        "print" => {
            if args.len() == 3 {
                let vdf = read_file(&args[2])?;
                print(&vdf);
                success = true;
            }
        }
        _ => {}
    }

    if !success { print_help(&args[0]); }
    Ok(())
}

fn print_help(self_path: &str) {
    let name = self_path.split(|c| c == '/').next_back().unwrap();
    println!("usage:");
    println!("  {} fix INPUT OUPUT", name);
    println!("    Patch appinfo.vdf to make Assassin's Creed 2 work. \n");
    println!("  {} print INPUT", name);
    println!("    Parse and print the contents of appinfo.vdf.");
}

fn read_file(path: &str) -> io::Result<VDF> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let (_, vdf) = read(&buffer)?;
    Ok(vdf)
}

fn write_file(path: &str, vdf: &VDF) -> io::Result<()> {
    let mut file = File::create(path)?;
    write(&mut file, &vdf)?;
    Ok(())
}
