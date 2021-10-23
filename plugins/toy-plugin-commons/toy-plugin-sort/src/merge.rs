use crate::service::Candidate;
use bytes::Bytes;
use futures_util::SinkExt;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio_stream::StreamExt;
use tokio_util::codec::Decoder;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use toy_core::error::ServiceError;

pub async fn to_disk(
    path: &Path,
    items: &mut BinaryHeap<Reverse<Candidate>>,
) -> Result<(), ServiceError> {
    let f = tokio::fs::File::create(path).await?;
    let mut writer = Framed::new(f, LengthDelimitedCodec::new());

    while let Some(Reverse(item)) = items.pop() {
        let v = toy_pack_mp::pack(&item).map_err(|e| ServiceError::error(e))?;
        writer.send(Bytes::from(v)).await?;
    }
    Ok(())
}

pub async fn create_merge_reader(
    paths: &HashSet<PathBuf>,
    _merge_file_limit: u32,
) -> Result<MergeReader, ServiceError> {
    // TODO: file open limit....
    // if paths.len() > (merge_file_limit as usize){
    //     paths[..merge_file_limit]
    // } else {
    merge_reader0(paths).await
    // }
}

async fn merge_reader0(paths: &HashSet<PathBuf>) -> Result<MergeReader, ServiceError> {
    let mut readers = Vec::new();
    for p in paths {
        let f = tokio::fs::File::open(p)
            .await
            .map_err(|e| ServiceError::error(e))?;
        let lines = LengthDelimitedCodec::new().framed(f);
        readers.push(lines);
    }
    let mut r = MergeReader {
        readers,
        buffer: BinaryHeap::new(),

        prev_poped_idx: None,
    };
    r.prepare().await?;
    Ok(r)
}

pub struct MergeReader {
    readers: Vec<Framed<File, LengthDelimitedCodec>>,
    buffer: BinaryHeap<Reverse<MergeOutput>>,

    prev_poped_idx: Option<usize>,
}

impl MergeReader {
    pub async fn prepare(&mut self) -> Result<(), ServiceError> {
        for idx in 0..self.readers.len() {
            self.read_buf(idx).await?;
        }
        self.prev_poped_idx = None;
        Ok(())
    }

    pub async fn next(&mut self) -> Result<Option<Candidate>, ServiceError> {
        if self.prev_poped_idx.is_some() {
            self.read_buf(self.prev_poped_idx.unwrap()).await?;
        }

        let p = self.buffer.pop();
        match p {
            Some(Reverse(v)) => {
                self.prev_poped_idx = Some(v.reader_index as usize);
                Ok(Some(v.candidate))
            }
            None => Ok(None),
        }
    }

    async fn read_buf(&mut self, idx: usize) -> Result<Option<()>, ServiceError> {
        let r = &mut self.readers[idx];
        let item = r.next().await;
        match item {
            Some(Ok(bytes)) => {
                let candidate =
                    toy_pack_mp::unpack::<Candidate>(&bytes).map_err(|e| ServiceError::error(e))?;
                self.buffer
                    .push(Reverse(MergeOutput::new(candidate, idx as u32)));
                Ok(Some(()))
            }
            Some(Err(e)) => return Err(ServiceError::error(e)),
            None => Ok(None),
        }
    }
}

struct MergeOutput {
    candidate: Candidate,
    reader_index: u32,
}

impl MergeOutput {
    pub fn new(candidate: Candidate, reader_index: u32) -> Self {
        Self {
            candidate,
            reader_index,
        }
    }
}

impl PartialEq for MergeOutput {
    fn eq(&self, other: &Self) -> bool {
        self.candidate.key() == other.candidate.key()
    }
}

impl Eq for MergeOutput {}

impl PartialOrd for MergeOutput {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.candidate.key().partial_cmp(&other.candidate.key())
    }
}

impl Ord for MergeOutput {
    fn cmp(&self, other: &Self) -> Ordering {
        self.candidate.key().cmp(&other.candidate.key())
    }
}
