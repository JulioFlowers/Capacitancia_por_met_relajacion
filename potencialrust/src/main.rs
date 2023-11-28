
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};

const MAX_ITER: usize = 1000;

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

fn cpotencial(matriz: &Vec<Vec<f32>>, i: usize, j: usize) -> f32 {
    // Verificar que las filas y columnas necesarias estén presentes
    let i_plus_1 = if i < matriz.len() - 1 {
        i + 1
    } else {
        matriz.len() - 1
    };
    let i_minus_1 = if i >= 1 { i - 1 } else { 0 };
    let i_minus_2 = if i >= 2 { i - 2 } else { 0 }; // Agregado para evitar desbordamiento
    let j_plus_1 = if j < matriz[0].len() - 1 {
        j + 1
    } else {
        matriz[0].len() - 1
    };
    let j_minus_1 = if j >= 1 { j - 1 } else { 0 };

    let potencial = (-matriz[i + 2][j] - matriz[i][j + 2]
        + 16.0 * matriz[i + 1][j]
        + 16.0 * matriz[i][j + 1]
        + 16.0 * matriz[i_minus_1][j]
        + 16.0 * matriz[i][j_minus_1]
        - matriz[i_minus_1][j_minus_1]
        - matriz[i_minus_2][j])
        / 60.0;

    potencial
}

fn main() /*-> io::Result<()> */
{
    // Solicitar al usuario el número de filas y columnas
    let m = 2046;
    let n = 2430;

    // Crear la matriz
    let mut phi: Vec<Vec<f32>> = crear_matriz(m, n);

    //potencial positivo
    for i in 0..m {
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
    
/* 
    for _k in 0..MAX_ITER {
        for i in 0..m {
            if i <= 195 && i <= 321 {
                for j in 386..1020 {
                    matriz[i][j] = cpotencial(&matriz, i, j);
                }

                for j in 1405..2039 {
                    matriz[i][j] = cpotencial(&matriz, i, j);
                }
            }

            if i <= 321 && i <= 705 {
                for j in 386..512 {
                    matriz[i][j] = cpotencial(&matriz, i, j);
                }

                for j in 896..1022 {
                    matriz[i][j] = cpotencial(&matriz, i, j);
                }

                for j in 1407..1532 {
                    matriz[i][j] = cpotencial(&matriz, i, j);
                }

                for j in 1916..2041 {
                    matriz[i][j] = cpotencial(&matriz, i, j);
                }
            }
        }
    }
    */

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
