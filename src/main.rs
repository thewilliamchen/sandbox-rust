mod clazz;
fn main() {
    let a = 69;
    println!("Hello, world!");
    print!("{}", test());
}

fn test() -> i32 {
    0
}

fn me(space: i32) -> bool {
    match space {
        69 => {
            return true;
        }, _ => {
            return false;
        }
    }
}


fn <'a> lifetime('a ) -> {

}