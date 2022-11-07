use std::{
    collections::{HashMap, HashSet},
    fs,
};

use parse_lib::{
    data::{read_csv, write_html, write_json, Data},
    files::parse_all,
};

fn main() {
    gather_info_copy_files();
    // make_problem_list();
    // pdflatex::run();
}

fn gather_info_copy_files() {
    let mut data = read_csv();
    parse_all(&mut data).expect("oops");
    write_json(&data).expect("oops");
    write_html(&data);
}

fn make_problem_list(data: &HashMap<usize, Data>) {
    let mut packages = HashSet::new();
    let problems: String = (2200070..2200130_usize)
        .filter_map(|i| {
            let problem_info = data.get(&i)?;
            let problem_statement = &problem_info.enunciado;
            let id = problem_info.id;
            packages.extend(problem_info.paquetes.iter());
            Some(format!(
                "\\begin{{ejer}}\n% Problema {id}\n\n{problem_statement}\n\\end{{ejer}}\n\n\n"
            ))
        })
        .collect();
    let packages: String = packages.into_iter().cloned().collect();
    fs::write("problemas_juntos.tex", problems).expect("Failed to write");
    fs::write("paquetes_juntos.tex", packages).expect("Failed to write");
}
