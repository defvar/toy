use std::io::Error;

use toy_plugin_file::config::{FileConfig, SinkConfig, SourceConfig};
use toy_plugin_file::{FileReaderBuilder, FileWriterBuilder, Row};

fn main() {
    let config: FileConfig = toy_config::from_yaml("toy-file/src/bin/csv.yml").unwrap();

    println!("config {:?}", config);

    match file(config.get_source_config().unwrap(), config.get_sink_config().unwrap()) {
        Ok(()) => println!("end"),
        Err(e) => println!("error! {}", e),
    };
}

fn file(src_config: &SourceConfig, sink_config: &SinkConfig) -> Result<(), Error> {
    let mut source = FileReaderBuilder::configure(src_config)?;
    let mut sink = FileWriterBuilder::configure(sink_config)?;
    let mut line = 0u32;
    let mut row = Row::new();

    // header_read
    let h = source.headers()?;
    println!("headers:{:?}", h);

    // header_write
    sink.write_iter([0, 1, 2].iter().map(|&i| &h[i])).unwrap();
    line += 1;

    while source.read(&mut row).unwrap() {
        line += 1;
        sink.write_iter([0, 1, 2].iter().map(|&i| &row[i])).unwrap();
    }

    sink.flush()?;

    println!("read line:{:?}", line);
    println!(
        "wrote_bytes:{:?}, wrote_row:{:?}",
        sink.get_wrote_bytes(),
        sink.get_wrote_row()
    );

    Ok(())
}
