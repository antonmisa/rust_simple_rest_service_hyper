use std::str;
use hyper::{Client, StatusCode, Uri, Body, Request};

#[tokio::test]
async fn test_status() {	
	let url: Uri = "http://127.0.0.1:8000/status".parse().unwrap();
	let client = Client::new();
    let res = client.get(url).await.expect("error retrieving");
    assert_eq!(res.status(), StatusCode::OK);
	
	let body_bytes = hyper::body::to_bytes(res).await.expect("error converting body");
	let body_string = str::from_utf8(&body_bytes).unwrap();
	assert_eq!(body_string, r##"{"result":true,"code":0,"description":"Everything is OK!"}"##);
}

#[tokio::test]
async fn test_get() {	
	let client = Client::new();
	let req = Request::builder()
		.method("GET")
		.uri("http://127.0.0.1:8000/data/v1/test")
		.body(Body::from(""))
		.expect("error request builder");
		
	let res = client.request(req).await.expect("error");	
    assert_eq!(res.status(), StatusCode::OK);
	
	let body_bytes = hyper::body::to_bytes(res).await.expect("error converting body");
	let body_string = str::from_utf8(&body_bytes).unwrap();
	assert_eq!(body_string, r##"{"result":true,"code":0,"description":"You requested get method with name: test"}"##);
}

#[tokio::test]
async fn test_post() {	
	let client = Client::new();
	let req = Request::builder()
		.method("POST")
		.uri("http://127.0.0.1:8000/data/v1/test")
		.body(Body::from(r#"{"data":"test data"}"#))
		.expect("error request builder");
		
	let res = client.request(req).await.expect("error");	
    assert_eq!(res.status(), StatusCode::OK);
	
	let body_bytes = hyper::body::to_bytes(res).await.expect("error converting body");
	let body_string = str::from_utf8(&body_bytes).unwrap();
	assert_eq!(body_string, r##"{"result":true,"code":0,"description":"You requested post method with name: test, data is test data"}"##);
}

#[tokio::test]
async fn test_put() {	
	let client = Client::new();
	let req = Request::builder()
		.method("PUT")
		.uri("http://127.0.0.1:8000/data/v1/test")
		.body(Body::from(r#"{"data":"test data"}"#))
		.expect("error request builder");
		
	let res = client.request(req).await.expect("error");	
    assert_eq!(res.status(), StatusCode::OK);
	
	let body_bytes = hyper::body::to_bytes(res).await.expect("error converting body");
	let body_string = str::from_utf8(&body_bytes).unwrap();
	assert_eq!(body_string, r##"{"result":true,"code":0,"description":"You requested put method with name: test, data is test data"}"##);
}

#[tokio::test]
async fn test_delete() {	
	let client = Client::new();
	let req = Request::builder()
		.method("DELETE")
		.uri("http://127.0.0.1:8000/data/v1/test")
		.body(Body::from(r#"{"data":"test data"}"#))
		.expect("error request builder");
		
	let res = client.request(req).await.expect("error");	
    assert_eq!(res.status(), StatusCode::OK);
	
	let body_bytes = hyper::body::to_bytes(res).await.expect("error converting body");
	let body_string = str::from_utf8(&body_bytes).unwrap();
	assert_eq!(body_string, r##"{"result":true,"code":0,"description":"You requested delete method with name: test, data is test data"}"##);
}