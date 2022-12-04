mod day1;
mod day2;
mod day3;
mod day4;
fn main() {
    let solution = day1::solve();
    println!("The max calories is {}", solution.0);
    println!("The sum of the 3 max calories is {}", solution.0 + solution.1 + solution.2);

    let solution = day2::solve();
    println!("The strategy guide as a score of {} and then {}", solution.0.clone(), solution.1.clone());

    let solution = day3::solve();
    println!("The sum of the priorities of those item and badges are {} and {}", solution.0, solution.1);

    let solution = day4::solve();
    println!("There are {} assignment pairs with one range fully containing the other", solution);

}
