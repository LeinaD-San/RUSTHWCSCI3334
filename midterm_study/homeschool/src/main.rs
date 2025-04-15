/*
Rust works in a ownership model, for memory safety without a garbage collector
compiler enforces to manage how memory is allocated, used, and deallocated
EACH VALUE HAS A OWNER, every piece of data is 'owned' by a single variable at any given time
ONLY one mutable reference(borrow) or any number of immutable references
    either let one thing change the data, or let many look at is but not both at once
once owner goes out of scope, value is dropped
    once variable holding data isn't needed anymore(like at the end of a function), it gets
    automatically freed from memory
Think of a toy box, toys being the data. Ownership is when only ONE kid plays with a toy at
a time. If they hand it to someone else, they do NOT get to touch/play with it anymore unless
it's handed back



fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1;                    // Ownership moves to s2, s1 is invalidated
    println!("{}", s2);            // Prints "hello"
    // println!("{}", s1);         // Error! s1 no longer owns anything
    let s1 = s2;
    println!("{}",s1);
}

Borrowing
When you do not want to give away ownership you can allow others to borrow or tweak data temp.
    Immutable borrow(&T) Anyone can look,but no one can change it.You can have MANY
    Mutable borrow(&mut T) One borrower gets full control to mod it, but no one else can even
    look as it's happening
    
    fn main() {
        let mut s = String::from("hello"); // `mut` makes it mutable
        let r1 = &s;                      // Immutable borrow
        let r2 = &s;                      // Another immutable borrow—fine!
        println!("{} {}", r1, r2);        // Prints "hello hello"
        
        let r3 = &mut s;                  // Mutable borrow
        r3.push_str(", world");           // Modifies s
        // let r4 = &s;                   // Error! Can’t borrow immutably while mutably borrowed
        println!("{}", r3);              // Prints "hello, world"
    }

fn main() {
    let r;                    // r is declared, but not initialized
    {
        let x = 5;           // x owns the value 5
        r = &x;              // r borrows x
    }                        // x goes out of scope, dropped
    // println!("{}", r);   // Error! r would be a dangling reference
}
    if the print line was within the function scope{}, it will be able to print




fn take_ownership(s: String) { // s takes ownership
    println!("{}", s);
} // s is dropped here

fn borrow_it(s: &String) { // s is just borrowed
    println!("{}", s);
} // s isn’t dropped, ownership stays with caller

fn main() {
    let s = String::from("hello");
    borrow_it(&s);     // s is still mine
    take_ownership(s); // s is moved, gone from main
    // println!("{}", s); // Error!
}

------------------------------------------------------------------------------------

fn greet(name: &String){
    println!("Hello, {}!",name);
}

fn shout(name:String){
   println!("{} IS HERE",name.to_uppercase());
}

fn main() {
    let name = String::from("Leinad"); // Replace with your name
    greet(&name);
    shout(name);
}


------------------------------------------------------------------------------------
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("looooong");
    let result = longest(&s1, &s2);
    println!("Longest: {}", result); // Prints "looooong"
}

FUNCTION DEFINITION
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str 
    fn longest: defines a function called longest, compares two strings, returing longer one
    <'a>: lifetime parameter,variable for time,placeholder to track how long references are valid
    x: &'a str: first parameter x,reference to string slice(&str) w/ lifetime 'a. It's borrowed not owned
    y: &'a str: same deal for the second parameter, valid for the same duration
    -> &'a str: this function returns a string slice(&str) w/ lifetime 'a.The returned reference is guaranteed to live
                as long as both x and y.

FUNCTION BODY
if x.len() > y.len() { x } else { y }
    if x.len() > y.len(): Compares lengths of x and y using .len() method
    {x} else {y}: if x is longer, return x; otherwise return y

MAIN FUNCTION
fn main()
    let s1 = String::from("short");Creates a string called s1 w/ value 'short'
    let s2 = String::from('loooooong')Creates as string called s2, with the value 'looooong'
    let = result longest(&s1, &s2): Calls w/ references to s1 and s2,the & borrows them from &str
        (string slices),not string. The lifetime 'a is implicitly scope of main, since s1 and s2 live that long.result get returned &str
    
------------------------------------------------------------------------------------

fn pick_string<'a>(flag:bool ,long_lived: &'a str, short_lived: &'a str) -> &'a str{
    if flag{
        long_lived
    }else{
        short_lived
        //if short_lived was stated it would create an error due to no lifetime being tied to it
        
    }
}
fn main(){
    let outer = String::from("I live long");
    let result;
    {
        let inner = String::from("I die soon");
        result = pick_string(false, &outer, &inner);
    }
    println!("Picked {}", result);
}


//----------------------------------------------------------------------------------------
//Leetcode

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 1. Import HashMap
        use std::collections::HashMap;
        
        // 2. Create an empty HashMap
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        
        // 3. Iterate through the array with index
        for (i, &num) in nums.iter().enumerate() {
            // 4. Calculate the complement
            let complement = target - num;
            
            // 5. Check if complement exists in map
            if let Some(&j) = num_map.get(&complement) {
                // 6. Return solution if found
                return vec![j, i as i32];
            }
            
            // 7. Add current number and index to map
            num_map.insert(num, i as i32);
        }
        
        // 8. Return empty vector if no solution (won't happen per problem constraints)
        vec![]
    }
}
    from leetcode, then translated to regular
    
//1.Import HashMap
use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //2.Create an empty HashMap
    let mut num_map: HashMap<i32, i32> = HashMap::new();
    
    //3.Iterate through the array with index
    for (i, &num) in nums.iter().enumerate() {
        //4.Calculate complement
        let complement = target - num;
        //5.Check if complement exists in map
        if let Some(&j) = num_map.get(&complement) {
            //6.Return solution if found
            return vec![j, i as i32];
        }
        //7.Add current number and index to map
        num_map.insert(num, i as i32);
    }
    //8.Return empty vector if no solution
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    let result = two_sum(nums, target);
    println!("{:?}", result);
}

let mut num_map: HashMap<i32, i32> declares a mutable HashMap
key type: i32(the numbers)
value type: i32(thier indicies)
HashMap::new() creates an empty HashMap



*/



