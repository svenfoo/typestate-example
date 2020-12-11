use crate::http::Connection;

use std::marker::PhantomData;

pub struct Response<'a, S: ResponseState> {
    conn: &'a mut Connection,
    state: PhantomData<S>,
}

pub struct Start;
pub struct Headers;

pub trait ResponseState {}

impl ResponseState for Start {}
impl ResponseState for Headers {}

impl<'a> Response<'a, Start> {
    pub fn new(conn: &'a mut Connection) -> Self {
        Response::<'a, Start> {
            conn,
            state: PhantomData,
        }
    }

    pub fn status(self, code: u16, reason: &'a str) -> Response<Headers> {
        self.conn
            .send(format!("HTTP/1.1 {} {}\r\n", code, reason).as_ref());
        Response::<'a, Headers> {
            conn: self.conn,
            state: PhantomData,
        }
    }
}

impl Response<'_, Headers> {
    pub fn header(self, key: &str, value: &str) -> Self {
        self.conn.send(format!("{}: {}\r\n", key, value).as_ref());
        self
    }

    pub fn body(self, body: &str) {
        self.conn.send(body);
    }
}
