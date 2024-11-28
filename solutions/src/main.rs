fn main() {
    println!("This is the list of problems solved here ...");

    println!("- (sum): Sum of sum-series of first N Natural numbers");

    let option: &str = "sum";

    match option {
        "sum" => sum_of_sum_series(1),
        &_ => println!("not found"),
    }
}

fn sum_of_sum_series(n: i32) {
    println!("{}", n);
}
