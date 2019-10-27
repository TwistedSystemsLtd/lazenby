use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    command: String
}

fn main() {
    let args = Cli::from_args();

    println!("Command: {}", args.command);
}
