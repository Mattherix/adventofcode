mod day1;

fn main() {
    let solution = day1::solve();
    println!("The max calories is {}", solution.0);
    println!("The sum of the 3 max calories is {}", solution.0 + solution.1 + solution.2);
}
