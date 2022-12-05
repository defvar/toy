use crate::config::{BufferFullStrategy, SortConfig, SortKey};
use crate::merge::{create_merge_reader, to_disk};
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::future::Future;
use std::path::PathBuf;
use toy_core::prelude::{
    Frame, Outgoing, PortType, Service, ServiceContext, ServiceError, ServiceFactory, ServiceType,
    TaskContext, Value,
};

#[derive(Clone, Debug)]
pub struct Sort;

pub struct SortContext {
    config: SortConfig,
    buffer: BinaryHeap<Reverse<Candidate>>,
    paths: HashSet<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub struct Candidate {
    key: Value,
    payload: Frame,
}

impl Candidate {
    pub fn from(key: Value, payload: Frame) -> Self {
        Self { key, payload }
    }

    pub fn key(&self) -> &Value {
        &self.key
    }

    pub fn payload(&self) -> &Frame {
        &self.payload
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl SortContext {
    pub async fn flush_if_needed(
        &mut self,
        task_ctx: &TaskContext,
        tx: &mut Outgoing<Frame, ServiceError>,
    ) -> Result<(), ServiceError> {
        if self.buffer.len() > (self.config.buffer_capacity() as usize) {
            self.flush0(task_ctx, tx).await
        } else {
            Ok(())
        }
    }

    pub async fn force_flush(
        &mut self,
        task_ctx: &TaskContext,
        tx: &mut Outgoing<Frame, ServiceError>,
    ) -> Result<(), ServiceError> {
        if self.buffer.len() > 0 {
            self.flush0(task_ctx, tx).await
        } else {
            Ok(())
        }
    }

    async fn flush0(
        &mut self,
        task_ctx: &TaskContext,
        tx: &mut Outgoing<Frame, ServiceError>,
    ) -> Result<(), ServiceError> {
        match self.config.buffer_full_strategy() {
            BufferFullStrategy::Flush => {
                while let Some(item) = self.buffer.pop() {
                    tx.send_ok(item.0.payload).await?;
                }
            }
            BufferFullStrategy::Persist { path: temp_path } => {
                let name = task_ctx.uri().to_string().replace("/", "-");
                let path = PathBuf::from(temp_path).join(format!(
                    "{}-{}-{}",
                    task_ctx.id().to_string(),
                    name,
                    self.paths.len()
                ));
                tracing::debug!("buffer full, create temp data. path: {:?}", path);
                to_disk(&path, &mut self.buffer).await?;
                self.paths.insert(path);
                self.buffer.clear();
            }
        }
        Ok(())
    }
}

impl Service for Sort {
    type Context = SortContext;
    type Request = Frame;
    type Future = impl Future<Output = Result<ServiceContext<SortContext>, ServiceError>> + Send;
    type UpstreamFinishFuture =
        impl Future<Output = Result<ServiceContext<SortContext>, ServiceError>> + Send;
    type UpstreamFinishAllFuture =
        impl Future<Output = Result<ServiceContext<SortContext>, ServiceError>> + Send;
    type Error = ServiceError;

    fn port_type() -> PortType {
        PortType::flow()
    }

    fn handle(
        &mut self,
        task_ctx: TaskContext,
        mut ctx: Self::Context,
        req: Self::Request,
        mut tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::Future {
        async move {
            match req.value() {
                Some(v) => {
                    let key = match ctx.config.sort_key() {
                        SortKey::Value => v,
                        SortKey::Name(n) => v.path(n).unwrap_or(v),
                        SortKey::Index(idx) => v
                            .as_vec()
                            .map(|x| x.get(*idx as usize).unwrap_or(v))
                            .unwrap_or(v),
                    };
                    ctx.buffer.push(Reverse(Candidate::from(key.clone(), req)));
                }
                None => {}
            }
            ctx.flush_if_needed(&task_ctx, &mut tx).await?;
            Ok(ServiceContext::Ready(ctx))
        }
    }

    fn upstream_finish(
        &mut self,
        _task_ctx: TaskContext,
        ctx: Self::Context,
        _req: Self::Request,
        _tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishFuture {
        async move { Ok(ServiceContext::Ready(ctx)) }
    }

    fn upstream_finish_all(
        &mut self,
        task_ctx: TaskContext,
        mut ctx: Self::Context,
        mut tx: Outgoing<Self::Request, Self::Error>,
    ) -> Self::UpstreamFinishAllFuture {
        async move {
            ctx.force_flush(&task_ctx, &mut tx).await?;
            match ctx.config.buffer_full_strategy() {
                BufferFullStrategy::Persist { .. } => {
                    let mut reader = create_merge_reader(&ctx.paths, 200).await?;
                    while let Some(v) = reader.next().await? {
                        tx.send_ok(v.payload).await?;
                    }
                }
                _ => (),
            };

            Ok(ServiceContext::Complete(ctx))
        }
    }
}

impl ServiceFactory for Sort {
    type Future = impl Future<Output = Result<Self::Service, Self::InitError>> + Send;
    type Service = Sort;
    type CtxFuture = impl Future<Output = Result<Self::Context, Self::InitError>> + Send;
    type Context = SortContext;
    type Config = SortConfig;
    type Request = Frame;
    type Error = ServiceError;
    type InitError = ServiceError;

    fn new_service(&self, _tp: ServiceType) -> Self::Future {
        async move { Ok(Sort) }
    }

    fn new_context(&self, _tp: ServiceType, config: Self::Config) -> Self::CtxFuture {
        let count = config.buffer_capacity();
        async move {
            Ok(SortContext {
                config,
                buffer: BinaryHeap::with_capacity(count as usize),
                paths: HashSet::new(),
            })
        }
    }
}
