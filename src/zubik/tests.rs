use mockito::mock;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn get_request() {
    let instance = super::app(super::Config::new("".to_string(), "".to_string()));
    let client = Client::new(instance).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn payload_for_release_no_assets() {
    let instance = super::app(super::Config::new("".to_string(), "".to_string()));
    let client = Client::new(instance).expect("valid rocket instance");
    let mut request = client.post("/webhook").body(
        r#"
        {
  "action": "published",
  "release": {
    "id": 11248810,
    "node_id": "MDc6UmVsZWFzZTExMjQ4ODEw",
    "tag_name": "0.0.1",
    "assets": [

    ]
  }
}"#,
    );
    request.add_header(rocket::http::ContentType::JSON);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Not handled".into()));
}

#[test]
fn payload_for_release_2_assets() {
    let instance = super::app(super::Config::new("".to_string(), "".to_string()));
    let client = Client::new(instance).expect("valid rocket instance");
    let mut request = client.post("/webhook").body(
        r#"
        {
  "action": "published",
  "release": {
    "id": 11248810,
    "tag_name": "0.0.1",
    "assets": [
        {"url":"https://test.dk/1"},
        {"url":"https://test.dk/2"}
    ]
  }
}"#,
    );
    request.add_header(rocket::http::ContentType::JSON);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Not handled".into()));
}

#[test]
fn payload_for_release_single_asset() {
    let instance = super::app(super::Config::new("".to_string(), "".to_string()));
    let client = Client::new(instance).expect("valid rocket instance");
    let mut request = client.post("/webhook").body(format!(
        r#"{{
  "action": "published",
  "release": {{
    "id": 11248810,
    "tag_name": "0.0.1",
    "assets": [
        {{"url":"{}"}}
    ]
  }}
}}"#,
        [mockito::SERVER_URL, "/test"].join("")
    ));
    request.add_header(rocket::http::ContentType::JSON);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("No release script defined".into())
    );
}

#[test]
fn payload_for_release_single_asset_with_script() {
    let instance = super::app(super::Config::new("".to_string(), "head".to_string()));
    let client = Client::new(instance).expect("valid rocket instance");
    let mut request = client.post("/webhook").body(format!(
        r#"{{
  "action": "published",
  "release": {{
    "id": 11248810,
    "tag_name": "0.0.1",
    "assets": [
        {{"url":"{}"}}
    ]
  }}
}}"#,
        [mockito::SERVER_URL, "/test"].join("")
    ));
    request.add_header(rocket::http::ContentType::JSON);
    let _m = mock("GET", "/test")
        .match_header("Accept", "application/octet-stream")
        .with_body("I am a file")
        .create()
        .expect(1);

    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("I am a file".into()));
}

#[test]
fn payload_for_release_single_asset_with_script_and_token() {
    let instance = super::app(super::Config::new(
        "secret_token".to_string(),
        "head".to_string(),
    ));
    let client = Client::new(instance).expect("valid rocket instance");
    let mut request = client.post("/webhook").body(format!(
        r#"{{
  "action": "published",
  "release": {{
    "id": 11248810,
    "tag_name": "0.0.1",
    "assets": [
        {{"url":"{}"}}
    ]
  }}
}}"#,
        [mockito::SERVER_URL, "/test"].join("")
    ));
    request.add_header(rocket::http::ContentType::JSON);
    let _m = mock("GET", "/test?access_token=secret_token")
        .match_header("Accept", "application/octet-stream")
        .with_body("I am a file")
        .create()
        .expect(1);

    let mut response = request.dispatch();
    _m.assert();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("I am a file".into()));
}
