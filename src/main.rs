use std::env;
use std::io::{stdin, stdout, Write};
use std::option::Option::None;
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        print!("> ");
        stdout()
            .flush()
            .expect("Not all bytes could be written due to I/O errors or EOF being reached.");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("letme couldn't read the line");

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().expect("Command wasn't correct");
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.expect(""))
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            final_command.wait().expect("Command wasn't running");
        }
    }
}
