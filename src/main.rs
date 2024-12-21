pub mod aoc;

fn usage(exe: &str) -> ! {
    use std::path::Path;
    let exe = Path::new(exe).file_name().unwrap().to_str().unwrap();
    println!("usage: {exe} <day (1-25)>");
    std::process::exit(0)
}

// macro_rules! aoc {
//     ($day:expr, [$( $x:expr ),* ]) => {
//         {
//             match $day {
//                 $($x => aoc::day_$x::solve(),)*
//                 _ => println!("day {} not implemented", $day)
//             }
//         }
//     };
// }

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
        7 => aoc::day_7::solve(),
        8 => aoc::day_8::solve(),
        9 => aoc::day_9::solve(),
        10 => aoc::day_10::solve(),
        11 => aoc::day_11::solve(),
        12 => aoc::day_12::solve(),
        13 => aoc::day_13::solve(),
        14 => aoc::day_14::solve(),
        15 => aoc::day_15::solve(),
        16 => aoc::day_16::solve(),
        _ => println!("day {day} not implemented")
    }
}
