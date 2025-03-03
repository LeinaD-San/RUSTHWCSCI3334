#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    *total = (low..=high).sum();
    0;
}

fn main() {
    let mut total = 0;
    sum(&mut total,0,100);
    println!("Sum: {}",total);
}