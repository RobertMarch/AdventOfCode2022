mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No day provided!");
    }

    let day_to_run: u8 = args[1].parse().unwrap();

    days::run_day(day_to_run);
}
