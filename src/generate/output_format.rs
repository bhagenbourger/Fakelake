use std::sync::{Arc, Mutex};

use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use log::debug;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::config::Config;
use crate::errors::FakeLakeError;

use super::parquet::batch_generator::{parquet_batch_generator_builder, ParquetBatchGenerator};

pub trait OutputFormat {
    fn flush(&mut self);
    fn get_extension(&self) -> &str;
    fn write(&mut self, batch: &RecordBatch);

    fn generate_from_config(&mut self, config: &Config) -> Result<(), FakeLakeError> {
        if config.columns.is_empty() {
            return Err(FakeLakeError::BadYAMLFormat(
                "No columns to generate".to_string(),
            ));
        }

        let rows = config.get_number_of_rows();

        let batch_size = 8192 * 8;
        // ceil division
        let iterations = (rows as f64 / batch_size as f64).ceil() as u32;

        let mut schema_cols: Vec<(String, ArrayRef)> = Vec::new();
        let mut provider_generators: Vec<Box<dyn ParquetBatchGenerator>> = Vec::new();
        config.columns.clone().into_iter().for_each(|column| {
            schema_cols.push((
                column.clone().name,
                Arc::new(Int32Array::from(vec![0])) as ArrayRef,
            ));
            provider_generators.push(parquet_batch_generator_builder(column.clone()))
        });

        for i in 0..iterations {
            debug!("Generating batch {} of {}...", i, iterations);
            let rows_to_generate = if i == iterations - 1 {
                rows - (i * batch_size)
            } else {
                batch_size
            };

            let schema_cols: Mutex<Vec<(String, ArrayRef)>> = Mutex::new(schema_cols.clone());
            let provider_generators = provider_generators.clone();

            provider_generators.into_par_iter().enumerate().for_each(
                |(index, provider_generator)| {
                    let array = provider_generator.batch_array(rows_to_generate);
                    schema_cols.lock().unwrap()[index] =
                        (provider_generator.name().to_string(), array);
                },
            );

            let batch = RecordBatch::try_from_iter(schema_cols.lock().unwrap().clone()).unwrap();
            self.write(&batch);
        }
        // writer must be closed to write footer
        self.flush();
        Ok(())
    }
}
