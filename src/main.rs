use clap::Parser;
use pingora::prelude::*;
use crate::proxy_service::IdObfuscationProxy;
use crate::config::Config;
mod proxy_service;
mod html_processor;
mod id_mapping;
mod config;

#[derive(Parser)]
struct Opt {
    #[clap(long, default_value = "config.yml")]
    config: String,
}

fn main() {
    env_logger::init();

    let opt = Opt::parse();
    let config = Config::from_yaml(&opt.config).expect("Failed to load config");
    
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let mut my_proxy = pingora_proxy::http_proxy_service(
        &my_server.configuration,
        IdObfuscationProxy::new(config.clone()),
    );

    my_proxy.add_tcp(&config.listen_addr);

    my_server.add_service(my_proxy);
    my_server.run_forever();
}

