use std::marker::PhantomData;

struct HttpResponse<S: ResponseState> {
    // connection would go here
    state: PhantomData<S>,
}

struct Start;
struct Headers;

trait ResponseState {}
impl ResponseState for Start {}
impl ResponseState for Headers {}

impl HttpResponse<Start> {
    pub fn new() -> Self {
        HttpResponse::<Start> { state: PhantomData }
    }

    pub fn status(self, _code: u16, _message: &str) -> HttpResponse<Headers> {
        // send status line ...
        HttpResponse::<Headers> { state: PhantomData }
    }
}

impl HttpResponse<Headers> {
    pub fn header(self, _key: &str, _value: &str) -> Self {
        // send header
        self
    }

    pub fn body(self, _body: &str) {
        // send body
    }
}

fn main() {
    HttpResponse::new()
        .status(200, "Ok")
        .header("Content-Type", "foo/bar")
        .body("Foo Bar");
}
