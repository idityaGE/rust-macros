macro_rules! four {
    () => {
        1 + 3
    };

    // notice I have added the parentheses to return
    ($prar:ident) => {{
        1 + 3
    }};
}



fn main() {
    // four!();
    // four![];
    // four! {} // notice there no semicolon

    four!(true);
    four! {true};
    four![false];


    println!("Hello, world!");
}
