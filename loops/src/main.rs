fn main() {
    let temp_c = 12.;
    let temp_f = from_celsius(temp_c);
    println!("{}°C is {}°F", temp_c, temp_f);
    println!("{}C again", from_fahrenheit(temp_f));

    for i in 0..=10 {
        println!("The {}th Fibonacci number is {}", i, fibonacci(i));
    }
    christmas_carrol();
}

fn from_fahrenheit(temp_f: f32) -> f32 {
    (temp_f - 32.) * 5. / 9.
}

fn from_celsius(temp_c: f32) -> f32 {
    temp_c * 9. / 5. + 32.
}

fn fibonacci(n: u32) -> u32 {
    let mut start = 0;
    let mut next = 1;

    for _ in 0..n {
        let new = start + next;
        start = next;
        next = new;
    }
    start
}

fn christmas_carrol() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love gave to me", days[i]);
        for j in (0..i+1).rev() {
            if i > 0 && j == 0 {
                println!("And {}.", gifts[j]);
            }
            else{
                println!("{},", gifts[j]);
            }
        }
        println!();
    }
}