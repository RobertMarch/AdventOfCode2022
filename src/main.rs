mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No day provided!");
    }

    let mut part_to_run = String::from("both");
    let mut inputs_to_run = String::from("both");

    if args.len() >= 3 {
        for arg in args[2..].iter() {
            if [String::from("a"), String::from("b")].contains(&arg) {
                part_to_run = arg.clone();
            }
            if [String::from("example"), String::from("puzzle")].contains(&arg) {
                inputs_to_run = arg.clone();
            }
        }
    }

    let day_to_run: u8 = args[1].parse().unwrap();

    days::run_day(
        day_to_run,
        part_to_run.to_string(),
        inputs_to_run.to_string(),
    );
}
