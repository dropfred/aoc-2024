pub mod aoc;

fn usage(exe: &str) -> ! {
    use std::path::Path;
    let exe = Path::new(exe).file_name().unwrap().to_str().unwrap();
    println!("usage: {exe} <day (1-25)>");
    std::process::exit(0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        usage(&args[0]);
    }

    let day = match args[1].parse::<u8>() {
        Ok(n) if (1..=25).contains(&n) => n,
        _ => {
            usage(&args[0]);
        }
    };

    match day {
        1 => aoc::day_1::solve(),
        2 => aoc::day_2::solve(),
        4 => aoc::day_4::solve(),
        5 => aoc::day_5::solve(),
        6 => aoc::day_6::solve(),
        _ => println!("day {day} not implemented")
    }
}
