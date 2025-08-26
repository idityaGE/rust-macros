use rust_examples::*;
// A tool for debugging : `cargo expand`

macro_rules! say_hello {
    () => {
        println!("Hello")
    };
}

macro_rules! create_function {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("you called ${:?}()", stringify!($fn_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
    };

    ($left:expr; or $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
    };
}

macro_rules! find_min {
    // base case
    ($x:expr) => {
        $x
    };

    ($x:expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y), +))
    };
}

macro_rules! vector {
    // Case 1: vector![1; 3] => [1,1,1]
    ($elem:expr; $count:expr) => ({
        let mut temp_vec = Vec::new();
        temp_vec.reserve($count);
        for _ in 0..$count {
            temp_vec.push($elem);
        }
        temp_vec
    });

    // Case 2: vector![1,2,4,5] or vector![]
    ($($elem:expr),* $(,)?) => ({
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($elem);
        )*
        temp_vec
    });
}

pub trait MyDebug {
    fn my_fmt(&self);
}

#[derive(MyDebug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(MyDebug)]
struct Coords(f64, f64);

#[derive(MyDebug)]
struct Empty;

#[log_call]
pub fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

// use : cargo expand --bin rust_examples
fn main() {
    say_hello!();

    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));

    let vv = vector![1, 3, 5, 5, 6, 3,];
    println!("{:?}", vv);

    let reversed = reverse_exprs!(1, 3, 5);
    println!("{:?}", reversed);

    let p = Point { x: 10, y: 20 };
    p.my_fmt();

    let c = Coords(1.0, 2.5);
    c.my_fmt();

    let e = Empty;
    e.my_fmt();

    calculate_sum(3, 9);
}
