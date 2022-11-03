use parse_lib::parse_file;

fn main() {
    let result = parse_file("ejercicios-in/220001.tex").unwrap();
    println!("{result}");
}
