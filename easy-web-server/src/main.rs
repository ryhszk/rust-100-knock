extern crate iron;

#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

fn main() {
    println!("Serving on http://localhost:300...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

// 2020/08/24 IronResut in error ... expected enum `std::result::Result`, found `()`
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    // r#""# is aa is the raw string syntax for rust. 
    // This is useful when working with long strings without escaping.
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text name="n" />
            <input type="text name="n" />
            <button type="submit">Compute GCD</button>
        </form>
    "#);

    Ok(response);
}