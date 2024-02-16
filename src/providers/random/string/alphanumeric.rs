use crate::providers::provider::{Provider, Value};
use crate::providers::utils::string::random_characters;

use yaml_rust::Yaml;

#[derive(Clone)]
pub struct AlphanumericProvider;

impl Provider for AlphanumericProvider {
    fn value(&self, _: u32) -> Value {
        Value::String(random_characters(10))
    }
    fn new_from_yaml(_: &Yaml) -> AlphanumericProvider {
        AlphanumericProvider
    }
}

#[cfg(test)]
mod tests {
    use super::AlphanumericProvider;
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider() -> AlphanumericProvider {
        let yaml_str = "name: id".to_string();

        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        AlphanumericProvider::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider: AlphanumericProvider = AlphanumericProvider;
        match provider.value(0) {
            Value::String(_) => (),
            _ => panic!(),
        };
    }

    // Validate YAML file
    #[test]
    fn given_no_config_should_return_default() {
        let _: AlphanumericProvider = generate_provider();
    }

    // Validate value calculation
    #[test]
    fn given_index_x_should_return_random_string_of_length_10() {
        let provider = AlphanumericProvider;

        let values_to_check = [0, 4, 50];
        for value in values_to_check {
            match provider.value(value) {
                Value::String(value) => assert_eq!(value.len(), 10),
                _ => panic!("Wrong type"),
            }
        }
    }
}