use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use tonic::metadata::{MetadataKey, MetadataMap};
use url::Url;

use opentelemetry::sdk::trace::IdGenerator;
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::trace::FutureExt;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use std::env::{remove_var, var};
use std::{env::vars, str::FromStr};
use tracing_opentelemetry::OpenTelemetryLayer;

const ENDPOINT: &str = "OLTP_TONIC_ENDPOINT";
const HEADER_PREFIX: &str = "OLTP_TONIC_";
// $Env:OLTP_TONIC_ENDPOINT = "https://api.honeycomb.io:443" = old endpoint

// fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
//     let endpoint = var(ENDPOINT).unwrap_or_else(|_| {
//         panic!(
//             "You must specify and endpoint to connect to with the variable {:?}.",
//             ENDPOINT
//         )
//     });
//     let endpoint = Url::parse(&endpoint).expect("endpoint is not a valid url");
//     remove_var(ENDPOINT);
//     let mut metadata = MetadataMap::new();
//     for (key, value) in vars()
//         .filter(|(name, _)| name.starts_with(HEADER_PREFIX))
//         .map(|(name, value)| {
//             let header_name = name
//                 .strip_prefix(HEADER_PREFIX)
//                 .map(|h| h.replace("_", "-"))
//                 .map(|h| h.to_ascii_lowercase())
//                 .unwrap();
//             (header_name, value)
//         })
//     {
//         metadata.insert(MetadataKey::from_str(&key).unwrap(), value.parse().unwrap());
//     }
//
//     opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_exporter(
//             opentelemetry_otlp::new_exporter()
//                 .tonic()
//                 .with_endpoint(endpoint.as_str())
//                 .with_metadata(dbg!(metadata))
//                 .with_tls_config(
//                     ClientTlsConfig::new().domain_name(
//                         endpoint
//                             .host_str()
//                             .expect("the specified endpoint should have a valid host"),
//                     ),
//                 ),
//         )
//         .install_batch(opentelemetry::runtime::Tokio)
// }
pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: impl MakeWriter + Send + Sync + 'static,
) -> impl Subscriber + Send + Sync {
    get_subscriber_jaegger(name, env_filter, sink)
    //get_subscriber_honeycomb(name, env_filter, sink)
}

/*
pub fn get_subscriber_honeycomb(
    name: String,
    env_filter: String,
    sink: impl MakeWriter + Send + Sync + 'static,
) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    //Open Telemetry stuff goes here
    let endpoint = var(ENDPOINT).unwrap_or_else(|_| {
        panic!(
            "You must specify and endpoint to connect to with the variable {:?}.",
            ENDPOINT
        )
    });
    let _x = dbg!(&endpoint);
    let endpoint = Url::parse(&endpoint).expect("endpoint is not a valid url");
    let _x = dbg!(&endpoint);
    remove_var(ENDPOINT);
    let mut metadata = MetadataMap::new();
    for (key, value) in vars()
        .filter(|(name, _)| name.starts_with(HEADER_PREFIX))
        .map(|(name, value)| {
            let header_name = name
                .strip_prefix(HEADER_PREFIX)
                .map(|h| h.replace("_", "-"))
                .map(|h| h.to_ascii_lowercase())
                .unwrap();
            (header_name, value)
        })
    {
        metadata.insert(MetadataKey::from_str(&key).unwrap(), value.parse().unwrap());
    }

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            trace::config()
                .with_id_generator(IdGenerator::default())
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "zero2prod",
                )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(dbg!(endpoint.as_str()))
                .with_metadata(dbg!(metadata))
                .with_tls_config(
                    ClientTlsConfig::new().domain_name(
                        endpoint
                            .host_str()
                            .expect("the specified endpoint should have a valid host"),
                    ),
                ),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("install of otlp failed");

    // old - let open_telemetry_layer = OpenTelemetryLayer::new(tracer);
    let opentelemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    // Done with Open Telemetry stuff now

    //set up the tracer
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(opentelemetry_layer)
}
*/
pub fn get_subscriber_jaegger(
    name: String,
    env_filter: String,
    _sink: impl MakeWriter + Send + Sync + 'static,
) -> impl Subscriber + Send + Sync {
    let _x = dbg!(&env_filter);
    let _y = dbg!(&name);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            trace::config()
                .with_id_generator(IdGenerator::default())
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "zero2prod",
                )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:9000"),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("failed to initialize otel tracing pipeline");
    let opentelemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(opentelemetry_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
