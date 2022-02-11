fn main() {
    calc(8, 28, 13, 9, 6); //initial values calculated by hand
}

fn calc(prev_n: isize, cur_total: isize, num_one: isize, num_two: isize, num_three: isize) {
    let cur_n = prev_n + 1;
    let cur_total = cur_total + prev_n;
    let num_three = num_three + 1;
    let num_two = num_two + 2;
    let num_one = num_one + prev_n - 3;
    
    let new_avg:f64 = ((num_one as f64 * 1.0) + (num_two as f64 * 2.0) + (num_three as f64 * 3.0) + 4.0) / (cur_total as f64);
    println!("{}", new_avg);
    if new_avg <= (4.0/3.0)  { 
        println!("Found it! at N: {}", cur_n);
        //if cur_n < 25 { //Do it one more time for a nicer number
        //    calc(cur_n, cur_total, num_one, num_two, num_three);
        //}
        return;
    } else {
        calc(cur_n, cur_total, num_one, num_two, num_three);
    }
}