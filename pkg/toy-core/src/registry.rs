use crate::context_box::BoxContextFactory;
use crate::service_box::BoxServiceFactory;
use std::any::TypeId;
use std::collections::HashMap;

pub struct Registry<Req, Err, InitErr> {
    services: HashMap<Key, Entry<Req, Err, InitErr>>,
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

struct Entry<Req, Err, InitErr> {
    service_factory: BoxServiceFactory<Req, Err, InitErr>,
    context_factory: BoxContextFactory,
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

    pub fn get(
        &self,
        kind: &str,
    ) -> Option<(&BoxServiceFactory<Req, Err, InitErr>, &BoxContextFactory)> {
        let key = Key::from::<Req, Err, InitErr>(kind.to_string());
        let e = self.services.get(&key);
        e.map(|x| (&x.service_factory, &x.context_factory))
    }

    pub fn set(
        &mut self,
        kind: &str,
        service_factory: BoxServiceFactory<Req, Err, InitErr>,
        context_factory: BoxContextFactory,
    ) {
        let key = Key::from::<Req, Err, InitErr>(kind.to_string());
        self.services.entry(key).or_insert_with(|| Entry {
            service_factory,
            context_factory,
        });
    }
}
