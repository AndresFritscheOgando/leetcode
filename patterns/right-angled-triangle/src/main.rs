fn main() {
    right_angle(30)
}

fn right_angle(num: i32) {

    for i in 0..num{
        for j in 0..i {
            print!("* ")
        }
        println!("* ")
    }
}


