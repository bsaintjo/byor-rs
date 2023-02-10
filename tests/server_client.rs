#[test]
fn test_server_client() {
    std::thread::spawn(|| server::run().expect("Failed to start server"));
    let handle = std::thread::spawn(|| {
        for n in 0..3 {
            println!("n = {n}");
            client::run().expect("Client failed");
        }
    });
    handle.join().unwrap();
}
