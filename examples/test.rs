fn main() {
    let s = "babad".to_string();
    let s = s.chars().collect::<Vec<_>>();
    let a = s[0..3].iter().collect::<String>();
    println!("{}",a);
}