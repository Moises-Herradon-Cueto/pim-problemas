use std::collections::HashMap;

use crate::{Data, Fields};

pub const PREAMBLE: &str = r"<!DOCTYPE html>
<html lang='en'>

<head>
	<meta charset='UTF-8'>
	<meta http-equiv='X-UA-Compatible' content='IE=edge'>
	<meta name='viewport' content='width=device-width, initial-scale=1.0'>
	<link rel='stylesheet' href='styles.css'>
	<script type='text/javascript' src='table.js'></script>
    <script src='https://polyfill.io/v3/polyfill.min.js?features=es6'></script>
    <script id='MathJax-script' async src='https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js'></script>
	<title>Información</title>
</head>

<body>
    ";

pub const POSTAMBLE: &str = r"    </table>
</body>

</html>
";

#[must_use]
pub fn make<K>(data: &HashMap<usize, Data, K>) -> String {
    let mut vec: Vec<_> = data.values().collect();
    vec.sort_by(|a, b| a.id.cmp(&b.id));
    let rows: String = vec
        .into_iter()
        .map(|data| {
            let entries: String = Fields::ALL
                .into_iter()
                .map(|f| {
                    if matches!(f, Fields::Problem) {
                        format!(
                            "<td><p class='problem-preview'>{}</p></td>",
                            f.get_string(data)
                        )
                    } else {
                        format!("<td>{}</td>", f.get_string(data))
                    }
                })
                .collect();
            format!("<tr>{entries}</tr>\n")
        })
        .collect();

    let headers: String = Fields::ALL
        .into_iter()
        .enumerate()
        .map(|(i, f)| format!("<th onclick=sortTable({i})><span></span>{f}</th>"))
        .collect();

    let buttons: String = Fields::ALL
        .into_iter()
        .enumerate()
        .map(|(i, f)| format!("<button onclick=toggle({i})><span>Ocultar</span> {f}</button>"))
        .collect();

    format!("{PREAMBLE}<div class='button-container'>{buttons}</div>\n<table>\n<thead><tr>{headers}</tr></thead>\n{rows}{POSTAMBLE}")
}
