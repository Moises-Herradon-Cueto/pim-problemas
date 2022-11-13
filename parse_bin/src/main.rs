use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use parse_lib::{
    get_json_string, pdflatex, read_csv, table_friendly::TableFriendly, write_csv, Data,
};

fn main() {
    read_json_write_csv();
    // make_problem_list();
    // pdflatex::run();
}

fn read_json_write_csv() {
    let data_json =
        get_json_string("/home/moises/pim-input/database.json").expect("Failed to open json");
    let data_json: HashMap<usize, Data> =
        serde_json::from_str(&data_json).expect("Failed to deserialize");
    let csv_friendly: Vec<TableFriendly> =
        data_json.into_values().map(|data| data.into()).collect();
    write_csv(&csv_friendly, "datos-modified.csv");
}

fn compare_csv_json() {
    let data_csv = read_csv(Path::new("Datos.csv")).0;
    let data_json =
        get_json_string("/home/moises/pim-input/database.json").expect("Failed to open json");
    let mut data_json: HashMap<usize, Data> =
        serde_json::from_str(&data_json).expect("Failed to deserialize");
    let json_len = data_json.len();

    let mut count = 0_usize;
    let mut count_errors = 0_usize;

    for (id, data) in &mut data_json {
        let Some(other_data) = data_csv.get(id) else {
            continue;
        };
        if other_data.has_more_data_than(data).is_some() {
            let result = data.merge_with(other_data);
            for err in result {
                println!("In problem {id}\n{err}");
                println!("Found more stuff.\nJson data:\n{data:#?}\nCsv data\n{other_data:#?}");
                count_errors += 1;
            }
            count += 1;
        }
        data.paquetes.sort();
        data.paquetes.dedup();
        let no_empty = data.paquetes.iter().filter(|x| !x.is_empty()).cloned();
        data.paquetes = no_empty.collect();
    }
    let data_json = serde_json::to_string(&data_json).expect("Failed to serialize");

    fs::write("/home/moises/pim-input/database-merged.json", data_json).expect("Failed to write");

    println!(
        "Errors: {count_errors}\nCount: {count}\ncsv: {}\njson: {}",
        data_csv.len(),
        json_len
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
