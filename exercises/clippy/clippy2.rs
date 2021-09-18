// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)


fn main() {
    let mut res = 42;
    let mut option = Some(12);
    while let Some(x) = option {   
        if x > 0 {
            res += x;
            option = Some(x - 1);
        } else {
            option = None;
        }
    };
    println!("{}", res);
}
