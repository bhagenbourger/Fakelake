use std::fs::File;

use crate::{config::Config, generate::output_format::OutputFormat};
use arrow_array::RecordBatch;
use arrow_json::writer::LineDelimitedWriter;

const JSON_EXTENSION: &str = ".json";

#[derive(Debug)]
pub struct OutputJson {
    writer: LineDelimitedWriter<File>,
}

impl OutputJson {
    pub fn new(config: &Config) -> OutputJson {
        let file_name = config.get_output_file_name(JSON_EXTENSION);
        let file = std::fs::File::create(file_name).unwrap();
        OutputJson {
            writer: LineDelimitedWriter::new(file),
        }
    }
}

impl OutputFormat for OutputJson {
    fn flush(&mut self) {
        self.writer.finish().unwrap();
    }
    fn get_extension(&self) -> &str {
        JSON_EXTENSION
    }
    fn write(&mut self, batch: &RecordBatch) {
        self.writer.write(batch).expect("Writing batch");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Column, Config, Info, OutputType};
    use crate::options::presence;
    use crate::providers::increment::integer::IncrementIntegerProvider;
    use crate::providers::random::bool::BoolProvider;
    use crate::providers::random::date::date::DateProvider;
    use crate::providers::random::date::datetime::DatetimeProvider;
    use crate::providers::random::number::f64::F64Provider;
    use crate::providers::random::string::alphanumeric::AlphanumericProvider;

    use yaml_rust::YamlLoader;

    fn get_config(nb_columns: u8, name: Option<String>, rows: Option<u32>) -> Config {
        let mut columns = vec![];

        for _ in 0..nb_columns {
            columns.push(Column {
                name: "id".to_string(),
                provider: Box::new(IncrementIntegerProvider { start: 0, step: 1 }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
                ),
            });
        }

        Config {
            columns,
            info: Some(Info {
                output_name: name,
                output_format: Some(OutputType::Json(true)),
                rows,
            }),
        }
    }

    #[test]
    fn given_get_extension() {
        let output = OutputJson { wrap_up: true };
        assert_eq!(output.get_extension(), ".json");
    }

    #[test]
    fn given_config_without_columns_should_error() {
        let config = get_config(0, None, None);
        let output = OutputJson { wrap_up: true };
        match output.generate_from_config(&config) {
            Err(_) => (),
            Ok(_) => panic!("Should fail"),
        }
    }

    #[test]
    fn given_config_without_info_should_write_file() {
        let config = get_config(1, None, None);
        let output = OutputJson { wrap_up: true };
        match output.generate_from_config(&config) {
            Ok(_) => (),
            Err(_) => panic!("Error"),
        }
    }

    #[test]
    fn given_config_should_write_file() {
        let config = get_config(1, Some("output_name".to_string()), Some(1000));
        let output = OutputJson { wrap_up: true };
        match output.generate_from_config(&config) {
            Ok(_) => (),
            Err(_) => panic!("Error"),
        }
    }

    #[test]
    fn given_all_providers_values_should_write_file() {
        let columns = vec![
            Column {
                name: "id".to_string(),
                provider: Box::new(IncrementIntegerProvider { start: 0, step: 1 }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
                ),
            },
            Column {
                name: "bool".to_string(),
                provider: Box::new(BoolProvider {}),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
                ),
            },
            Column {
                name: "id".to_string(),
                provider: Box::new(F64Provider { min: 0.0, max: 1.1 }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
                ),
            },
            Column {
                name: "id".to_string(),
                provider: Box::new(AlphanumericProvider {
                    min_length: 10,
                    max_length: 11,
                }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
                ),
            },
            Column {
                name: "id".to_string(),
                provider: Box::new(DateProvider {
                    format: "%Y-%m-%d".to_string(),
                    after: 0,
                    before: 10000,
                }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
                ),
            },
            Column {
                name: "id".to_string(),
                provider: Box::new(DatetimeProvider {
                    format: "%Y-%m-%d %H:%M:%S".to_string(),
                    after: 10_000_000,
                    before: 12_000_000,
                }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("name: id\npresence: 1").unwrap()[0],
                ),
            },
        ];

        let config = Config {
            columns,
            info: Some(Info {
                output_name: Some("output_name".to_string()),
                output_format: Some(OutputType::Json(true)),
                rows: Some(1000),
            }),
        };

        let output = OutputJson { wrap_up: true };
        match output.generate_from_config(&config) {
            Ok(_) => (),
            Err(_) => panic!("Error"),
        }
    }

    #[test]
    fn given_should_wrap_up_in_a_array() {
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
                output_name: Some("output_wrap_up".to_string()),
                output_format: Some(OutputType::Json(true)),
                rows: Some(5),
            }),
        };

        let output = OutputJson { wrap_up: true };
        match output.generate_from_config(&config) {
            Ok(_) => (),
            Err(_) => panic!("Error"),
        }

        assert_eq!(
            "[{\"id\":0},{\"id\":1},{\"id\":2},{\"id\":3},{\"id\":4}]",
            std::fs::read_to_string("output_wrap_up.json").unwrap()
        );
    }

    #[test]
    fn given_should_not_wrap_up() {
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
                output_name: Some("output_not_wrap_up".to_string()),
                output_format: Some(OutputType::Json(false)),
                rows: Some(5),
            }),
        };

        let output = OutputJson { wrap_up: false };
        match output.generate_from_config(&config) {
            Ok(_) => (),
            Err(_) => panic!("Error"),
        }

        assert_eq!(
            "{\"id\":0}\n{\"id\":1}\n{\"id\":2}\n{\"id\":3}\n{\"id\":4}\n",
            std::fs::read_to_string("output_not_wrap_up.json").unwrap()
        );
    }
}
