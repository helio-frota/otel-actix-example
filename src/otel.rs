use opentelemetry::{
    global::{set_text_map_propagator, set_tracer_provider},
    trace::TracerProvider,
};

use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{
    Resource,
    propagation::TraceContextPropagator,
    trace::{Sampler::ParentBased, SdkTracerProvider},
};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_otlp(name: &str) {
    set_text_map_propagator(TraceContextPropagator::new());

    #[allow(clippy::expect_used)]
    let exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Unable to build tracing exporter");

    let resource = Resource::builder().with_service_name(name.to_string()).build();

    let provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .with_sampler(ParentBased(Box::new(sampler())))
        .build();

    println!("Using OTEL Collector with Jaeger as the back-end.");
    println!("{:#?}", provider);

    let formatting_layer = tracing_subscriber::fmt::Layer::default();
    let tracer = provider.tracer(name.to_string());

    if let Err(e) = tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(OpenTelemetryLayer::new(tracer))
        .with(formatting_layer)
        .try_init()
    {
        eprintln!("Error initializing tracing: {:?}", e);
    }
    set_tracer_provider(provider);
}

fn sampler() -> opentelemetry_sdk::trace::Sampler {
    opentelemetry_sdk::trace::Sampler::TraceIdRatioBased(1.001)
}
