use crate::persist::error::PersistError;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs;
use toy_core::graph::Graph;
use toy_core::prelude::Value;

#[derive(Clone)]
pub struct GraphRegistry {
    root: PathBuf,
}

fn is_yaml(extension: Option<&OsStr>) -> bool {
    extension.map_or_else(|| false, |x| x == "yaml" || x == "yml")
}

impl GraphRegistry {
    pub fn new(root: impl Into<PathBuf>) -> GraphRegistry {
        GraphRegistry { root: root.into() }
    }

    pub fn root_path(&self) -> PathBuf {
        self.root.to_path_buf()
    }

    pub async fn list(&self) -> Result<Vec<Graph>, PersistError> {
        let mut entries = fs::read_dir(&self.root).await?;
        let mut r: Vec<Graph> = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            if is_yaml(entry.path().extension()) {
                let text = fs::read_to_string(entry.path()).await?;
                let v = toy_pack_yaml::unpack::<Value>(&text)?;
                r.push(Graph::from(v)?);
            }
        }
        Ok(r)
    }

    pub async fn find(&self, name: &str) -> Result<Option<Graph>, PersistError> {
        if let Some(ref path) = self.find0(name).await? {
            let text = fs::read_to_string(path).await?;
            let v = toy_pack_yaml::unpack::<Value>(&text)?;
            Ok(Some(Graph::from(v)?))
        } else {
            Ok(None)
        }
    }

    pub async fn put(&self, graph: Graph) -> Result<(), PersistError> {
        let name = graph.name();
        let c = graph.config();
        let text = toy_pack_yaml::pack_to_string(c)?;
        let mut path = PathBuf::from(&self.root);
        path.push(format!("{}.{}", name, "yml"));
        fs::write(path, text).await?;
        Ok(())
    }

    pub async fn remove(&self, name: &str) -> Result<bool, PersistError> {
        if let Some(path) = self.find0(name).await? {
            fs::remove_file(path).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn find0(&self, name: &str) -> Result<Option<PathBuf>, PersistError> {
        let mut entries = fs::read_dir(&self.root).await?;
        while let Some(entry) = entries.next_entry().await? {
            if is_yaml(entry.path().extension()) && entry.path().file_stem().unwrap() == name {
                return Ok(Some(entry.path()));
            }
        }
        Ok(None)
    }
}
