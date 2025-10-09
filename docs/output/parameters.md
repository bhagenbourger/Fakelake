Output parameters
--------------

### Generate file name
To change the name of the generated file, use output_name
```yaml
info:
  output_name: generate_file_name
```

### Format
To choose the format of the generated file, use output_format.
##### Parquet
```yaml
info:
 output_format: parquet
```

##### CSV
```yaml
info:
 output_format: csv
 delimiter: ','
```
Default delimiter is ',' but you can specify any character.

##### JSON
```yaml
info:
 output_format: json
 wrap_up: false
```
By default, wrap_up is set to false.  
When wrap_up is set to false, each line into the result file is a json object but the whole file is not a valid json.  
When wrap_up is set to true, the whole file is a valid json, rows are wrapped up into an array.

### Rows
To choose the number of rows in the generated file, use rows.
```yaml
info:
 rows: 1000000
```
It can also be written with delimiters for readibilty.
```yaml
info:
 rows: 1_000_000
```

### Seed
To make the generated data deterministic (reproducible), use seed with an integer value.
```yaml
info:
 seed: 12345
```
When a seed is specified, the same YAML configuration will always generate identical data across multiple runs.
This is useful for testing, debugging, or when you need consistent datasets.

If no seed is provided, the data generation will use random values and produce different results each time.

Example with seed:
```yaml
info:
 output_name: "my_data"
 output_format: csv
 rows: 1000
 seed: 42

columns:
 - name: id
   provider: Increment.integer
   start: 1
 - name: score
   provider: Random.i32
   min: 0
   max: 100
```
Running this configuration multiple times will always produce the same 1000 rows with identical scores.