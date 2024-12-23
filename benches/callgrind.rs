use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use std::hint::black_box;

fn load_input(day: u8) -> String {
    let input_file_path = format!("input/2024/day{day}.txt");
    if let Ok(input) = std::fs::read_to_string(input_file_path) {
        return input;
    }

    let session = std::env::var("AOC_SESSION")
        .expect("AOC_SESSION should be set when inputs are not stored locally");
    let client = ureq::AgentBuilder::new().build();
    let url = format!("https://adventofcode.com/2024/day/{day}/input");

    let response = client
        .get(&url)
        .set("Cookie", &format!("session={}", session))
        .call()
        .expect("should be able to send request");
    assert_eq!(response.status(), 200);

    response
        .into_string()
        .expect("should be able to read response body")
}

macro_rules! benchmark_days {
    ($($day:literal),*) => {
        paste::paste! {
            $(

            #[library_benchmark]
            #[bench::my_input(load_input($day))]
            fn [<day $day part1>](input: String) {
                use aoc24::[<day $day>]::part1;
                black_box(part1(black_box(&input)));
            }

            #[library_benchmark]
            #[bench::my_input(load_input($day))]
            fn [<day $day part2>](input: String) {
                use aoc24::[<day $day>]::part2;
                black_box(part2(black_box(&input)));
            }
            )*

            library_benchmark_group!(name = days; benchmarks = $([<day $day part1>], [<day $day part2>]),*);
        }
    };
}

benchmark_days!(
    1, 2, 3, 4, 5, 6, /*7,*/ 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23
);

main!(library_benchmark_groups = days);
