use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub fic: String,
    pub count_line: bool,
}

impl Default for Paramcli {
    fn default() -> Self {
        Paramcli::new()
    }
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut fic = String::new();
        let mut count_line = false;

        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args()
            .take(1)
            .next()
            .unwrap_or_else(|| String::from("wc"));
        if args.len() > 2 {
            println!("Error: too many parameters");
            help(&name);
        }
        for arg in args {
            if arg == "/?"
                || arg == "-?"
                || arg.to_lowercase() == "/help"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if arg == "-l" {
                count_line = true;
                continue;
            }
            fic = arg;
        }
        //checks
        if !fic.is_empty() {
            //check if file exists
            if File::open(&fic).is_err() {
                println!("Error file {} doesn't exists or unereadable", &fic);
                help(&name);
            };
        }
        Paramcli { fic, count_line }
    }
}

fn help(name: &str) {
    println!("{} 1.0 (2020)", name);
    println!("syntax : {} [file] [-l]", name);
    println!("parameters between [] are optionnals");
    println!("------------------------------------");
    println!("fic: file to work with (if non use stdin");
    println!("-l: count lines (if not set count words");
    std::process::exit(0);
}
