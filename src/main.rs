use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, Server, StatusCode, header};
use hyper::service::{make_service_fn, service_fn};
use serde::{Serialize, Deserialize};
use serde_json;

#[cfg(test)] mod test;

#[derive(Serialize, Deserialize)]
struct OutputData {
	result: bool,
	code:   u32,
	description: String,
}

#[derive(Serialize, Deserialize)]
struct InputData {
	data: String,
}

async fn not_found() -> Result<Response<Body>, Infallible> {
	let mut not_found = Response::default();
	*not_found.status_mut() = StatusCode::NOT_FOUND;
	Ok(not_found)
}

async fn process_requests(req: Request<Body>) -> Result<Response<Body>, Infallible> {
	//Unfortunately hyper does not has a router
	let (parts, body) = req.into_parts();
	let mut v: Vec<&str> = parts.uri.path().split("/").collect();
	v.retain(|&e| e != "");
			
	match parts.method {
		Method::GET => {			
			match v.len() {
				//get /status
				1 if v[0] == "status" => {
					let data = OutputData{
							result: true, 
							code: 0, 
							description: "Everything is OK!".to_string(), 
						};
					let string_data = serde_json::to_string(&data).expect("error serializing");			
					let mut response = Response::new(string_data.into());
					response.headers_mut().insert(header::CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
					Ok(response)
				},
				//get with params
				3 if v[0] == "data" && v[1] == "v1" => {
					let data = OutputData{
							result: true, 
							code: 0, 
							description: format!("You requested get method with name: {}", v[2]).to_string(), 
						};
					let string_data = serde_json::to_string(&data).expect("error serializing");			
					let mut response = Response::new(string_data.into());
					response.headers_mut().insert(header::CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
					Ok(response)
				},
				_ => {
					not_found().await					
				}
			}
		},
		Method::POST => {		
			match v.len() {
				3 if v[0] == "data" && v[1] == "v1" => {
					let body_bytes = hyper::body::to_bytes(body).await.expect("error converting body");
					let body: InputData = serde_json::from_slice(&body_bytes.to_vec()).expect("error deserializing");
					
					let data = OutputData{
							result: true, 
							code: 0, 
							description: format!("You requested post method with name: {}, data is {}", v[2], body.data).to_string(), 
						};
					let string_data = serde_json::to_string(&data).expect("error serializing");	
					let mut response = Response::new(string_data.into());
					response.headers_mut().insert(header::CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
					Ok(response)
				},
				_ => {
					not_found().await					
				}
			}				
		},
		Method::PUT => {		
			match v.len() {
				3 if v[0] == "data" && v[1] == "v1" => {
					let body_bytes = hyper::body::to_bytes(body).await.expect("error converting body");
					let body: InputData = serde_json::from_slice(&body_bytes.to_vec()).expect("error deserializing");
					
					let data = OutputData{
							result: true, 
							code: 0, 
							description: format!("You requested put method with name: {}, data is {}", v[2], body.data).to_string(), 
						};
					let string_data = serde_json::to_string(&data).expect("error serializing");	
					let mut response = Response::new(string_data.into());
					response.headers_mut().insert(header::CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
					Ok(response)
				},
				_ => {
					not_found().await					
				}
			}				
		},		
		Method::DELETE => {		
			match v.len() {
				3 if v[0] == "data" && v[1] == "v1" => {
					let body_bytes = hyper::body::to_bytes(body).await.expect("error converting body");
					let body: InputData = serde_json::from_slice(&body_bytes.to_vec()).expect("error deserializing");
					
					let data = OutputData{
							result: true, 
							code: 0, 
							description: format!("You requested delete method with name: {}, data is {}", v[2], body.data).to_string(), 
						};
					let string_data = serde_json::to_string(&data).expect("error serializing");	
					let mut response = Response::new(string_data.into());
					response.headers_mut().insert(header::CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
					Ok(response)
				},
				_ => {
					not_found().await					
				}
			}				
		},		
		_ => {
			not_found().await
		}
	}
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub async fn run_server(addr: SocketAddr) {
    // A `Service` is needed for every connection, so this
    // creates one from our `process_requests` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(process_requests))
    });

    let server = Server::bind(&addr).serve(make_svc);

	let graceful = server.with_graceful_shutdown( shutdown_signal() );

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
	let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
	run_server(addr).await;
	
	//signal::kill(process::id(), Signal::SIGTERM).unwrap();
}
