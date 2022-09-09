use rocust_lib::{test::Test, EndPoint, Method};
use std::time::Duration;

#[tokio::main(flavor = "multi_thread", worker_threads = 1000)]
async fn main() {
    let mut test = Test::new(
        2,
        Some(5),
        5,
        "https://google.com".to_string(),
        vec![
            EndPoint::new(Method::GET, "/".to_string(), None),
            EndPoint::new(Method::GET, "/get".to_string(), None),
            EndPoint::new(Method::POST, "/post".to_string(), None),
            EndPoint::new(Method::PUT, "/put".to_string(), None),
            EndPoint::new(Method::DELETE, "/delete".to_string(), None),
        ],
        None,
    );
    println!("test created {}", test);

    let test_handler = test.clone();
    tokio::spawn(async move {
        println!("canceling test in 200 seconds");
        tokio::time::sleep(Duration::from_secs(8)).await;
        println!("attempting cancel");
        test_handler.stop();
    });

    let test_handler = test.clone();
    tokio::spawn(async move {
        println!("canceling user 1 in 2 seconds");
        tokio::time::sleep(Duration::from_secs(8)).await;
        println!("attempting cancel user 1");
        test_handler.stop_a_user(1).unwrap_or_default();
    });

    let test_handler = test.clone();
    tokio::spawn(async move {
        println!("canceling user 15 in 7 seconds");
        tokio::time::sleep(Duration::from_secs(7)).await;
        println!("attempting cancel user 15");
        test_handler.stop_a_user(15).unwrap_or_default();
    });

    let test_handler = test.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;
            println!("STATUS: [{}]", test_handler.get_status().read());
        }
    });

    test.run().await;
    println!("\n{}", test);
    println!();
    let endpoints = test.get_endpoints();
    for endpoint in endpoints.iter() {
        println!("{}", endpoint);
        println!("------------------------------");
    }
    println!();
    let users = test.get_users();
    for user in users.read().iter() {
        println!("{}\n", user);
        for (endpoint_url, results) in user.get_endpoints().read().iter() {
            println!("\t[{}] | [{}]\n", endpoint_url, results);
        }
        println!("------------------------------");
    }
}
