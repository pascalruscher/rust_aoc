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
    let available_years = ["2015", "2021", "2022"];
    let args = Args::parse();

    if args.year.len() != 4 {
        panic!("Invalid year: {} - Year must have 4 digits", args.year);
    }

    if available_years.contains(&args.year.as_str()) == false {
        panic!("Invalid year: {} - Available years are {:?}", args.year, available_years);
        
    }

    let day = if args.day.len() == 2 {
        "0".to_string() + &args.day
    } else if args.day.len() == 3 {
        args.day.to_string()
    } else {
        if !args.day.contains("a") && !args.day.contains("b") {
            panic!("Invalid day: {} - Day must have a or b as suffix, i.e. 01a", args.day);
        }
        panic!("Invalid day: {} - Day must have 1 or 2 digits and end with a or b, i.e. 01a", args.day);
    };

    match args.year.as_str() {
        "2015" => {
            println!("{}", year2015::solution(&day));
        },
        "2021" => {
            println!("{}", year2021::solution(&day));
        },
        "2022" => {
            println!("{}", year2022::solution(&day));
        },
        _ => {}
    }
}