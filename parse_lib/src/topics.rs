use std::collections::{HashMap, HashSet};

use crate::Data;

#[must_use]
pub fn get<T>(database: &HashMap<usize, Data, T>) -> Vec<String> {
    let topics: HashSet<_> = database
        .values()
        .flat_map(|problem| problem.temas.iter())
        .collect();
    let mut output: Vec<_> = topics.into_iter().cloned().collect();
    output.sort();
    output
}

#[must_use]
pub fn into_php(topics: &[String]) -> String {
    topics
        .iter()
        .enumerate()
        .map(|(i, topic)| {
            format!("array_push($topics, array('ID' => '{i}', 'Titulo' => '{topic}'));\n")
        })
        .collect()
}
