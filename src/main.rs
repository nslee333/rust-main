pub fn fibonacci() {
    let mut a = 0;
    let mut b = 1;
    let mut result = 0;

    while result <= 1000 {
        result = a + b;
        a = b;
        b = result;
        println!("{result}");
    }
}

fn main() {
    fibonacci();
}
