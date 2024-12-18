use opentelemetry::{trace::TracerProvider, KeyValue};

use opentelemetry_sdk::Resource;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
pub fn init_otlp(name: &str) {
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );

    #[allow(clippy::expect_used)]
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    name.to_string(),
                )]))
                .with_sampler(opentelemetry_sdk::trace::Sampler::ParentBased(Box::new(
                    sampler(),
                ))),
        )
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("unable to setup tracing pipeline");

    println!("Using Jaeger tracing.");
    println!("{:#?}", provider);

    let formatting_layer = tracing_subscriber::fmt::Layer::default();

    if let Err(e) = tracing_subscriber::Registry::default()
        .with(EnvFilter::from_default_env())
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer(name.to_string())))
        .with(formatting_layer)
        .try_init()
    {
        eprintln!("Error initializing tracing: {:?}", e);
    }
}

fn sampler() -> opentelemetry_sdk::trace::Sampler {
    opentelemetry_sdk::trace::Sampler::TraceIdRatioBased(0.001)
}
