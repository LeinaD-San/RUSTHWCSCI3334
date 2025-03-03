fn concat_strings(s1: &String, s2: &String) -> String {
    let result = s1.to_owned()+s2;
    return result;
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}
