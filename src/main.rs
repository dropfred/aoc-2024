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
    
    // TODO: macro aoc!(day, [1, 2, 4..=8]);
    match day {
        1 => aoc::day_1::solve(),
        2 => aoc::day_2::solve(),
        4 => aoc::day_4::solve(),
        5 => aoc::day_5::solve(),
        6 => aoc::day_6::solve(),
        7 => aoc::day_7::solve(),
        8 => aoc::day_8::solve(),
        9 => aoc::day_9::solve(),
        _ => println!("day {day} not implemented")
    }
}
