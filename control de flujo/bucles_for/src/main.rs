fn main() {
    let bucle = 3;
    
    if bucle == 1 {
        while_bucle();
    }

    else if bucle == 2 {
        for_bucle();
    }

    else if bucle == 3 {
        loop_bucle();
    }

}


//bucle while
fn while_bucle(){
    println!("Soy el while!");
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");
}

//bucle for
fn for_bucle(){
    for x in 1..=5 {
        println!("x: {x}");
    }
}

//loop -> The loop statement just loops forever, until a break.

fn loop_bucle(){
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}
