#![deny(clippy::all)]

/*
Wow this one was way different for me. I'm not sure why I struggled with it so much,
but the syntax for the basic for loop is odd.
*/

fn main() {
    // Basic for loop
    // i is created. This is the value that will iterate each loop.
    // Then the range of the loop is declared so starting at 1
    // Then moving up to 5
    // The .. says that we are going up to a number and the = says that we are including the 5
    // so i will be 1, 2, 3, 4, 5
    for i in 1..=5 {
        println!("Iteration {}", i);
    }

    // Loop with a range and step
    // Same as before but we are only going up until 10 and no equals means we are exuding the 10.
    // .step_by(2) means we will be adding 2 to i each time.
    for i in (0..10).step_by(2) {
        println!("Even Number: {}", i);
    }

    // Loop through items in an array
    // Create a basic array
    let numbers = [10, 20, 30, 40, 50];

    // This is the same syntax as the C# foreach loop.
    // Remember to use .iter() so that it will iterate thought the array.
    for number in numbers.iter() {
        println!("Array Element: {}", number);
    }

    // While loop
    // Using a mutable variable so it can change.
    let mut counter = 0;
    // Syntax is the same as C#
    while counter < 3 {
        println!("While Loop Iteration {}", counter);
        counter += 1;
    }

    // Loop with break and continue
    // Syntax is the same as the basic loop then.
    // We can use continue to finnish the iteration and start on the next one.
    // break will terminate the loop.
    for i in 1..=10 {
        if i % 2 == 0 {
            // Skip even numbers
            continue;
        }

        println!("Odd Number: {}", i);

        if i == 7 {
            // Break the loop when i is 7
            break;
        }
    }
}
