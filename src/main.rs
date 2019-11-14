use async_std::prelude::*;
use async_std::task::{self, JoinHandle};
use std::time::Duration;
use futures::executor::block_on;
use futures::future::{join, join3};
use rand::prelude::*;

fn main() {
    let f = join3(print_letter_a(), print_letter_b(), print_letter_c());
    task::block_on(f);
}


async fn print_letter_a() {
    task::sleep(Duration::from_millis(3000)).await;
    println!("A");
}
async fn print_letter_b() {
    task::sleep(Duration::from_millis(2000)).await;
    println!("B");
}
async fn print_letter_c() {
    task::sleep(Duration::from_millis(1000)).await;
    println!("C");
}
