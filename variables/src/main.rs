// aritmetic
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a*b+b*c+c*a;
}


//Inferencia de tipos

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}



fn main() {
    let mut x: i32 = 10;
    println!("x: {x}");

    x = 25;
    println!("x: {x}");

    println!("result: {}", interproduct(120, 100, 248));

    //strings
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    println!("{:?}", &sentence[11..15]);

    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");

    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    //takes_u32(y);

}
