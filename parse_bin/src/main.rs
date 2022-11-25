use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use arguments::Action;
use clap::Parser;
use parse_lib::{
    clean_packages, commands::sync_db, get_json_string, make_problem_sheet, pdflatex, read_csv,
    table_friendly::TableFriendly, write_csv, Data, OldData,
};

use crate::arguments::MyArgs;

mod arguments;

fn main() {
    let cli = MyArgs::parse();
    // let cli = MyArgs {
    //     command: Action::SyncDb {
    //         output_dir: std::path::PathBuf::from("/home/moises/pim-input/ejercicios-out"),
    //         problems_dir: std::path::PathBuf::from("/home/moises/OneDrive/ejercicios"),
    //         database_dir: None,
    //     },
    // };
    match cli.command {
        Action::WriteCsv {
            database_dir,
            csv_dir,
        } => read_json_write_csv(&database_dir, &csv_dir),
        Action::SyncDb {
            output_dir,
            problems_dir,
            database_dir,
            database_dir_out,
        } => {
            let database_dir = database_dir.unwrap_or_else(|| problems_dir.join("database.json"));
            let result = sync_db(
                &database_dir,
                database_dir_out.as_deref(),
                &problems_dir,
                &output_dir,
            );
            println!("{result:#?}");
        }
        Action::CompareCsvJson {
            merged_path,
            database_dir,
        } => compare_csv_json(&database_dir, &merged_path),
        Action::Latex { output_dir } => {
            let result = pdflatex::run(output_dir);
            println!("{result:?}");
        }
        Action::Migrate {
            database_dir,
            new_database_dir,
        } => migrate(&database_dir, &new_database_dir),
        Action::MakeProblemList {
            database_path,
            start,
            end,
            output,
        } => make_problem_list(&database_path, start, end, &output),
        Action::CleanPackages {
            database_path,
            output_path,
        } => clean_packages_db(&database_path, output_path.as_deref()),
        Action::MakeProblemSheet {
            input_path,
            problems_path,
            output_no_solutions,
            output_with_solutions,
        } => {
            let output = make_problem_sheet(
                &input_path,
                &problems_path,
                &output_no_solutions,
                &output_with_solutions,
            );
            println!("{output:#?}");
        }
    }
}

fn clean_packages_db(database_path: &Path, output: Option<&Path>) {
    let output = output.unwrap_or(database_path);
    let data_json = get_json_string(database_path).expect("Failed to open json");
    let mut data_json: HashMap<usize, Data> =
        serde_json::from_str(&data_json).expect("Failed to deserialize");
    data_json.values_mut().for_each(clean_packages);
    let data_json = serde_json::to_string(&data_json).expect("Failed to serialize");
    fs::write(output, data_json).expect("Failed to write");
}

fn read_json_write_csv(database_dir: &Path, csv_dir: &Path) {
    let data_json = get_json_string(database_dir).expect("Failed to open json");
    let data_json: HashMap<usize, Data> =
        serde_json::from_str(&data_json).expect("Failed to deserialize");
    let csv_friendly: Vec<TableFriendly> =
        data_json.into_values().map(|data| data.into()).collect();
    write_csv(&csv_friendly, csv_dir);
}

fn compare_csv_json(database_dir: &Path, merged_path: &Path) {
    let data_csv = read_csv(database_dir).0;
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

    fs::write(merged_path, data_json).expect("Failed to write");

    println!(
        "Errors: {count_errors}\nCount: {count}\ncsv: {}\njson: {}",
        data_csv.len(),
        json_len
    );
    // parse_all(&mut data).expect("oops");
    // write_json(&data).expect("oops");
    // write_html(&data);
}

fn make_problem_list(database_path: &Path, start: usize, end: usize, output: &Path) {
    let data_json = get_json_string(database_path).expect("Failed to open json");
    let data: HashMap<usize, Data> =
        serde_json::from_str(&data_json).expect("Failed to deserialize");
    let mut packages = HashSet::new();
    let mut id_difficulty: Vec<(usize, u8, String)> = (start..=end)
        .filter_map(|i| {
            let problem_info = data.get(&i)?;
            packages.extend(problem_info.paquetes.iter());
            Some((i, problem_info.dificultad, problem_info.enunciado.clone()))
        })
        .collect();
    id_difficulty.sort_by(|(_, d1, _), (_, d2, _)| d1.cmp(d2));
    let problems: String = id_difficulty
        .into_iter()
        .map(|(id, diff, problem_statement)| {
            format!(
                "\\begin{{ejer}}\n% Problema {id}\n% Dificultad: {diff}\n\n{problem_statement}\n\\end{{ejer}}\n\n"
            )
        })
        .collect();
    let packages: String = packages.into_iter().map(|p| format!("{p}\n")).collect();
    let all = format!("%%% Paquetes\n\n{packages}\n\n%%%%%%\n\n%%%%% Problemas\n\n{problems}");
    fs::write(output, all).expect("Failed to write");
}

fn migrate(old: &Path, new: &Path) {
    let data_json = get_json_string(old).expect("Failed to open json");
    let old_data: HashMap<usize, OldData> =
        serde_json::from_str(&data_json).expect("Failed to deserialize");
    let new_data: HashMap<usize, Data> = old_data
        .into_iter()
        .map(|(i, problem)| (i, Data::from(problem)))
        .collect();
    let data_json = serde_json::to_string(&new_data).expect("Failed to serialize");

    fs::write(new, data_json).expect("Failed to write");
}
