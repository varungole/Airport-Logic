use std::io;
use std::thread;
use std::time::Duration;

fn gate()
{
   println!("Your terminal is B");
   println!("Please go to gate 60F");
   println!("");
}

fn security()
{
println!("Scanning bag one...");
thread::sleep(Duration::from_secs(3));
println!("Scanned");

println!("Scanning bag two...");
thread::sleep(Duration::from_secs(3));
println!("Scanned");

println!("Scanning handbag...");
thread::sleep(Duration::from_secs(3));
println!("Scanned");

println!("Hand tapping traveller");
thread::sleep(Duration::from_secs(3));
println!("Scanned");

println!("You are through security!")
}


fn checkin() -> i32
{
 println!("Welcome to the check-in process");

 println!("Note : Following items are not allowed in your checkin bag!");
 print!("1. Explosives 2. E-cigarettes 3. Guns 4. Water ");

 println!("Do you have any of the above items in your check-in bag? 1.Yes 2.No");
 let mut choice = String::new();
 io::stdin().read_line(&mut choice).expect("You did not enter a choice");

 let answer : i32 = choice.trim().parse().unwrap();

 if answer == 1
 {
    println!("Remove and throw them away immediately");
    checkin();
 }

 println!("Please enter the total number of bags you have:");
let mut bags = String::new();
io::stdin().read_line(&mut bags).expect("You did not enter your bags");
let bags_final : i32 = bags.trim().parse().unwrap();

if bags_final>2
{
    println!("You are allowed only two bags");
    checkin();
}

println!("Please enter the weight of the first bag:");
 let mut w1 = String::new();
 io::stdin().read_line(&mut w1).expect("You did not enter a choice");
 let weight_1 : i32 = w1.trim().parse().unwrap();

 if weight_1 > 23
 {
    println!("The weight of the bag exceeded the limit");
    checkin();
 }

 println!("Please enter the weight of the second bag:");
 let mut w2 = String::new();
 io::stdin().read_line(&mut w2).expect("You did not enter a choice");
 let weight_2 : i32 = w2.trim().parse().unwrap();
 if weight_2 > 23
 {
    println!("The weight of the second bag exceed the limit");
    checkin();
 }

println!("Your bags are checked-in for the flight!");

return 1;


}


fn main()
{
    println!("Welcome to Mumbai Airport");

    println!("Today, you have to go through four process \n : 1. Check-in 2.Security 3.Immigration 4.Visit the assigned Gate");

    println!("Do you want to check-in? \n Enter 1 for Yes and 0 for No");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("You did not enter a choice");

    let answer : i32 = choice.trim().parse().unwrap();

    if answer == 1
    {
        let checked_in : i32 = checkin();
    }
    else
    {
        println!("Take your time!");
    }


    security();

    gate();

    

    
}