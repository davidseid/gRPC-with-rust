use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayResponse, SayRequest};
mod hello;

// defining a struct for our service
#[derive(Default)]
pub struct MySay {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Say for MySay {
    // specify the output of rpc call
    type BidirectionalStream = mpsc::Receiver<Result<SayResponse,Status>>;
    type SendStreamStream = mpsc::Receiver<Result<SayResponse,Status>>;


    async fn bidirectional(&self, request: Request<tonic::Streaming<SayRequest>>) -> Result<Response<Self::BidirectionalStream>, Status> {
        // converting request into stream
        let mut streamer = request.into_inner();

        // creating queue
        let (mut tx, rx)= mpsc::channel(4);

        tokio::spawn(async move {
            // listening on request stream
            while let Some(req) = streamer.message().await.unwrap() {
                // sending data as soon as it is available
                tx.send(Ok(SayResponse {
                    message: format!("hello {}", req.name),
                }))
                .await;
            }
        });

        // return stream as receiver
        Ok(Response::new(rx))
    }

    // implementation of rpc call
    async fn send_stream(&self, _request: Request<SayRequest>) -> Result<Response<Self::SendStreamStream>, Status> {
        // create a queue or channel
        let (mut tx, rx) = mpsc::channel(4);
        // create a new task
        tokio::spawn(async move {
            // looping and sending our response using stream
            for _ in 0..4 {
                // sending response to our channel
                tx.send(Ok(SayResponse {
                    message: format!("hello"),
                }))
                .await;
            }
        });

        // return our receiver so that tonic can list on receiver and send response to the client
        Ok(Response::new(rx))
    }

    // our rpc implemented as a function
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>,Status>{
        // returning a response as SayResponse message as defined in .proto
        Ok(Response::new(SayResponse{
            // reading data from request which is a wrapper around our SayRequest message defined in .proto
            message:format!("hello {}", request.get_ref().name),
        }))
    }

    // our rpc to receive a stream
    async fn receive_stream(&self, request: Request<tonic::Streaming<SayRequest>>) -> Result<Response<SayResponse>, Status> {
        // converting request into stream
        let mut stream = request.into_inner();
        let mut message = String::from("");

        // listening on stream
        while let Some(req) = stream.message().await? {
            message.push_str(&format!("Hello {}\n", req.name))
        }
        // returning response
        Ok(Response::new(SayResponse { message }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating our service
    let say = MySay::default();
    println!("Server listening on {}", addr);
    // adding our service to our server
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}

