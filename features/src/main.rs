use futures::{executor::block_on,  join};
use std::sync::Arc;
use std::vec::Vec;
use async_std;
use futures::future::join_all;
use std::{thread, time::{self, Instant}};

fn main() {
    let now : Instant = Instant::now();
    let future_one         = do_something(1);
    let two_secs : time::Duration = time::Duration::new(2,0 );
    thread::sleep(two_secs);
    let outcome = block_on(future_one);
    println!("time elapsed {:?}", now.elapsed());
    println!("outcome is {}" ,outcome);

    let future_two = async {
        return do_something(2).await
    };
    let outcome_two = block_on(future_two);
    println!("Here is the outcome: {}", outcome_two);

    let fut_three = async {
        let outcome_wouan = do_something(1).await;
        let outcome_two = do_something(2).await;
        return outcome_wouan + outcome_two
    };

    let fut_outcome = block_on(fut_three);

    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {:?}", fut_outcome);



    let fut_four =  async {
        let out_one = do_something(41);
        let out_two = do_something(42);
        let results = join!(out_one, out_two);

        return results.0 + results.1
    };

    let now = time::Instant::now();

    let result = block_on(fut_four);

    println!("time elapsed {:?}", now.elapsed());

    println!("here is the result: {:?}", result);


    let async_outcome = async {

        let mut futures_vec = Vec::new();
        let fut_four = do_something(44);
        let fut_five =do_something(45);

        futures_vec.push(fut_four);
        futures_vec.push(fut_five);

        let handles = futures_vec.into_iter().map(async_std::task::spawn).collect::<Vec<_>>();

        let results = join_all(handles).await;

        return results.into_iter().sum::<i8>();


    };
    
        let now = time::Instant::now();

let result = block_on(async_outcome);

println!("time elapsed for join vec {:?}", now.elapsed());

println!("Here is the result: {:?}", result);
}

async fn do_something  ( number: i8) -> i8 {
    println!("number {} runs ", number);
    let two_secs : time::Duration = time::Duration::new(2,0 );
    thread::sleep(two_secs);
    2
}
