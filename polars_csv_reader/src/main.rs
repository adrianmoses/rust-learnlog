use polars::prelude::*;

fn main() {
    let df_csv = CsvReader::from_path("data/pima-indians-diabetes.csv")
        .unwrap()
        .infer_schema(None)
        .has_header(false)
        .finish()
        .unwrap();
    println!("{}", df_csv);
}
