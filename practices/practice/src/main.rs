fn main() {
    // let age = 10;

    // match age {
    //     18 | 19 => println!("You are an adult"),
    //     10..=15 => println!("You are 10!"),
    //     _ => println!("Invalid option!")
    // }

    // let account_balance: Option<i32> = Some(8824628);
    
    // match account_balance{
    //     Some(value: i32) => println!("Value was retrieved!: {}", value)
    //     None => println!("Nothing was retrieved")
    // }

    macro_rules! testing {
        ($arg:expr) => {
            println!("Testing a macro!: {}", $arg);
        };
    }

    testing!("West");
}
