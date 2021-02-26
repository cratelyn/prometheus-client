#![forbid(unsafe_code)]
#![deny(unused)]
#![deny(dead_code)]

//! Client library implementation of the [Open Metrics
//! specification](https://github.com/OpenObservability/OpenMetrics). Allows
//! developers to instrument applications and thus enables operators to monitor
//! said applications with monitoring systems like
//! [Prometheus](https://prometheus.io/).
//!
//! # Examples
//!
//! ```
//! # use open_metrics_client::encoding::text::Encode;
//! # use open_metrics_client::encoding::text::encode;
//! # use open_metrics_client::metrics::counter::{Atomic, Counter};
//! # use open_metrics_client::metrics::family::Family;
//! # use open_metrics_client::registry::Registry;
//! # use std::io::Write;
//! # use std::sync::atomic::AtomicU64;
//! #
//! // Create a metric registry.
//! //
//! // Note the angle brackets to make sure to use the default (dynamic
//! // dispatched boxed metric) for the generic type parameter.
//! let mut registry = <Registry>::default();
//!
//! // Define a type representing a metric label set, i.e. a key value pair.
//! //
//! // You could as well use `(String, String)` to represent a label set,
//! // instead of the custom type below.
//! #[derive(Clone, Hash, PartialEq, Eq, Encode)]
//! struct Labels {
//!   // Use your own enum types to represent label values.
//!   method: Method,
//!   // Or just a plain string.
//!   path: String,
//! };
//!
//! #[derive(Clone, Hash, PartialEq, Eq, Encode)]
//! enum Method {
//!   GET,
//!   PUT,
//! };
//!
//! // Create a sample counter metric family utilizing the above custom label
//! // type, representing the number of HTTP requests received.
//! let http_requests = Family::<Labels, Counter<AtomicU64>>::default();
//!
//! // Register the metric family with the registry.
//! registry.register(
//!   // With the metric name.
//!   "http_requests",
//!   // And the metric help text.
//!   "Number of HTTP requests received",
//!   Box::new(http_requests.clone()),
//! );
//!
//! // Somewhere in your business logic record a single HTTP GET request.
//! http_requests.get_or_create(
//!     &Labels { method: Method::GET, path: "/metrics".to_string() }
//! ).inc();
//!
//! // When a monitoring system like Prometheus scrapes the local node, encode
//! // all metrics in the registry in the text format, and send the encoded
//! // metrics back.
//! let mut buffer = vec![];
//! encode(&mut buffer, &registry).unwrap();
//!
//! let expected = "# HELP http_requests Number of HTTP requests received.\n".to_owned() +
//!                "# TYPE http_requests counter\n" +
//!                "http_requests_total{method=\"GET\",path=\"/metrics\"} 1\n" +
//!                "# EOF\n";
//! assert_eq!(expected, String::from_utf8(buffer).unwrap());
//! ```
//! See [examples] directory for more.
//!
//! [examples]: https://github.com/mxinden/rust-open-metrics-client/tree/master/examples

pub mod encoding;
pub mod metrics;
pub mod registry;
