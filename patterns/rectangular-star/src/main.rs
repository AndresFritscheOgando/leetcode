fn main() {
    rectangular_star(10)
}

fn rectangular_star(num: i32) {
    for _i in 0..num {
        for _j in 0..num {
            print!("* ");
        }
        println!("")
    }
}


