use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::Version;
use hyper::{Body, Error, Method, Request, Response, Server};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio;
use hyper::service::service_fn;

async fn request_handler(
    conn: &'static AddrStream,
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

    let make_service = make_service_fn(|conn: &AddrStream| async {
        let request_handler = |req: Request<Body>| async { request_handler(conn, req).await };
        let service = service_fn(request_handler);
        Ok::<_, Error>(service)
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}