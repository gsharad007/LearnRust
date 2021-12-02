fn main() {
    let s: &'static str = "  竜, 龍 please  ";
    println!("s='{}' pt={}, len={}", s, s.as_ptr() as usize, s.len());
    let s2 = s.trim();
    println!("s2='{}' pt={}, len={}", s2, s2.as_ptr() as usize, s2.len());
    let s3 = s.replace("please", "pretty please");
    println!("s3='{}' pt={}, len={}", s3, s3.as_ptr() as usize, s3.len());
    
    let mut ss = String::from(s);
    ss.push('&');
    ss.push_str(" sandwich");
    println!("ss='{}' pt={}, len={}", ss, ss.as_ptr() as usize, ss.len());

    let ss2: &str = &ss;
    println!("ss2='{}' pt={}, len={}", ss2, ss2.as_ptr() as usize, ss2.len());
}
