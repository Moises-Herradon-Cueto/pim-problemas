use parse_lib::{parse_all_files, read_csv, run_latex::run_latex, write_json};

fn main() {
    pretty_env_logger::init();
    let mut data = read_csv().expect("oops");
    parse_all_files(&mut data).expect("oops");
    write_json(&data).expect("oops");
    run_latex();
}
