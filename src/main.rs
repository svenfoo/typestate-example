mod http;

fn main() {
    let mut connection = http::Connection::new();

    http::Response::new(&mut connection)
        .status(200, "Ok")
        .header("Content-Type", "foo/bar")
        .body("Foo Bar");

    println!("\n{} bytes sent", connection.sent());
}
