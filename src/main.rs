use std::fmt::Display;

struct Person {
    name: String,
    age: i32,
}
// Is this a comment?
// Yes!
// This is an instance of the typeclass display.
impl Display for Person {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} ({} years old)", self.name, self.age)
    }
}

fn main () {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // We can print this because of Display.
    println!("Person: {}:", alice);
    p_strings();

    //iterator looping
    // 1-10
    for i in 1..11 { println!("{}", i) }

    // lets fizzbuzz like it's 2010
    //

    for i in 1..100 {
        match (i % 3==0, i%5==0) {
        (true, true) => println!("FizzBuzz"),
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        (_, _) => println!("{}", i),
        }
    }



}


// What is references
// lmao

fn p_strings() {
    let s : String = String::from("Here's something");
    printer(&s);
    printer(&s);
}

fn printer(s: &str) {
    println!("The value is {}", &s)
}
