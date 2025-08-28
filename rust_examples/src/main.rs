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

//? Repetitions :
macro_rules! vec_strs {
    (
       // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}

macro_rules! repeat_two {
    ($($i:ident)*, $($i2:ident)*) => {
        $( let $i: (); let $i2: (); )*
    }
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

    let s = vec_strs![1, "a", true, 3.14159f32];
    assert_eq!(s, &["1", "a", "true", "3.14159"]);

    repeat_two!(a b c d e f, u v w x y z ); // this allowed because both repetition are equally
    // repeat_two!( a b c d e f, x y z ); // Not allowed
    // error: meta-variable `i` repeats 6 times, but `i2` repeats 3 times

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
