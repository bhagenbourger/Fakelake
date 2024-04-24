pub mod batch_generator;
pub mod utils;

use crate::config::Config;
use crate::generate::output_format::OutputFormat;

use arrow_array::RecordBatch;
use arrow_schema::{Field, Schema};
use log::debug;
use parquet::{arrow::ArrowWriter, basic::Compression, file::properties::WriterProperties};
use std::{fs::File, sync::Arc};

const PARQUET_EXTENSION: &str = ".parquet";

#[derive(Debug)]
pub struct OutputParquet {
    pub writer: ArrowWriter<File>,
}

impl OutputParquet {
    pub fn new(config: &Config) -> OutputParquet {
        let schema = get_schema_from_config(config);
        debug!("Writing schema: {:?}", schema);

        // WriterProperties can be used to set Parquet file options
        let props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();

        let file_name = config.get_output_file_name(PARQUET_EXTENSION);
        let file = std::fs::File::create(file_name).unwrap();
        OutputParquet {
            writer: ArrowWriter::try_new(file, Arc::new(schema.clone()), Some(props)).unwrap(),
        }
    }
}

impl OutputFormat for OutputParquet {
    fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
    fn get_extension(&self) -> &str {
        PARQUET_EXTENSION
    }
    fn write(&mut self, batch: &RecordBatch) {
        self.writer.write(batch).expect("Writing batch");
    }
}

fn get_schema_from_config(config: &Config) -> Schema {
    let mut fields = Vec::new();

    for column in &config.columns {
        let parquet_type = utils::get_parquet_type_from_column(column.clone());
        fields.push(Field::new(&column.name, parquet_type, column.can_be_null()));
    }

    Schema::new(fields)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Column, Config, Info};
    use crate::options::presence;
    use crate::providers::increment::integer::IncrementIntegerProvider;

    use yaml_rust::YamlLoader;

    #[test]
    fn given_config_get_schema() {
        let columns = vec![Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0, step: 1 }),
            presence: presence::new_from_yaml(
                &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
            ),
        }];

        let config = Config {
            columns,
            info: Some(Info {
                output_name: None,
                output_format: None,
                rows: None,
            }),
        };
        let schema = get_schema_from_config(&config);

        assert_eq!(schema.fields().len(), 1);
        assert_eq!(schema.fields()[0].name(), "id");
    }

    #[test]
    fn given_get_extension() {
        let output_parquet = OutputParquet {};
        assert_eq!(output_parquet.get_extension(), ".parquet");
    }

    #[test]
    fn given_normal_config_should_generate_file() {
        let columns = vec![Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0, step: 1 }),
            presence: presence::new_from_yaml(
                &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
            ),
        }];
        let config = Config {
            columns,
            info: Some(Info {
                output_name: Some("not_default_file".to_string()),
                output_format: None,
                rows: None,
            }),
        };

        let output_parquet = OutputParquet {};
        match output_parquet.generate_from_config(&config) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_no_column_should_not_generate_file() {
        let columns = Vec::new();
        let config = Config {
            columns,
            info: Some(Info {
                output_name: Some("not_default_file".to_string()),
                output_format: None,
                rows: None,
            }),
        };

        let output_parquet = OutputParquet {};
        match output_parquet.generate_from_config(&config) {
            Err(_) => (),
            _ => panic!(),
        }
    }
}
