use rocket::local::Client;
use rocket::http::Status;
use mockito::mock;


#[test]
fn get_request() {
    let instance = super::app();
    let client = Client::new(instance).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn payload_for_release_no_assets() {
    let instance = super::app();
    let client = Client::new(instance).expect("valid rocket instance");
    let mut request = client.post("/webhook").body(r#"
        {
  "action": "published",
  "release": {
    "id": 11248810,
    "node_id": "MDc6UmVsZWFzZTExMjQ4ODEw",
    "tag_name": "0.0.1",
    "assets": [

    ]
  }
}"#);
    request.add_header(rocket::http::ContentType::JSON);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Not handled".into()));
}


#[test]
fn payload_for_release_2_assets() {
    let instance = super::app();
    let client = Client::new(instance).expect("valid rocket instance");
    let mut request = client.post("/webhook").body(r#"
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
}"#);
    request.add_header(rocket::http::ContentType::JSON);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Not handled".into()));
}

#[test]
fn payload_for_release_single_asset() {
    let instance = super::app();
    let client = Client::new(instance).expect("valid rocket instance");
    let mut request = client.post("/webhook").body(format!(r#"{{
  "action": "published",
  "release": {{
    "id": 11248810,
    "tag_name": "0.0.1",
    "assets": [
        {{"url":"{}"}}
    ]
  }}
}}"#, [mockito::SERVER_URL, "/test"].join("")));
    request.add_header(rocket::http::ContentType::JSON);
    mock("GET","/test").with_body("I am a file").create();

    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("No release script defined".into()));
}