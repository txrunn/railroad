use axum::{http::Uri, Router};
use serde::Deserialize;
use std::net::SocketAddr;

/// A configuration object for the reverse proxy.
#[derive(Deserialize)]
struct ProxyConfig {
    /// A list of microservices to proxy.
    proxies: Vec<MicroserviceConfig>,
}

/// A configuration object for a microservice.
#[derive(Deserialize)]
struct MicroserviceConfig {
    /// The name of the microservice.
    name: String,
    /// The base URL of the microservice.
    base_url: String,
    /// A list of routes to proxy.
    routes: Vec<RouteConfig>,
}

/// A configuration object for a route.
#[derive(Deserialize)]
struct RouteConfig {
    /// The path to match for the route.
    path: String,
    /// The target URL to forward traffic to.
    target: String,
}

#[tokio::main]
async fn main() {
    // Load the YAML configuration file
    let config_str = std::fs::read_to_string("config.yaml").unwrap();
    let config: ProxyConfig = serde_yaml::from_str(&config_str).unwrap(); // Deserialize the configuration file using serde

    // Create a new router
    let mut router = Router::new();

    // Add routes for each microservice in the configuration
    for microservice in config.proxies {
        // Loop through each microservice in the configuration
        for route in microservice.routes {
            // Loop through each route for each microservice
            let target = Uri::try_from(microservice.base_url.to_owned() + &route.target).unwrap();
            // Construct the target Uri for the route using the microservice base url and the route target
            // TODO: Figure out/define logic
            // Here we would define the logic for handling the incoming requests and forwarding them to the appropriate microservice based on the route configuration.
            // This could involve using axum's `proxy` handler to forward requests to the target URI, or implementing custom logic to handle the requests and responses.
            // The exact implementation will depend on the requirements of your application and the desired behavior of the reverse proxy.
        }
    }

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080)); // Bind to the IPv4 address 0.0.0.0 on port 8080
    axum::Server::bind(&addr)
        .serve(router.into_make_service()) // Start serving the requests using the router
        .await
        .unwrap();
}
