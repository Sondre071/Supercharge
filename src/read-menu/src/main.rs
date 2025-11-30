use clap::Parser;

mod init;
mod input;
mod menu;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    header: Option<String>,

    #[arg(long, value_delimiter = ',')]
    subheaders: Option<Vec<String>>,

    #[arg(long, value_delimiter = ',')]
    options: Vec<String>,

    #[arg(long)]
    exit_option: Option<String>,

    #[arg(long)]
    width: usize,
}

fn main() {
    let args = Cli::parse();

    menu::run(
        args.header,
        args.subheaders,
        args.options,
        args.exit_option,
        args.width,
    );
}
