
use std::process::{Command, Stdio};
use std::env;

///help: displays a help message
fn help() {
    println!("Help");
}

///run_cmd: Is a Command wrapper to run any needed cmd and output to stdio/stderr
fn run_cmd(cmd: &str, argsv: Vec<&str>) {
    let mut output = Command::new(cmd)
                         .args(&argsv)
                         .stdout(Stdio::inherit())
                         .stderr(Stdio::inherit())
                         .spawn()
                         .unwrap();

    // let status = output.wait();
    // println!("{:?}", status);
}

///not_impl: Displays a msg for features not yey implemented
fn not_impl() {
    println!("Not yet implemented");
}

///snapcraft_cmd: Builds the Vec to run a snapcraft-cli arg
fn snapcraft_cmd(cmd1: &str, cmd2: &str) -> Vec<String> {
    let pwd = env::current_dir().unwrap();
    let vol = format!("{}:/root/snappy-project", pwd.display()).to_owned();
    let cmd_all = format!("snapcraft {} {}", cmd1, cmd2).to_owned();
    let argsv: Vec<String> = vec![String::from("run"),
                                  String::from("-t"),
                                  String::from("-v"),
                                  vol.clone(),
                                  String::from("snappy"),
                                  String::from("/bin/bash"),
                                  String::from("-c"),
                                  cmd_all.clone()];
    argsv
}

///snapcraft: Parses any snapcraft commands
fn snapcraft(args: Vec<String>) {
    if args.len() > 2 {
        match args[2].as_ref() {
            "init" => {
                let cmd: &str = "docker";
                let argsv = snapcraft_cmd("init", "");
                let stat_argsv: Vec<&str> = argsv.iter().map(|s| &**s).collect();
                run_cmd(cmd, stat_argsv);
            }
            "build" => not_impl(),

            _ => help(),
        }
    } else {
        let cmd: &str = "docker";
        let argsv: Vec<&str> = vec!["run", "-t", "snappy", "/bin/bash", "-c", "\"snapcraft\""];
        run_cmd(cmd, argsv);
    }
}

///init: is used to build the needed Docker container
fn init() {
    let cmd: &str = "docker";
    let argsv: Vec<&str> = vec!["build", "-t", "snappy", "settings"];
    run_cmd(cmd, argsv);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_ref() {
            "init" => init(),
            "sc" => snapcraft(args),

            _ => help(),
        }
    } else {
        help();
    }
}
