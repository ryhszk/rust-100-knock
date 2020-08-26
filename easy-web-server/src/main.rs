extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

extern crate router;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.get("/gcd", post_gcd, "gcd");
    
    println!("Serving on http://localhost:300...");
    Iron::new(router).http("localhost:3000").unwrap();
}

extern crate urlencoded;

use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' parameter not number: {:?}\n",
                            unparsed));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n); }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of the numgers {:?} is <b>{}</b>\n",
                numbers, d));
    Ok(response)
}

/* 
 * 2020/08/24 ... IronResut in error ... expected enum `std::result::Result`, found `()`
 * The error has been resolved. The reason is that there was semicolon(;) of the OK(response)... 
 * So, it took 3 hours to resolve ( ；∀；)
 */
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

    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}