use context::Context;
use std::io::Write;
use std::io;
use mount::Mount;
use router::Router;
use iron::prelude::*;
use iron::{Handler, Url, status};
use iron::modifier::Modifier;
use iron::modifiers::Redirect;
use iron::response::WriteBody;
use mount::OriginalUrl;
use mime_guess;
use std::sync::Arc;
use std::borrow::Cow;
use url;

include!(concat!(env!("OUT_DIR"), "/data_www.rs"));

macro_rules! declare_endpoint {
	($router:expr, $context:expr, $route:expr, $method:ident, $name:expr, $lambda:expr) => {
		{
			let context = $context.clone();
			$router.$method($route, move |req: &mut Request| -> IronResult<Response> {
				$lambda(req, &context)
			}, $name);
		}
	}
}
macro_rules! declare_multiple_endpoints {
	($pool:expr, $mainRouter:expr, $mount_route:expr, $( $name:expr => $method:ident, $route:expr, $handler:expr ),*) => {
		{
			let mut sub_router = Router::new();
			$(
				declare_endpoint!(sub_router, $pool, $route, $method, $name, $handler);
			)*
			$mainRouter.mount($mount_route, sub_router);
		}
	}
}

pub fn exec(_: &mut Request, ctx: &Context) -> IronResult<Response> {
    Ok(Response::with((status::Ok)))
}

pub fn declare_endpoints(ctx: Arc<Context>) -> Mount {
    let mut mount = Mount::new();
    mount.mount("/", Static::new("data/www/"));
    declare_multiple_endpoints!(
        ctx, mount, "/api/",
        "sqlCheck" => post, "/sql/check/:task",  exec
    );
    mount
}

#[derive(Clone)]
pub struct Static {
    /// Path prefix.
    pub prefix: String,
}

pub struct StaticFile {
    pub mime: mime_guess::Mime,
    pub data: Cow<'static, [u8]>,
}

impl Static {
    pub fn new<P: Into<String>>(root: P) -> Static {
        Static { prefix: root.into() }
    }
}

impl Handler for Static {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut path = self.prefix.clone() + &req.url.path().join("/");
        if path.ends_with("/") {
            path.push_str("index.html");
        }

        let slash = path.rfind('/').unwrap_or(0);
        let dot = path.rfind('.').map(|i| i + 1).unwrap_or(0);
        let mime = if slash < dot {
            mime_guess::get_mime_type(&path[dot..])
        } else {
            mime_guess::octet_stream()
        };

        WWW.get(&path)
            .map(|data| {
                Response::with((
                    status::Ok,
                    StaticFile {
                        mime: mime,
                        data: data,
                    },
                ))
            })
            .or_else(|e| match WWW.get(&(path + "/" + "index.html")) {
                Ok(_) => {
                    // Perform an HTTP 301 Redirect.
                    let mut original_url: url::Url = match req.extensions.get::<OriginalUrl>() {
                        None => &req.url,
                        Some(original_url) => original_url,
                    }.clone()
                        .into();

                    // Append the trailing slash
                    //
                    // rust-url automatically turns an empty string in the last
                    // slot in the path into a trailing slash.
                    original_url.path_segments_mut().unwrap().push("");
                    let redirect_path = Url::from_generic_url(original_url).unwrap();
                    Ok(Response::with((
                        status::MovedPermanently,
                        format!("Redirecting to {}", redirect_path),
                        Redirect(redirect_path),
                    )))
                }
                Err(_) => Err(IronError::new(e, status::NotFound)),
            })
    }
}

impl WriteBody for StaticFile {
    fn write_body(&mut self, res: &mut Write) -> io::Result<()> {
        res.write(self.data.as_ref()).map(|_| ())
    }
}

impl Modifier<Response> for StaticFile {
    fn modify(self, response: &mut Response) {
        response.headers.append_raw(
            "Content-Type",
            Vec::<u8>::from(self.mime.as_ref()),
        );
        response.body = Some(Box::new(self));
    }
}
