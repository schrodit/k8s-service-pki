use clap::{Arg, App};
use actix_web::{HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::fs::File;


pub struct Options {
    port: String,
    config: Configuration,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    namespaces: Vec<String>,
}

impl Options {

    pub fn new() -> Options {
        let app = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::new("CONFIG")
                    .short('c')
                    .long("config")
                    .takes_value(true)
                    .about("Path to the configuration file"))
        .arg(Arg::new("PORT")
                    .short('p')
                    .long("port")
                    .takes_value(true)
                    .about("HTTP server port"))
        .get_matches();

        let config_path = app.value_of("CONFIG").expect("config required");
        let port_val = app.value_of("PORT").unwrap_or("8080");

        // read config file from given path
        let file = File::open(config_path).expect("unable to to open file");
        let config: Configuration = serde_json::from_reader(file).expect("unable to parse config");
        
        return Options{
            port: String::from(port_val),
            config: config,
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("Running server on port {}", self.port);

        println!("Config: {}", serde_yaml::to_string(&self.config).expect("unable to encode config as yaml"));

        HttpServer::new(|| actix_web::App::new()
            .service(crate::webhooks::mutating::index)
            .service(web::resource("/version").route(web::get().to(crate::routes::version)))
        )
        .bind(format!("127.0.0.1:{}", self.port))?
        .run()
        .await
    }

}
