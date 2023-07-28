fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);

    // mutability
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // shadowing
    let x : &str = "Hello, World!";
    println!("The value of x is: {}", x);

    // integer arithmentic
    let a: i32 = 5;
    let a = a + 1;
    let a = a * 2;
    let a = a - 1;

    // floating-point arithmetic
    let b = 10.50;
    let b = b * 2.0;
    let b = b - 1.0;
    let b = b / 2.0;
    let b = b % 2.0;

    // write print statements for all arithmetic operations above
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);

    // boolean
    let t = true;
    let f: bool = false;

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;

    // array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let first = a[0];

    fn another_function(x: i32, y: i32) -> i32{
        println!("Another function.");
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
        return x + y;
    }

    // functions
    let sum = another_function(1, 2);
    println!("The value of sum is: {}", sum);

    // if expression
    if sum < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value of element is: {}", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }

    for num in 1..4 {
        println!("{}!", num);
    }

}
