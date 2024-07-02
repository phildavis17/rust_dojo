use req::get_check;

mod primes;
mod fibb;
mod req;

#[tokio::main]
async fn main() {
    // primes::primes_main();
    println!("Hello");
    let _ = get_check().await;
    println!("Bye");
    // println!("Hello, world!");
}
