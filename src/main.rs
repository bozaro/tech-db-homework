extern crate dotenv;
#[macro_use]
extern crate mime_guess;
extern crate iron;
extern crate hyper;
extern crate router;
extern crate mount;
extern crate url;
extern crate phf;
extern crate includedir;

mod context;
mod endpoints;

use dotenv::dotenv;
use context::Context;
use endpoints::declare_endpoints;
use hyper::net::{HttpStream, NetworkListener};

use iron::Protocol;
use iron::prelude::Iron;
use std::env;
use std::io;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::sync::Arc;

#[derive(Clone)]
struct TcpListenerNoDelay {
    listener: Arc<TcpListener>,
}

impl NetworkListener for TcpListenerNoDelay {
    type Stream = HttpStream;

    fn accept(&mut self) -> Result<Self::Stream, hyper::Error> {
        let tcp = try!(self.listener.accept());
        try!(tcp.0.set_nodelay(true));
        let stream = HttpStream(tcp.0);
        Ok(stream)
    }

    fn local_addr(&mut self) -> io::Result<SocketAddr> {
        self.listener.local_addr()
    }
}

fn main() {
    dotenv().ok();

    let router = declare_endpoints(Arc::new(Context {}));

    // Start server
    let addr = env::var("LISTEN").unwrap_or("localhost:5000".to_string());
    println!("http://{}/", addr);

    let listener = TcpListener::bind(addr).unwrap();
    Iron::new(router)
        .listen(
            TcpListenerNoDelay { listener: Arc::new(listener) },
            Protocol::http(),
        )
        .unwrap();
}
