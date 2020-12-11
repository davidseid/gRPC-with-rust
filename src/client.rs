use futures::stream::iter;
use hello::say_client::SayClient;
use hello::SayRequest;

mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    
    // creating gRPC client from channel
    let mut client = SayClient::new(channel);

    // creating a new Request
    let request = tonic::Request::new(iter(vec![
        SayRequest {
            name:String::from("David")
        },
        SayRequest {
            name:String::from("Emi")
        },
        SayRequest {
            name:String::from("Bern")
        },
    ]));
  
    // // now the response is stream
    // let mut response = client.send_stream(request).await?.into_inner();

    // // listening to the stream
    // while let Some(res) = response.message().await? {
    //     println!("NOTE = {:?}", res);
    // }

    let response = client.receive_stream(request).await?.into_inner();
    println!("RESPONSE=\n{}", response.message);
    Ok(())
}
