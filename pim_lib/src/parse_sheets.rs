use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};

use regex::Regex;
use tracing::{info, instrument, trace, warn};

use crate::files::decode_file;

/// .
///
/// # Panics
///
/// Panics if anything goes wrong
pub fn directory(path: &Path, output_path: &Path) {
    let mut map = HashMap::new();
    make_hashmap(path, &mut map);
    let names = fs::read_to_string(Path::new("names.json")).unwrap_or_default();
    let mut names: HashMap<String, String> = serde_json::from_str(&names).unwrap_or_default();
    let file = fs::File::create(output_path).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    writer.write_all(b"START TRANSACTION;\n").unwrap();
    into_sql(map, &mut writer, &mut names);
    writer.write_all(b"COMMIT;\n").unwrap();
    writer.flush().unwrap();
    let names = serde_json::to_string(&names).unwrap();
    fs::write("names.json", names).unwrap();
}
fn get_name<'a>(
    file_name: &'a str,
    names: &'a mut HashMap<String, String>,
) -> (String, Option<&'static str>) {
    let lower_case = file_name.to_lowercase();
    let planet = if lower_case.contains("mercurio") {
        Some("Mercurio")
    } else if lower_case.contains("venus") {
        Some("Venus")
    } else if lower_case.contains("jupiter") {
        Some("Jupiter")
    } else if lower_case.contains("urano") {
        Some("Urano")
    } else {
        None
    };
    if let Some(name) = names.get(file_name) {
        return (name.clone(), planet);
    }
    println!("¿Qué titulo dar a la hoja {file_name}?");
    planet.map_or_else(
        || {
            println!("Es de todos los grupos.");
        },
        |p| {
            println!("Pertenece a {p}.");
        },
    );
    let title = {
        io::stdout().flush().expect("Error flushing");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading");
        user_input.pop();
        user_input
    };
    names.insert(file_name.to_owned(), title);
    (names.get(file_name).unwrap().clone(), planet)
}

fn into_sql(
    map: HashMap<String, HashSet<usize>>,
    file: &mut BufWriter<File>,
    names: &mut HashMap<String, String>,
) {
    let mut sheets = HashSet::new();
    for (file_name, problems) in map {
        if problems.is_empty() {
            continue;
        }
        let (title, planet) = get_name(&file_name, names);
        trace!(file_name, title, ?planet);
        assert!(
            sheets.insert((title.clone(), planet)),
            "Hay dos hojas {title} de {planet:?}"
        );
        into_sql_sheet(&title, planet, problems, file);
    }
}

fn into_sql_sheet(
    title: &str,
    planet: Option<&str>,
    problems: HashSet<usize>,
    file: &mut BufWriter<File>,
) {
    let insert = format!(
        "INSERT INTO pim_hojas (Titulo, {}Curso) VALUES ('{title}', {}2022);\n",
        planet.map_or("", |_| "Grupo, "),
        planet.map_or_else(String::new, |p| format!("'{p}', "))
    );
    file.write_all(insert.as_bytes()).unwrap();
    for problem in problems {
        let cross = format!(
            "INSERT IGNORE INTO pim_problemas_hojas (ID_Problema, ID_Hoja)
SELECT pim_problemas.ID,hojas.ID
FROM pim_problemas
    CROSS JOIN (
        SELECT ID
        FROM pim_hojas
        WHERE Titulo = '{title}' {}
    ) as hojas
    WHERE pim_problemas.titulo = '{problem}';\n",
            planet.map_or_else(String::new, |p| format!("AND Grupo = '{p}'"))
        );
        file.write_all(cross.as_bytes()).unwrap();
    }
}

fn make_hashmap(path: &Path, map: &mut HashMap<String, HashSet<usize>>) {
    let entries = fs::read_dir(path).expect("Failed to read directory");
    for entry in entries {
        let entry = entry.expect("Error reading");
        let file_type = entry.file_type().expect("Couldn't get file type");
        if file_type.is_dir() {
            make_hashmap(&entry.path(), map);
            continue;
        }
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if !file_name.ends_with(".tex")
            || file_name.ends_with("oluciones.tex")
            || file_name.starts_with("Plantilla")
        {
            continue;
        }
        map.insert(file_name.to_string(), find_problems(&entry.path()));
        if map.get(file_name.as_ref()).unwrap().is_empty() {
            info!(?file_name, "No problems found");
        }
    }
}

#[instrument]
fn find_problems(path: &Path) -> HashSet<usize> {
    let contents = decode_file(path).expect("Failed to decode");
    let regex = Regex::new(r"%\s*(\d+)").expect("Failed to make regex");
    regex
        .captures_iter(&contents)
        .map(|c| {
            c.get(1)
                .expect("Have capture group")
                .as_str()
                .parse()
                .expect("Failed to parse")
        })
        .collect()
}
