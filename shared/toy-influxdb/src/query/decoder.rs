use crate::models::constants;
use crate::models::flux_table::{FluxTable, FluxTableBuilder};
use crate::InfluxDBError;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use toy_text_parser::dfa::{ByteParser, ByteParserBuilder, ParseResult};
use toy_text_parser::Line;

pub struct Decoder<T> {
    src: BufReader<T>,
    parser: ByteParser,
    builder: FluxTableBuilder,
    eof: bool,
    field_capacity: usize,
    line_buffer_size: usize,
}

impl<T> Decoder<T>
where
    T: Read,
{
    pub fn with(
        src: T,
        field_capacity: usize,
        row_capacity: usize,
        line_buffer_size: usize,
    ) -> Decoder<T> {
        Self {
            src: BufReader::new(src),
            parser: ByteParserBuilder::csv().build(),
            builder: FluxTableBuilder::with_capacity(field_capacity, row_capacity),
            eof: false,
            field_capacity,
            line_buffer_size,
        }
    }

    pub fn decode(mut self) -> Result<Vec<FluxTable>, InfluxDBError> {
        let mut buf = Line::with_capacity(self.line_buffer_size, self.field_capacity);
        let mut index = 0;
        let mut read_column_header = false;

        while self.read_line(&mut buf)? {
            let ident = buf.get(0);

            // start new table
            if read_column_header {
                match ident {
                    Some(v) if constants::ANNOTATION_IDENTS.contains(&v) => {
                        index += 1;
                        read_column_header = false;
                    }
                    _ => (),
                };
            }

            match ident {
                Some(v) if v == constants::ANNOTATION_IDENT_GROUP => {
                    self.read_group(index, &mut buf)?
                }
                Some(v) if v == constants::ANNOTATION_IDENT_DATATYPE => {
                    self.read_datetype(index, &mut buf)?
                }
                Some(v) if v == constants::ANNOTATION_IDENT_DEFAULT => (),
                Some(b"") => {
                    if read_column_header {
                        self.read_data(index, &mut buf)?;
                    } else {
                        self.read_header(index, &mut buf)?;
                        read_column_header = true;
                    }
                }
                _ => (),
            }
        }
        Ok(self.builder.build())
    }

    fn read_group(&mut self, index: usize, line: &mut Line) -> Result<(), InfluxDBError> {
        self.builder.group(index, as_str_iterator(line, 1));
        Ok(())
    }

    fn read_datetype(&mut self, index: usize, line: &mut Line) -> Result<(), InfluxDBError> {
        self.builder.data_type(index, as_str_iterator(line, 1));
        Ok(())
    }

    fn read_header(&mut self, index: usize, line: &mut Line) -> Result<(), InfluxDBError> {
        self.builder.headers(index, as_str_iterator(line, 1));
        Ok(())
    }

    fn read_data(&mut self, table_index: usize, line: &mut Line) -> Result<(), InfluxDBError> {
        let table_field_index = self.builder.get_field_index(table_index, "table");
        let table = match table_field_index {
            Some(i) => {
                let table_column = line
                    .get(i + 1)
                    .map(|x| std::str::from_utf8(x).ok())
                    .flatten();
                table_column.map(|x| x.parse::<u32>().ok()).flatten()
            }
            None => None,
        };
        let table = match table {
            Some(table) => table,
            None => return Err(InfluxDBError::error("not found or invalid [table] column.")),
        };
        self.builder.start_record(table_index, table);
        line.iter()
            .skip(1)
            .enumerate()
            .try_for_each(|(field_index, x)| {
                self.builder.push(field_index, std::str::from_utf8(x)?)
            })?;
        self.builder.end_record();
        Ok(())
    }

    #[inline]
    fn read_line(&mut self, line: &mut Line) -> Result<bool, InfluxDBError> {
        line.clear();

        if self.eof {
            return Ok(false);
        }

        let (mut out_pos, mut column) = (0, 0);
        loop {
            let (state, in_size, out_size, col) = {
                let input = self.src.fill_buf()?;
                let (buf, edges) = line.parts();
                self.parser
                    .read_record(input, &mut buf[out_pos..], &mut edges[column..])
            };
            self.src.consume(in_size);

            column += col;
            out_pos += out_size;

            match state {
                ParseResult::OutputFull => {
                    line.expand_force_columns();
                    continue;
                }
                ParseResult::OutputEdgeFull => {
                    line.expand_force_edges();
                    continue;
                }
                ParseResult::InputEmpty => continue,
                ParseResult::End => {
                    self.eof = true;
                    return Ok(false);
                }
                ParseResult::Record => {
                    line.set_len(column);
                    return Ok(true);
                }
            }
        }
    }
}

fn as_str_iterator(line: &Line, skip: usize) -> impl Iterator<Item = &str> {
    line.iter()
        .skip(skip)
        .map(|x| std::str::from_utf8(x))
        .flatten()
}
