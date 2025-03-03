fn even(n: i32) -> bool{
    n % 2 == 0
}

fn main(){
    let numbers:[i32;10] = [5,8,24,65,35,22,7,12,19,1];

    for &num in numbers.iter(){
        if num % 3 == 0 && num % 5 == 0 {
            println!{"{}, FizzBuzz",num};
        }
        else if num % 3 == 0{
            println!{"{}, Fizz",num};
        }
        else if num % 5 == 0{
            println!{"{}, Buzz",num};
        }
        else{
            let even_or_odd = if even(num) {"Even"} else {"Odd"};
            println!("{}-> {}",num,even_or_odd);
        }
        
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len(){
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of numbers: {}",sum);

    let mut max_num = numbers[0];
    for &num in numbers.iter(){
        if num > max_num{
            max_num = num;
        }
    }
    println!("Largerst Number: {}",max_num);
}