fn main() {
    let fun = 1;
    let x =50;
    
    if fun == 1 {simple_if(x);}  else {expresionif(x);}

}

fn simple_if(x:u32){
    if x < 20 {
        println!("small");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}

fn expresionif(x:u32){
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}