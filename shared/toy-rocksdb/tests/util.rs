use tempdir::TempDir;
use toy_rocksdb::Client;

pub fn setup(cf: &str) -> Client {
    let tmp_dir = TempDir::new("toy-rocksdb-test").unwrap();
    let path = tmp_dir.path().join(cf);
    let mut opt = rocksdb::Options::default();
    opt.create_if_missing(true);
    opt.create_missing_column_families(true);
    rocksdb::DB::destroy(&opt, &path).unwrap();
    Client::new(&path, cf).unwrap()
}
