use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::Version;
use hyper::{Body, Error, Method, Request, Response, Server};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio;
use hyper::service::service_fn;

async fn request_handler(
    socket_addr: SocketAddr,
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    println!("req: {:?}", req);
    if req.method() == Method::CONNECT {
        println!("Connect")
    }
    let res: Response<Body> = Response::builder()
        .status(200)
        .version(Version::HTTP_11)
        .body(Body::empty())
        .unwrap();
    return Ok(res);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let addr = SocketAddr::new(ip, 1337);
    //let client = Client::new();

    let make_service = make_service_fn(|conn: &AddrStream|  {
        // The solution is to clone what he needs to request_handler
        // For example, if he wants to use AddrStream.remote_addr field
        let socket_addr = conn.remote_addr().clone();
        async move {
            let request_handler = move |req: Request<Body>| async move {
                request_handler(socket_addr, req).await 
            };
            let service = service_fn(request_handler);
            Ok::<_, Error>(service)
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}