use async_trait::async_trait;

#[async_trait]
pub trait GraphClient {
    async fn list();
    async fn find();
    async fn put();
    async fn delete();
}
