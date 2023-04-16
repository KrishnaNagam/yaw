use curl::easy::Easy;

#[test]
fn ok() {
    let mut curl = Easy::new();
    curl.url("http://localhost:8080/").unwrap();
    curl.perform().unwrap();
    assert_eq!(curl.response_code(),Ok(200))
}

#[test]
fn unauthorized() {
    let mut curl = Easy::new();
    curl.url("http://localhost:8080/admin").unwrap();
    curl.perform().unwrap();
    assert_eq!(curl.response_code(),Ok(401))
}

#[test]
fn authorized() {
    let mut curl = Easy::new();
    curl.url("http://localhost:8080/admin").unwrap();
    curl.username("user").unwrap();
    curl.password("password").unwrap();
    curl.perform().unwrap();
    assert_eq!(curl.response_code(),Ok(200))
}

#[test]
fn method_not_implemented() {
    let mut curl = Easy::new();
    curl.url("http://localhost:8080/hello.html").unwrap();
    curl.post(true).unwrap();
    curl.perform().unwrap();
    assert_eq!(curl.response_code(),Ok(501))
}

#[test]
fn not_found() {
    let mut curl = Easy::new();
    curl.url("http://localhost:8080/non-existing-file.html").unwrap();
    curl.perform().unwrap();
    assert_eq!(curl.response_code(),Ok(404))
}