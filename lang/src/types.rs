use inet::protocoll::http::HttpResponse;

pub type RequestIdentifier = (&'static str, &'static str);
pub type RequestHandler = fn() -> HttpResponse;
pub type RequestIdentifierWithParam = (&'static str, &'static str, &'static str);
pub type RequestHandlerWithParam = fn(Vec<&str>) -> HttpResponse;
pub type RequestHandlerWithTwoParam = fn(Vec<&str>, bool) -> HttpResponse;
