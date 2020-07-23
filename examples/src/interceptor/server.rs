use tonic::{transport::Server, Request, Response, Status, InterceptorChain, IntoInterceptor};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    let mut ic = InterceptorChain::new();
    ic.add(intercept);
    ic.add(intercept2);
    let svc = GreeterServer::with_interceptor(greeter, ic.into_interceptor());

    println!("GreeterServer listening on {}", addr);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

/// This function will get called on each inbound request, if a `Status`
/// is returned, it will cancel the request and return that status to the
/// client.
fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("1. Intercepting request: {:?}", req);
    let mut req = req;
    req.metadata_mut().insert("app", "tonic".parse().unwrap());
    Ok(req)
}

fn intercept2(req: Request<()>) -> Result<Request<()>, Status> {
    println!("2. Intercepting request: {:?}, app={:?}", req, req.metadata().get("app"));
    Ok(req)
}