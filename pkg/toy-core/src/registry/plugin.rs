use crate::data::Frame;
use crate::executor::ServiceExecutor;
use crate::registry::{App, ExecuteResult, Registry, ServiceSchema};
use crate::{ServiceType, Uri};

#[derive(Clone)]
pub struct Plugin<L, R> {
    other: L,
    layer: R,
}

impl<L, R> Plugin<L, R>
where
    Self: Sized,
    L: Registry,
    R: Registry,
{
    pub fn new(init: L, layer: R) -> Plugin<L, R> {
        Plugin { layer, other: init }
    }

    pub fn with<F>(self, layer: F) -> Plugin<Self, F>
    where
        F: Registry,
    {
        Plugin::<Self, F>::new(self, layer)
    }

    pub fn build(self) -> App<Self> {
        App::new(self)
    }
}

impl<L, R> Registry for Plugin<L, R>
where
    L: Registry,
    R: Registry,
{
    fn service_types(&self) -> Vec<ServiceType> {
        let mut vec = self.other.service_types();
        vec.extend(self.layer.service_types());
        vec
    }

    fn schemas(&self) -> Vec<ServiceSchema> {
        let mut vec = self.other.schemas();
        vec.extend(self.layer.schemas());
        vec
    }

    fn delegate<T>(&self, tp: &ServiceType, uri: &Uri, executor: &mut T) -> ExecuteResult
    where
        T: ServiceExecutor<Request = Frame>,
    {
        match self.other.delegate(tp, uri, executor) {
            ExecuteResult::Done => ExecuteResult::Done,
            ExecuteResult::NotFound => self.layer.delegate(tp, uri, executor),
        }
    }
}

impl<L, R> std::fmt::Debug for Plugin<L, R>
where
    L: Registry,
    R: Registry,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plugin {{ services:[{:?}] }}", self.service_types())
    }
}
