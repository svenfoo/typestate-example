use std::marker::PhantomData;

pub struct Response<S: ResponseState> {
    // connection would go here
    state: PhantomData<S>,
}

pub struct Start;
pub struct Headers;

pub trait ResponseState {}

impl ResponseState for Start {}
impl ResponseState for Headers {}

impl Response<Start> {
    pub fn new() -> Self {
        Response::<Start> { state: PhantomData }
    }

    pub fn status(self, _code: u16, _message: &str) -> Response<Headers> {
        // send status line ...
        Response::<Headers> { state: PhantomData }
    }
}

impl Response<Headers> {
    pub fn header(self, _key: &str, _value: &str) -> Self {
        // send header
        self
    }

    pub fn body(self, _body: &str) {
        // send body
    }
}
