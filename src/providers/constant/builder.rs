use yaml_rust::Yaml;

use crate::{errors::FakeLakeError, providers::provider::Provider};

use super::string;

pub fn get_corresponding_provider(
    mut provider_split: std::str::Split<'_, char>,
    column: &Yaml,
) -> Result<Box<dyn Provider>, FakeLakeError> {
    match provider_split.next() {
        Some("string") => Ok(string::new_from_yaml(column)),
        _ => Err(FakeLakeError::BadYAMLFormat("".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::get_corresponding_provider;

    use yaml_rust::YamlLoader;

    #[test]
    fn given_string_should_return_provider() {
        let provider_name = "string";
        let yaml_str = format!("name: is_suscribed{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_wrong_provider_should_return_error() {
        let provider_name = "not_a_provider";
        let yaml_str = format!("name: email{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Err(_) => (),
            _ => panic!(),
        }
    }
}
