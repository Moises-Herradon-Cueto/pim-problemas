use std::fmt::Display;

#[allow(clippy::too_many_arguments)]
pub fn into_template<T1, T2, T3, T4, T5, T6, T7, T8, T9>(
    paquetes: &T1,
    temas: &T5,
    dificultad: &T2,
    historial: &T3,
    curso: &T4,
    fuente: &T6,
    comentarios: &T7,
    id: &T8,
    document: &T9,
) -> String
where
    T1: Display,
    T2: Display,
    T3: Display,
    T4: Display,
    T5: Display,
    T6: Display,
    T7: Display,
    T8: Display,
    T9: Display,
{
    format!(
        "% !TeX encoding = UTF-8



%%% PLANTILLA PARA SUBIR EJERCICIOS A LA BASE DE DATOS DEL PIM
\\documentclass[12pt,a4paper]{{article}}
\\usepackage[utf8]{{inputenc}}
\\usepackage{{pim}}

% Si necesitas más paquetes, añádelos debajo de la siguiente línea
%%% Paquetes extra
{paquetes}
%%% Fin de paquetes extra


% Introduce los temas separados por comas
% Por ejemplo
% \\temas{{
% Inducción, Numeritos
% }}
\\temas{{
{temas}
}}

% Dificultad del 1 al 10
% \\dificultad{{
% 10
% }}
\\dificultad{{
{dificultad}
}}

% De dónde viene el problema
% \\fuente{{
% Aritmética de Diofanto, capítulo 1.
% }}
\\fuente{{
{fuente}
}}

% Curso a partir del cual se puede poner el problema
% Opciones:
% 1Primaria, 2Primaria ... 6Primaria
% 1ESO, 2ESO, 3ESO, 4ESO
% 1Bach, 2Bach
% \\curso{{
% 1ESO
% }}
\\curso{{
{curso}
}}

% Descomentar para restringir el acceso:
%\\acceso{{
%Sí
%}}

% Comentarios, separados por comas
% \\comentarios{{
% Un problema muy fácil, les salió a todos
% }}
\\comentarios{{
{comentarios}
}}

\\id{{
{id}
}}

\\historial{{
{historial}
}}

\\begin{{document}}
{document}
\\end{{document}}"
    )
}
