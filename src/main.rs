use std::thread;
fn add(){
let a = 12;
let b = 21;
let c = a+b;
println!("{}",c);
}
fn main(){
    add();
    //Spawn three threads
    for _ in 0..3 {
        thread::spawn(||{
            let mut counter = 0.64;
            loop {
                counter += 0.001;
            }
        });
    }
    loop {
        //Main thread does nothing just keep the project alive just like the while loop running infintely in the nodejs process.
    }

}
