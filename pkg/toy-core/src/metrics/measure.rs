use serde::{Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Counter {
    raw: Option<AtomicU64>,
}

impl Counter {
    pub fn noop() -> Counter {
        Counter { raw: None }
    }

    pub fn from(v: u64) -> Counter {
        Counter {
            raw: Some(AtomicU64::new(v)),
        }
    }

    pub fn get(&self) -> Option<u64> {
        if let Some(r) = &self.raw {
            Some(r.load(Ordering::Relaxed))
        } else {
            None
        }
    }

    pub fn increment(&self) {
        if let Some(r) = &self.raw {
            r.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn increment_by(&self, v: u64) {
        if let Some(r) = &self.raw {
            r.fetch_add(v, Ordering::Relaxed);
        }
    }

    pub fn absolute(&self, v: u64) {
        if let Some(r) = &self.raw {
            r.fetch_max(v, Ordering::AcqRel);
        }
    }
}

impl Clone for Counter {
    fn clone(&self) -> Self {
        if let Some(v) = self.get() {
            Counter::from(v)
        } else {
            Counter::noop()
        }
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = self.get() {
            f.write_fmt(format_args!("Counter({})", v))
        } else {
            f.write_str("Counter(None)")
        }
    }
}

impl Debug for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Serialize for Counter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(&self.get())
    }
}

pub struct Gauge {
    raw: Option<AtomicU64>,
}

impl Gauge {
    pub fn noop() -> Gauge {
        Gauge { raw: None }
    }

    pub fn from(v: f64) -> Gauge {
        Gauge {
            raw: Some(AtomicU64::new(v.to_bits())),
        }
    }

    pub fn get(&self) -> Option<f64> {
        if let Some(r) = &self.raw {
            Some(f64::from_bits(r.load(Ordering::Relaxed)))
        } else {
            None
        }
    }

    pub fn increment(&self, v: f64) {
        if let Some(r) = &self.raw {
            loop {
                let result = r.fetch_update(Ordering::AcqRel, Ordering::Relaxed, |cur| {
                    let input = f64::from_bits(cur);
                    let output = input + v;
                    Some(output.to_bits())
                });
                if result.is_ok() {
                    break;
                }
            }
        }
    }

    pub fn decrement(&self, v: f64) {
        if let Some(r) = &self.raw {
            loop {
                let result = r.fetch_update(Ordering::AcqRel, Ordering::Relaxed, |cur| {
                    let input = f64::from_bits(cur);
                    let output = input - v;
                    Some(output.to_bits())
                });
                if result.is_ok() {
                    break;
                }
            }
        }
    }

    pub fn set(&self, v: f64) {
        if let Some(r) = &self.raw {
            let _ = r.swap(v.to_bits(), Ordering::AcqRel);
        }
    }
}

impl Clone for Gauge {
    fn clone(&self) -> Self {
        if let Some(v) = self.get() {
            Gauge::from(v)
        } else {
            Gauge::noop()
        }
    }
}

impl Display for Gauge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = self.get() {
            f.write_fmt(format_args!("Gauge({})", v))
        } else {
            f.write_str("Gauge(None)")
        }
    }
}

impl Debug for Gauge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Serialize for Gauge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(&self.get())
    }
}
