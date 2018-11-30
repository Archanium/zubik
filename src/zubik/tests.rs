use rocket::local::Client;
use rocket::http::Status;

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
    "url": "https://api.github.com/repos/Codertocat/Hello-World/releases/11248810",
    "assets_url": "https://api.github.com/repos/Codertocat/Hello-World/releases/11248810/assets",
    "upload_url": "https://uploads.github.com/repos/Codertocat/Hello-World/releases/11248810/assets{?name,label}",
    "html_url": "https://github.com/Codertocat/Hello-World/releases/tag/0.0.1",
    "id": 11248810,
    "node_id": "MDc6UmVsZWFzZTExMjQ4ODEw",
    "tag_name": "0.0.1",
    "target_commitish": "master",
    "name": null,
    "draft": false,
    "prerelease": false,
    "created_at": "2018-05-30T20:18:05Z",
    "published_at": "2018-05-30T20:18:44Z",
    "assets": [

    ],
    "tarball_url": "https://api.github.com/repos/Codertocat/Hello-World/tarball/0.0.1",
    "zipball_url": "https://api.github.com/repos/Codertocat/Hello-World/zipball/0.0.1",
    "body": null
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
    "url": "https://api.github.com/repos/Codertocat/Hello-World/releases/11248810",
    "assets_url": "https://api.github.com/repos/Codertocat/Hello-World/releases/11248810/assets",
    "upload_url": "https://uploads.github.com/repos/Codertocat/Hello-World/releases/11248810/assets{?name,label}",
    "html_url": "https://github.com/Codertocat/Hello-World/releases/tag/0.0.1",
    "id": 11248810,
    "node_id": "MDc6UmVsZWFzZTExMjQ4ODEw",
    "tag_name": "0.0.1",
    "target_commitish": "master",
    "name": null,
    "draft": false,
    "prerelease": false,
    "created_at": "2018-05-30T20:18:05Z",
    "published_at": "2018-05-30T20:18:44Z",
    "assets": [
        {"url":"https://test.dk/1"},
        {"url":"https://test.dk/2"}
    ],
    "tarball_url": "https://api.github.com/repos/Codertocat/Hello-World/tarball/0.0.1",
    "zipball_url": "https://api.github.com/repos/Codertocat/Hello-World/zipball/0.0.1",
    "body": null
  }
}"#);
    request.add_header(rocket::http::ContentType::JSON);
    let mut response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Not handled".into()));
}