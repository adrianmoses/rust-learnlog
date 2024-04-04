use polars::prelude::*;

fn normalize_dataset(df: DataFrame) -> Vec<(Option<i32>, Option<i32>)> {
    let mut min_max = vec![];
    for col in df.get_columns() {
        let min = col.min().unwrap();
        let max = col.max().unwrap();
        min_max.push((min, max));
    }
    min_max
}

fn read_csv() -> DataFrame {
    CsvReader::from_path("data/pima-indians-diabetes.csv")?
        .infer_schema(None)
        .has_header(false)
        .finish()?
}

fn main() {
    let df_csv = read_csv();
    let min_max = normalize_dataset(df_csv);
    println!("{:?}", min_max);
}
