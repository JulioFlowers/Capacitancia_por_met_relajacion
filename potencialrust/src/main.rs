use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};

const MAX_ITER: usize = 1000;

fn obtener_valores(matriz: &mut Vec<Vec<f32>>, valor_buscado: f32) -> Vec<(usize, usize)> {
    let mut indices_a_cambiar = Vec::new();

    for (i, fila) in matriz.iter_mut().enumerate() {
        for (j, valor) in fila.iter_mut().enumerate() {
            if *valor == valor_buscado {
                // Guardar los índices
                indices_a_cambiar.push((i, j));
            }
        }
    }

    indices_a_cambiar
}


fn verificar_limites(matriz: &Vec<Vec<f32>>, i: usize, j: usize) -> bool {
    let filas = matriz.len();

    if filas == 0 {
        return false; // Matriz vacía
    }

    let columnas = matriz[0].len();

    let i_valido = i < filas;
    let j_valido = j < columnas;

    let i_menos_dos_valido = i >= 2;
    let i_menos_uno_valido = i >= 1;
    let i_mas_uno_valido = i + 1 < filas;
    let i_mas_dos_valido = i + 2 < filas;

    let j_menos_dos_valido = j >= 2;
    let j_menos_uno_valido = j >= 1;
    let j_mas_uno_valido = j + 1 < columnas;
    let j_mas_dos_valido = j + 2 < columnas;

    i_valido && j_valido &&
        i_menos_dos_valido && i_menos_uno_valido &&
        i_mas_uno_valido && i_mas_dos_valido &&
        j_menos_dos_valido && j_menos_uno_valido &&
        j_mas_uno_valido && j_mas_dos_valido
}

fn sumar_terminos(matriz: &Vec<Vec<f32>>, i: usize, j: usize) -> Option<f32> {
    if verificar_limites(matriz, i, j) {
        let resultado =  
            - matriz[i - 2][j] 
            - matriz[i][j - 2] 
            + 16.0 * matriz[i - 1][j] 
            + 16.0 * matriz[i][j - 1] 
            +16.0 * matriz[i + 1][j] 
            + 16.0 * matriz[i][j + 1] 
            - 16.0 * matriz[i + 2][j] 
            - matriz[i][j + 2];

        Some(resultado)
    } else {
        None  // Índices fuera de los límites
    }
}

fn write_matrix_to_file_parallel(matrix: &mut Vec<Vec<f32>>, file_path: &str) -> io::Result<()> {
    let formatted_rows: Vec<String> = matrix
        .par_iter_mut()
        .map(|row| {
            // Formatear la fila como una cadena separada por tabulaciones
            row.iter_mut()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join("\t")
        })
        .collect();

    let mut file = File::create(file_path)?;

    for row_str in formatted_rows {
        // Escribir la fila en el archivo seguido de un salto de línea
        writeln!(file, "{}", row_str)?;
    }

    Ok(())
}

fn main() /*-> io::Result<()> */
{
    // Solicitar al usuario el número de filas y columnas
    let m = 2046;
    let n = 2430;

    // Crear la matriz
    let mut phi: Vec<Vec<f32>> = crear_matriz(m, n);

    //potencial positivo
    for i in 2..m - 2 {
        if (0..=194).contains(&i) {
            for j in 0..n {
                phi[i][j] = 3.5;
            }
        }

        if (194..=704).contains(&i) {
            for range in [(0..385), (1021..1404), (2040..n)] {
                for j in range {
                    phi[i][j] = 3.5;
                }
            }
        }

        if (704..=1340).contains(&i) {
            for range in [(0..385), (2040..n)] {
                for j in range {
                    phi[i][j] = 3.5;
                }
            }
        }

        if (1340..=1723).contains(&i) {
            for range in [(0..898), (1535..n)] {
                for j in range {
                    phi[i][j] = 3.5;
                }
            }
        }
    }

    //potencial 0
    for i in 0..m {
        if (322..=832).contains(&i) {
            for range in [(512..894), (1531..1913)] {
                for j in range {
                    phi[i][j] = 0.0;
                }
            }
        }

        if (832..=1214).contains(&i) {
            for j in 512..1913 {
                phi[i][j] = 0.0;
            }
        }

        if (1214..=1851).contains(&i) {
            for j in 1022..1405 {
                phi[i][j] = 0.0;
            }
        }

        if i >= 1851 {
            for j in 0..n {
                phi[i][j] = 0.0;
            }
        }
    }

    //obtener indices a iterar
    let indices = obtener_valores(&mut phi, 1.75);

    for (i, j) in indices.clone() {
        // Verificar que los índices estén dentro del rango de la matriz
        if i < phi.len() && j < phi[0].len() {

            match sumar_terminos(&phi, i, j) {
                Some(resultado) => phi[i][j] = resultado,
                None => println!("Índices fuera de los límites o matriz vacía."),
            }
            
        } else {
            println!("Índices ({}, {}) fuera de rango", i, j);
        }
    }

    // Llamar a la función para escribir en el archivo de manera paralela
    if let Err(e) = write_matrix_to_file_parallel(&mut phi, "output.txt") {
        eprintln!("Error al escribir en el archivo: {}", e);
    } else {
        println!("Datos escritos exitosamente en el archivo.");
    }
}

// Función para crear una matriz de m filas por n columnas
fn crear_matriz(m: usize, n: usize) -> Vec<Vec<f32>> {
    let mut matriz = Vec::with_capacity(m);

    for _ in 0..m {
        let fila: Vec<f32> = vec![1.75; n];
        matriz.push(fila);
    }

    matriz
}
