use parse_lib::{
    data::{read_csv, write_html, write_json},
    files::parse_all,
    pdflatex,
};

fn main() {
    pretty_env_logger::init();
    let mut data = read_csv();
    parse_all(&mut data).expect("oops");
    write_json(&data).expect("oops");
    write_html(&data);
    // pdflatex::run();
}
