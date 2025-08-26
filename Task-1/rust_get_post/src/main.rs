//request is create for making http request and
//Client is like a browser inside your program that can send requests to the internet.
// refrence:- https://docs.rs/reqwest/latest/reqwest/
use reqwest::Client;


//tokio a runtime manager to allow asynchronous programming
#[tokio::main]


async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //Creates a new HTTP client (like opening a browser session).
    let client = Client::new();

    // ---------- GET Request ----------
    let get_url = "https://httpbin.org/get"; // free test server
    let get_response = client.get(get_url).send().await?;
    let get_text = get_response.text().await?;
    println!("GET Response:\n{}", get_text);

    // ---------- POST Request ----------
    let post_url = "https://httpbin.org/post";
    let post_body = serde_json::json!({
        "message": "Hello World from Rust!"
    });

    let post_response = client.post(post_url)
        .json(&post_body)
        .send()
        .await?;

    let post_text = post_response.text().await?;
    println!("POST Response:\n{}", post_text);

    Ok(())
}
