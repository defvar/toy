use crate::error::RocksError;
use rocksdb::{
    BlockBasedOptions, DBCompactionStyle, DBWithThreadMode, IteratorMode, MultiThreaded, WriteBatch,
};
use std::path::Path;

#[derive(Debug)]
pub struct Client {
    db: DBWithThreadMode<MultiThreaded>,
    current_cf: String,
}

impl Client {
    pub fn new<P: AsRef<Path>, N: AsRef<str>>(path: P, cf: N) -> Result<Client, RocksError> {
        let cf_opt = default_options();
        let opt = default_options();
        let desc = rocksdb::ColumnFamilyDescriptor::new(cf.as_ref(), cf_opt);
        Ok(Client {
            db: rocksdb::DB::open_cf_descriptors(&opt, path, [desc])?,
            current_cf: cf.as_ref().to_owned(),
        })
    }

    pub fn iter(
        &self,
    ) -> Result<impl Iterator<Item = Result<(Box<[u8]>, Box<[u8]>), RocksError>> + '_, RocksError>
    {
        match self.db.cf_handle(&self.current_cf) {
            Some(ref h) => Ok(self
                .db
                .iterator_cf(h, IteratorMode::Start)
                .map(|x| x.map_err(|e| e.into()))),
            None => Err(RocksError::error("column family handle not found.")),
        }
    }

    pub fn get<K: AsRef<[u8]>>(&self, k: K) -> Result<Option<Vec<u8>>, RocksError> {
        match self.db.cf_handle(&self.current_cf) {
            Some(ref h) => self.db.get_cf(h, k).map_err(|e| e.into()),
            None => Err(RocksError::error("column family handle not found.")),
        }
    }

    pub fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, k: K, v: V) -> Result<(), RocksError> {
        match self.db.cf_handle(&self.current_cf) {
            Some(ref h) => self.db.put_cf(h, k, v).map_err(|e| e.into()),
            None => Err(RocksError::error("column family handle not found.")),
        }
    }

    pub fn put_batch<K: AsRef<[u8]>, V: AsRef<[u8]>>(
        &self,
        values: &[(K, V)],
    ) -> Result<(), RocksError> {
        let h = match self.db.cf_handle(&self.current_cf) {
            Some(h) => h,
            None => return Err(RocksError::error("column family handle not found.")),
        };
        let mut batch = WriteBatch::default();
        for (k, v) in values {
            batch.put_cf(&h, k, v);
        }
        self.db.write(batch).map_err(|e| RocksError::error(e))
    }

    pub fn drop(&self) -> Result<(), RocksError> {
        let name = &self.current_cf.to_owned();
        self.db.drop_cf(name).map_err(|e| e.into())
    }
}

fn default_options() -> rocksdb::Options {
    let mut opt = rocksdb::Options::default();

    opt.create_if_missing(true);
    opt.create_missing_column_families(true);
    opt.set_max_write_buffer_number(4);
    opt.set_min_write_buffer_number_to_merge(2);
    opt.enable_statistics();

    // level compaction
    opt.set_compaction_style(DBCompactionStyle::Level);
    opt.set_level_zero_file_num_compaction_trigger(8);
    opt.set_level_zero_slowdown_writes_trigger(17);
    opt.set_level_zero_stop_writes_trigger(24);
    opt.set_num_levels(4);
    opt.set_max_bytes_for_level_base(536870912); //512MB
    opt.set_max_bytes_for_level_multiplier(8.0);

    //recommend
    //https://github.com/facebook/rocksdb/wiki/Setup-Options-and-Basic-Tuning
    opt.set_max_background_jobs(4);
    opt.set_bytes_per_sync(1048576);

    let mut block_opts = BlockBasedOptions::default();
    block_opts.set_block_size(16 * 1024);
    block_opts.set_cache_index_and_filter_blocks(true);
    block_opts.set_pin_l0_filter_and_index_blocks_in_cache(true);
    block_opts.set_format_version(4);

    opt.set_block_based_table_factory(&block_opts);

    opt
}
