use std::{collections::HashMap, time::SystemTime};

use prometheus::proto::MetricFamily;
use reqwest::Client;

/// Special label for the name of a metric.
pub const LABEL_NAME: &str = "__name__";
pub const CONTENT_TYPE: &str = "application/x-protobuf";
pub const HEADER_NAME_REMOTE_WRITE_VERSION: &str = "X-Prometheus-Remote-Write-Version";
pub const REMOTE_WRITE_VERSION_01: &str = "0.1.0";
pub const COUNT_SUFFIX: &str = "_count";
pub const SUM_SUFFIX: &str = "_sum";
pub const TOTAL_SUFFIX: &str = "_total";

/// A label.
///
/// .proto:
/// ```protobuf
/// message Label {
///   string name  = 1;
///   string value = 2;
/// }
/// ```
#[derive(prost::Message, Clone, Hash, PartialEq, Eq)]
pub struct Label {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub value: String,
}

/// A sample.
///
/// .proto:
/// ```protobuf
/// message Sample {
///   double value    = 1;
///   int64 timestamp = 2;
/// }
/// ```
#[derive(prost::Message, Clone, PartialEq)]
pub struct Sample {
    #[prost(double, tag = "1")]
    pub value: f64,
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
}

pub enum ExtraLabel {
    LessThan(f64),
    Quantile(f64),
}

/// A time series.
///
/// .proto:
/// ```protobuf
/// message TimeSeries {
///   repeated Label labels   = 1;
///   repeated Sample samples = 2;
/// }
/// ```
#[derive(prost::Message, Clone, PartialEq)]
pub struct TimeSeries {
    #[prost(message, repeated, tag = "1")]
    pub labels: Vec<Label>,
    #[prost(message, repeated, tag = "2")]
    pub samples: Vec<Sample>,
}

impl TimeSeries {
    /// Sort labels by name, and the samples by timestamp.
    ///
    /// Required by the specification.
    pub fn sort_labels_and_samples(&mut self) {
        self.labels.sort_by(|a, b| a.name.cmp(&b.name));
        self.samples.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }
}

/// A write request.
///
/// .proto:
/// ```protobuf
/// message WriteRequest {
///   repeated TimeSeries timeseries = 1;
///   // Cortex uses this field to determine the source of the write request.
///   // We reserve it to avoid any compatibility issues.
///   reserved  2;

///   // Prometheus uses this field to send metadata, but this is
///   // omitted from v1 of the spec as it is experimental.
///   reserved  3;
/// }
/// ```
#[derive(prost::Message, Clone, PartialEq)]
pub struct WriteRequest {
    #[prost(message, repeated, tag = "1")]
    pub timeseries: Vec<TimeSeries>,
}

fn get_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

impl WriteRequest {
    /// Prepare the write request for sending.
    ///
    /// Ensures that the request conforms to the specification.
    /// See https://prometheus.io/docs/concepts/remote_write_spec.
    pub fn sort(&mut self) {
        for series in &mut self.timeseries {
            series.sort_labels_and_samples();
        }
    }

    pub fn sorted(mut self) -> Self {
        self.sort();
        self
    }

    /// Encode this write request as a protobuf message.
    ///
    /// NOTE: The API requires snappy compression, not a raw protobuf message.
    pub fn encode_proto3(self) -> Vec<u8> {
        prost::Message::encode_to_vec(&self.sorted())
    }

    pub fn encode_compressed(self) -> Result<Vec<u8>, snap::Error> {
        snap::raw::Encoder::new().compress_vec(&self.encode_proto3())
    }

    /// Encode Prometheus metric families into a WriteRequest
    pub fn from_metric_families(
        metric_families: Vec<MetricFamily>,
        custom_labels: Option<Vec<(String, String)>>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut timeseries = Vec::new();
        let now = get_timestamp();
        let custom_labels = custom_labels.unwrap_or_default();
        metric_families
            .iter()
            .for_each(|mf| match mf.get_field_type() {
                prometheus::proto::MetricType::GAUGE => {
                    mf.get_metric().iter().for_each(|m| {
                        let mut labels = m
                            .get_label()
                            .iter()
                            .map(|l| (l.name().to_string(), l.value().to_string()))
                            .collect::<Vec<_>>();
                        labels.push((LABEL_NAME.to_string(), mf.name().to_string()));
                        labels.extend_from_slice(&custom_labels);

                        let samples = vec![Sample {
                            value: m.get_gauge().value(),
                            timestamp: now,
                        }];

                        timeseries.push(TimeSeries {
                            labels: labels
                                .iter()
                                .map(|(k, v)| Label {
                                    name: k.to_string(),
                                    value: v.to_string(),
                                })
                                .collect::<Vec<_>>(),
                            samples,
                        });
                    });
                }
                prometheus::proto::MetricType::COUNTER => {
                    mf.get_metric().iter().for_each(|m| {
                        let mut labels = m
                            .get_label()
                            .iter()
                            .map(|l| (l.name().to_string(), l.value().to_string()))
                            .collect::<Vec<_>>();
                        labels.push((LABEL_NAME.to_string(), mf.name().to_string()));
                        labels.extend_from_slice(&custom_labels);
                        let samples = vec![Sample {
                            value: m.get_counter().value(),
                            timestamp: now,
                        }];

                        timeseries.push(TimeSeries {
                            labels: labels
                                .iter()
                                .map(|(k, v)| Label {
                                    name: k.to_string(),
                                    value: v.to_string(),
                                })
                                .collect::<Vec<_>>(),
                            samples,
                        });
                    });
                }
                prometheus::proto::MetricType::SUMMARY => {
                    mf.get_metric().iter().for_each(|m| {
                        let mut labels = m
                            .get_label()
                            .iter()
                            .map(|l| (l.name().to_string(), l.value().to_string()))
                            .collect::<HashMap<String, String>>();
                        labels.insert(LABEL_NAME.to_string(), mf.name().to_string());
                        custom_labels.iter().for_each(|(k, v)| {
                            labels.insert(k.to_string(), v.to_string());
                        });
                        m.get_summary().get_quantile().iter().for_each(|quantile| {
                            let mut our_labels = labels.clone();
                            our_labels.insert(
                                "quantile".to_string(),
                                quantile.quantile().to_string(),
                            );
                            let samples = vec![Sample {
                                value: quantile.value(),
                                timestamp: now,
                            }];
                            timeseries.push(TimeSeries {
                                labels: our_labels
                                    .iter()
                                    .map(|(k, v)| Label {
                                        name: k.to_string(),
                                        value: v.to_string(),
                                    })
                                    .collect::<Vec<_>>(),
                                samples,
                            });
                        });
                        let mut top_level_labels = labels.clone();
                        top_level_labels.insert(
                            LABEL_NAME.to_string(),
                            format!("{}{}", mf.name(), SUM_SUFFIX),
                        );
                        timeseries.push(TimeSeries {
                            samples: vec![Sample {
                                value: m.get_summary().sample_sum(),
                                timestamp: now,
                            }],
                            labels: top_level_labels
                                .iter()
                                .map(|(k, v)| Label {
                                    name: k.to_string(),
                                    value: v.to_string(),
                                })
                                .collect(),
                        });
                        top_level_labels.insert(
                            LABEL_NAME.to_string(),
                            format!("{}{}", mf.name(), COUNT_SUFFIX),
                        );
                        timeseries.push(TimeSeries {
                            samples: vec![Sample {
                                value: m.get_summary().sample_count() as f64,
                                timestamp: now,
                            }],
                            labels: top_level_labels
                                .iter()
                                .map(|(k, v)| Label {
                                    name: k.to_string(),
                                    value: v.to_string(),
                                })
                                .collect(),
                        });
                    });
                }
                prometheus::proto::MetricType::UNTYPED => {}
                prometheus::proto::MetricType::HISTOGRAM => {
                    mf.get_metric().iter().for_each(|m| {
                        let mut labels = m
                            .get_label()
                            .iter()
                            .map(|l| (l.name().to_string(), l.value().to_string()))
                            .collect::<HashMap<String, String>>();
                        labels.insert(LABEL_NAME.to_string(), mf.name().to_string());
                        custom_labels.iter().for_each(|(k, v)| {
                            labels.insert(k.to_string(), v.to_string());
                        });
                        m.get_histogram().get_bucket().iter().for_each(|bucket| {
                            let mut our_labels = labels.clone();
                            our_labels
                                .insert("le".to_string(), bucket.upper_bound().to_string());
                            let samples = vec![Sample {
                                value: bucket.cumulative_count() as f64,
                                timestamp: now,
                            }];
                            timeseries.push(TimeSeries {
                                labels: our_labels
                                    .iter()
                                    .map(|(k, v)| Label {
                                        name: k.to_string(),
                                        value: v.to_string(),
                                    })
                                    .collect::<Vec<_>>(),
                                samples,
                            });
                        });
                        let mut top_level_labels = labels.clone();
                        top_level_labels.insert(
                            LABEL_NAME.to_string(),
                            format!("{}{}", mf.name(), SUM_SUFFIX),
                        );
                        timeseries.push(TimeSeries {
                            samples: vec![Sample {
                                value: m.get_histogram().get_sample_sum(),
                                timestamp: now,
                            }],
                            labels: top_level_labels
                                .iter()
                                .map(|(k, v)| Label {
                                    name: k.to_string(),
                                    value: v.to_string(),
                                })
                                .collect(),
                        });
                        top_level_labels.insert(
                            LABEL_NAME.to_string(),
                            format!("{}{}", mf.name(), COUNT_SUFFIX),
                        );
                        timeseries.push(TimeSeries {
                            samples: vec![Sample {
                                value: m.get_histogram().get_sample_count() as f64,
                                timestamp: now,
                            }],
                            labels: top_level_labels
                                .iter()
                                .map(|(k, v)| Label {
                                    name: k.to_string(),
                                    value: v.to_string(),
                                })
                                .collect(),
                        });
                        top_level_labels.insert(LABEL_NAME.to_string(), mf.name().to_string());
                        top_level_labels.insert("le".into(), "+Inf".into());
                        timeseries.push(TimeSeries {
                            samples: vec![Sample {
                                value: m.get_histogram().get_sample_count() as f64,
                                timestamp: now,
                            }],
                            labels: top_level_labels
                                .iter()
                                .map(|(k, v)| Label {
                                    name: k.to_string(),
                                    value: v.to_string(),
                                })
                                .collect(),
                        });
                    });
                }
            });
        timeseries.sort_by(|a, b| {
            let name_a = a.labels.iter().find(|l| l.name == LABEL_NAME).unwrap();
            let name_b = b.labels.iter().find(|l| l.name == LABEL_NAME).unwrap();
            name_a.value.cmp(&name_b.value)
        });
        let s = Self { timeseries };
        Ok(s.sorted())
    }

    pub fn build_http_request(
        self,
        client: Client,
        endpoint: &str,
        user_agent: &str,
    ) -> Result<reqwest::Request, reqwest::Error> {
        client
            .post(endpoint)
            .header(reqwest::header::CONTENT_TYPE, CONTENT_TYPE)
            .header(HEADER_NAME_REMOTE_WRITE_VERSION, REMOTE_WRITE_VERSION_01)
            .header(reqwest::header::CONTENT_ENCODING, "snappy")
            .header(reqwest::header::USER_AGENT, user_agent)
            .body(
                self.encode_compressed()
                    .expect("Failed to compress metrics data"),
            )
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use prometheus::{histogram_opts, Counter, Gauge, Histogram, Registry};

    #[test]
    pub fn can_encode_counter() {
        let registry = Registry::new();
        let counter_name = "my_counter";
        let help = "an extra description";
        let counter = Counter::new(counter_name, help).unwrap();
        registry.register(Box::new(counter.clone())).unwrap();
        let incremented_by = 5.0;
        counter.inc_by(incremented_by);
        let req = WriteRequest::from_metric_families(registry.gather(), None)
            .expect("Failed to encode counter");
        assert_eq!(req.timeseries.len(), 1);
        let entry = req.timeseries.first().unwrap();
        assert_eq!(entry.labels.len(), 1);
        assert_eq!(
            entry
                .labels
                .iter()
                .find(|l| l.name == LABEL_NAME)
                .unwrap()
                .value,
            counter_name
        );
        assert_eq!(entry.samples.first().unwrap().value, incremented_by);
    }
    #[test]
    pub fn can_encode_gauge() {
        let registry = Registry::new();
        let gauge_name = "my_gauge";
        let help = "an extra description";
        let counter = Gauge::new(gauge_name, help).unwrap();
        registry.register(Box::new(counter.clone())).unwrap();
        let incremented_by = 5.0;
        counter.set(incremented_by);
        let req = WriteRequest::from_metric_families(registry.gather(), None)
            .expect("Failed to encode gauge");
        assert_eq!(req.timeseries.len(), 1);
        let entry = req.timeseries.first().unwrap();
        assert_eq!(entry.labels.len(), 1);
        assert_eq!(
            entry
                .labels
                .iter()
                .find(|l| l.name == LABEL_NAME)
                .unwrap()
                .value,
            gauge_name
        );
        assert_eq!(entry.samples.first().unwrap().value, incremented_by);
    }
    #[test]
    pub fn can_encode_histogram() {
        let registry = Registry::new();
        let histogram_name = "my_histogram";
        let help = "an extra description".to_string();
        let opts = histogram_opts!(histogram_name, help, vec![10.0, 1000.0, 10000.0]);
        let histogram = Histogram::with_opts(opts).unwrap();
        registry.register(Box::new(histogram.clone())).unwrap();
        histogram.observe(5.0);
        histogram.observe(500.0);
        histogram.observe(5000.0);
        histogram.observe(50000.0);
        let req = WriteRequest::from_metric_families(registry.gather(), None)
            .expect("Failed to encode histogram");
        assert_eq!(req.timeseries.len(), 6);
        let bucket_names: Vec<String> = req
            .timeseries
            .clone()
            .into_iter()
            .filter_map(|ts| {
                ts.labels
                    .iter()
                    .find(|l| l.name == "le")
                    .map(|l| l.value.clone())
            })
            .collect();
        assert_eq!(bucket_names, vec!["10", "1000", "10000", "+Inf"]);

        let count_observations = req
            .timeseries
            .clone()
            .iter()
            .find(|l| {
                l.labels.iter().any(|label| {
                    label.name == LABEL_NAME
                        && label.value == format!("{}{}", histogram_name, COUNT_SUFFIX)
                })
            })
            .map(|ts| ts.samples.first().unwrap().value)
            .unwrap();
        assert_eq!(count_observations, 4.0);
        let sum_observation = req
            .timeseries
            .iter()
            .find(|l| {
                l.labels.iter().any(|label| {
                    label.name == LABEL_NAME
                        && label.value == format!("{}{}", histogram_name, SUM_SUFFIX)
                })
            })
            .map(|ts| ts.samples.first().unwrap().value)
            .unwrap();
        assert_eq!(sum_observation, 55505.0)
    }
    #[test]
    pub fn can_add_custom_labels() {
        let registry = Registry::new();
        let counter_name = "my_counter";
        let help = "an extra description";
        let counter = Counter::new(counter_name, help).unwrap();
        registry.register(Box::new(counter.clone())).unwrap();
        let incremented_by = 5.0;
        counter.inc_by(incremented_by);
        let req = WriteRequest::from_metric_families(
            registry.gather(),
            Some(vec![("foo".into(), "bar".into())]),
        )
        .expect("Failed to encode counter");
        assert_eq!(req.timeseries.len(), 1);
        let entry = req.timeseries.first().unwrap();
        assert_eq!(entry.labels.len(), 2);
        assert_eq!(
            entry
                .labels
                .iter()
                .find(|l| l.name == LABEL_NAME)
                .unwrap()
                .value,
            counter_name
        );
        assert_eq!(
            entry.labels.iter().find(|l| l.name == "foo").unwrap().value,
            "bar"
        );
        assert_eq!(entry.samples.first().unwrap().value, incremented_by);
    }
}
