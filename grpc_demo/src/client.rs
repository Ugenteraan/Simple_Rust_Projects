use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tokio_stream::StreamExt;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Alice".into(),
    });

    let response = client.say_hello(request).await?;
    println!("Unary Response: {}", response.into_inner().message);

    let request = tonic::Request::new(HelloRequest {
        name: "Bob".to_string(),
    });

    let mut stream = client.say_hello_stream(request).await?.into_inner();

    while let Some(response) = stream.next().await {
        match response {
            Ok(reply) => {
                println!("Stream response: {}", reply.message);
            }

            Err(e) => eprintln!("Stream Error: {}", e),
        }
    }

    Ok(())
}
