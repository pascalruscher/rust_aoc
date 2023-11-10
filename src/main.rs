mod year2015;
mod year2021;
mod year2022;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: String,
    #[arg(short, long)]
    day: String,
}

fn main() {
    let args = Args::parse();
    match args.year.as_str() {
        "2015" => {
            println!("{}", year2015::solution(&args.day));
        },
        "2021" => {
            println!("{}", year2021::solution(&args.day));
        },
        "2022" => {
            println!("{}", year2022::solution(&args.day));
        },
        _ => {}
    }
}