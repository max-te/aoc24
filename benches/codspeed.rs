use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reqwest::blocking::Client;

fn load_input(day: u8) -> String {
    let input_file_path = format!("input/2024/day{day}.txt");
    if let Ok(input) = std::fs::read_to_string(input_file_path) {
        return input;
    }

    let session = std::env::var("AOC_SESSION")
        .expect("AOC_SESSION should be set when inputs are not stored locally");
    let client = Client::new();
    let url = format!("https://adventofcode.com/2024/day/{day}/input");

    client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .expect("should be able to send request")
        .error_for_status()
        .expect("status should be successful")
        .text()
        .expect("should be able to read response body")
}

macro_rules! benchmark_days {
    ($($day:literal),*) => {
        paste::paste! {
            $(
            fn [<day $day>](c: &mut Criterion) {
                let input = load_input($day);
                use aoc24::[<day $day>]::{part1, part2};
                c.bench_function(&format!("day {} part 1", $day), |b| b.iter(||
                    part1(black_box(&input))
                ));
                c.bench_function(&format!("day {} part 2", $day), |b| b.iter(||
                    part2(black_box(&input))
                ));

            }
            )*

            criterion_group!(days, $([<day $day>],)*);
        }
    };
}

benchmark_days!(1, 2, 3, 4, 5, 6);
criterion_main!(days);
