use std::time::Duration;

#[test]
fn test_server_client() {
    std::thread::spawn(|| {
        server::run().expect("Failed to start server")
    });
    let handle = std::thread::spawn(|| {
        for _ in 0 .. 3 {
            client::run().expect("Client failed");
        }
    });
    handle.join().unwrap();
    std::thread::sleep(Duration::from_millis(5000));
}