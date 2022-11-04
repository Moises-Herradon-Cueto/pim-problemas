use parse_lib::{
    data::{read_csv, write_json},
    files::parse_all,
    pdflatex,
};

fn main() {
    pretty_env_logger::init();
    let mut data = read_csv().expect("oops");
    parse_all(&mut data).expect("oops");
    write_json(&data).expect("oops");
    pdflatex::run();
}
