use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Router {
    pub rule: String,
    pub service: String,
    #[serde(rename = "entryPoints")]
    pub entry_points: Vec<String>,
    pub tls: Tls,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tls {
    #[serde(rename = "certResolver")]
    pub cert_resolver: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    #[serde(rename = "loadBalancer")]
    pub load_balancer: LoadBalancer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadBalancer {
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Http {
    pub routers: HashMap<String, Router>,
    pub services: HashMap<String, Service>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub http: Http,    
}