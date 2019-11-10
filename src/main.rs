extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::{
    prelude::{Iron, IronResult, Plugin, Request, Response, Set},
    status,
};
use router::Router;
use std::{mem::swap, num::ParseIntError, str::FromStr};
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#,
    );

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let unparsed_numbers = match request.get_ref::<UrlEncodedBody>() {
        Ok(form_data) => form_data.get("n"),
        Err(e) => {
            return send_bad_request(response, &format!("Error parsing form data: {:?}\n", e));
        }
    };

    if unparsed_numbers.is_none() {
        return send_bad_request(response, "form data has no 'n' parameter\n");
    }

    let numbers = match parse_numbers(unparsed_numbers.unwrap()) {
        Ok(numbers) => numbers,
        Err(e) => {
            return send_bad_request(response, &format!("{:?}", e));
        }
    };

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!(
        "The greatest common divisor of the number {:?} is <b>{}</b>\n",
        numbers, get_gcd(&numbers)
    ));

    Ok(response)
}

fn send_bad_request(mut response: Response, message: &str) -> IronResult<Response> {
    response.set_mut(status::BadRequest);
    response.set_mut(message);
    Ok(response)
}

fn parse_numbers(strings: &[String]) -> Result<Vec<u64>, ParseIntError> {
    strings
        .iter()
        .try_fold(Vec::new(), |mut numbers, unparsed| {
            numbers.push(u64::from_str(unparsed)?);
            Ok(numbers)
        })
}

fn get_gcd(numbers: &[u64]) -> u64 {
    numbers.iter().skip(1).fold(numbers[0], |d, num| {
        let factor = gcd(d, *num);

        if factor > 0 {
            factor
        } else {
            d
        }
    })
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0, m != 0);

    while m != 0 {
        if m < n {
            swap(&mut m, &mut n);
        }

        m %= n;
    }

    n
}
