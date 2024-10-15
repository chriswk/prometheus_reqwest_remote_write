# Prometheus Reqwest Remote Write

This repository contains a Rust library for sending metrics to Prometheus over the remote write API. It uses Reqwest APIs to build a snappy compressed payload and sends it to the Prometheus server.
Building directly from the prometheus protobuff format, if you have a prometheus registry, Since a reqwest client can have default headers baked in,
the library does not provide a way to configure extra headers. It does set the correct headers according to remote write 1.0 spec though, so if you need basic authentication, bake it into the client you pass to the build_http_request call


## Basic Usage
```rust

use prometheus_reqwest_remote_write::WriteRequest;

pub async fn from_prometheus_registry_and_request_client(client: reqwest::Client, remote_write_url: &str, registry: prometheus::Registry) {
    let write_request = WriteRequest::from_metric_families(registry.gather()).expect("Could not format write request");
    let http_request = write_request.build_http_request(client.clone(), remote_write_url, "your_user_agent").expect("Could not build http request");
    match client.execute(http_request).await {
        Ok(r) =>  {
            if r.status().is_success() {
                println!("Metrics sent successfully");
            } else {
                println!("Failed to send metrics: {:?}", r.text().await.expect("Could not read body from response");
            }
        },
        Err(e) => {
            println!("Failed to send metrics: {:?}", e);
        }
    }
```
