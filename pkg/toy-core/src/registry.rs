use super::service::Service;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Registry {
    services: HashMap<String, Arc<Box<dyn Service>>>,
}

lazy_static::lazy_static! {
    pub static ref SERVICE_REG: Mutex<Registry> = {
        Mutex::new(Registry{
          services: HashMap::new(),
        })
    };
}

impl Registry {
    pub fn get(name: &str) -> Option<Arc<Box<dyn Service>>> {
        let sreg = SERVICE_REG.lock().unwrap();
        if let Some(arc) = sreg.services.get(name) {
            Some(arc.clone())
        } else {
            None
        }
    }

    pub fn set(name: &str, service: impl Service + 'static) {
        let mut sreg = SERVICE_REG.lock().unwrap();
        sreg.services
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Box::new(service)));
    }
}
