use reqwest::Client;
use tokio::net::TcpListener;
use zero2prod::run;


#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client.get("http://localhost:8080/health_check").send().await.expect("Failed to execute");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}


fn spawn_app()  {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let server = zero2prod::run("127.0.0.1:0").expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}
