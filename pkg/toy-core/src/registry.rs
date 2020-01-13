use crate::service_box::BoxServiceFactory;
use std::any::TypeId;
use std::collections::HashMap;

pub struct Registry<Req, Err, InitErr> {
    services: HashMap<Key, BoxServiceFactory<Req, Err, InitErr>>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Key {
    kind: String,
    req: TypeId,
    err: TypeId,
    init_err: TypeId,
}

impl Key {
    fn from<Req, Err, InitErr>(kind: String) -> Key
    where
        Req: 'static,
        Err: 'static,
        InitErr: 'static,
    {
        Key {
            kind,
            req: TypeId::of::<Req>(),
            err: TypeId::of::<Err>(),
            init_err: TypeId::of::<InitErr>(),
        }
    }
}

impl<Req, Err, InitErr> Registry<Req, Err, InitErr>
where
    Req: 'static,
    Err: 'static,
    InitErr: 'static,
{
    pub fn new() -> Registry<Req, Err, InitErr> {
        Registry {
            services: HashMap::new(),
        }
    }

    pub fn get(&self, kind: &str) -> Option<&BoxServiceFactory<Req, Err, InitErr>> {
        let key = Key::from::<Req, Err, InitErr>(kind.to_string());
        self.services.get(&key)
    }

    pub fn set(&mut self, kind: &str, factory: BoxServiceFactory<Req, Err, InitErr>) {
        let key = Key::from::<Req, Err, InitErr>(kind.to_string());
        self.services.entry(key).or_insert_with(|| factory);
    }
}
