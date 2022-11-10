use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use parse_lib::{get_json_string, read_csv, Data};

fn main() {
    gather_info_copy_files();
    // make_problem_list();
    // pdflatex::run();
}

fn gather_info_copy_files() {
    let data_csv = read_csv(Path::new("Datos.csv")).0;
    let data_json =
        get_json_string("/home/moises/pim-input/database.json").expect("Failed to open json");
    let mut data_json: HashMap<usize, Data> =
        serde_json::from_str(&data_json).expect("Failed to deserialize");

    let mut count = 0_usize;

    for (id, data) in &mut data_json {
        let Some(other_data) = data_csv.get(id) else {
            continue;
        };
        if other_data.has_more_data_than(data).is_some() {
            println!("Found more stuff.\nJson data:\n{data:#?}\nCsv data\n{other_data:#?}");
            count += 1;
        }
    }

    println!(
        "Count: {count}\ncsv: {}\njson: {}",
        data_csv.len(),
        data_json.len()
    );
    // parse_all(&mut data).expect("oops");
    // write_json(&data).expect("oops");
    // write_html(&data);
}

fn _make_problem_list(data: &HashMap<usize, Data>) {
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
