mod models;
mod validators;

use colored::*;
use inquire::Confirm;
use inquire::{min_length, Text};
use models::{Configuration, Http, LoadBalancer, Router, Server, Service, Tls};
use serde_yaml;
use std::{collections::HashMap, fs::write};
use validators::{alphanumeric_validator, domain_validator, validate_address};

fn main() {
    println!(
        "{}",
        "---------------------------------------------".green()
    );
    println!("{}", "Traefik Domain Creator".cyan());
    println!("{}", "Developed by Lucas Ferreira - Jera Soft Co ®".blue());
    println!(
        "{}",
        "---------------------------------------------".green()
    );

    let service_name = Text::new("Name of the service?")
        .with_validator(alphanumeric_validator)
        .with_validator(min_length!(3))
        .with_help_message("e.g.: my-service")
        .prompt()
        .expect("Failed to get service name");

    let host = Text::new("Host name that will proxy to a local service?")
        .with_validator(domain_validator)
        .with_help_message("e.g.: example.com (use ; to separate multiple hosts)")
        .prompt()
        .expect("Failed to get host");
    

    let www = Confirm::new("Do you want to add www to the domain?")
        .with_default(false)
        .prompt()
        .expect("Failed to get www");

    let addresses = Text::new("Local service address?")
        .with_validator(validate_address)
        .with_help_message("e.g.: 192.168.1.10:8080 (use ; to separate multiple addresses)")
        .prompt()
        .expect("Failed to get address");

    let rule = if www {
        host.split(';')
            .map(|h: &str| format!("Host(`{}`) || Host(`www.{}`)", h.trim(), h.trim()))
            .collect::<Vec<String>>()
            .join(" || ")
    } else {
        host.split(';')
            .map(|h| format!("Host(`{}`)", h.trim()))
            .collect::<Vec<String>>()
            .join(" || ")
    };

    let mut routers = HashMap::new();
    routers.insert(
        format!("{}-router", service_name),
        Router {
            rule,
            service: format!("{}-service", service_name),
            entry_points: vec!["websecure".to_string()],
            tls: Tls {
                cert_resolver: "acmeresolver".to_string(),
            },
        },
    );

    let mut servers = Vec::new();
    for address in addresses.split(";") {
        servers.push(Server {
            url: format!("http://{}", address),
        });
    }

    let mut services = HashMap::new();
    services.insert(
        format!("{}-service", service_name),
        Service {
            load_balancer: LoadBalancer { servers },
        },
    );

    let configuration = Configuration {
        http: Http { routers, services },
    };

    let yaml = serde_yaml::to_string(&configuration).unwrap();
    let path = format!("config/services/{}.yml", service_name);
    write(path.clone(), yaml).expect("Failed to write file");

    println!(
        "{}",
        "---------------------------------------------".green()
    );
    println!(
        "Configuration file created at {}",
        format!("{}", path).cyan()
    );
}
