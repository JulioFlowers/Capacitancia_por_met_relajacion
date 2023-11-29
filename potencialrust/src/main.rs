use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};

const MAX_ITER: usize = 10000;

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

    for _k in 0..MAX_ITER {
        for (i, j) in indices.clone() {
            // Verificar que los índices estén dentro del rango de la matriz
            if i < phi.len() && j < phi[0].len() {
                if i >= 2 && j >= 2 && i + 2 < phi.len() && j + 2 < phi[0].len() {
                    phi[i][j] = (16.0 * phi[i - 1][j]
                        + 16.0 * phi[i][j - 1]
                        + 16.0 * phi[i + 1][j]
                        + 16.0 * phi[i][j + 1]
                        - 1.0 * phi[i - 2][j]
                        - 1.0 * phi[i][j - 2]
                        - 1.0 * phi[i + 2][j]
                        - 1.0 * phi[i][j + 2])
                        / 60.0;
                }
            } else {
                println!("Índices ({}, {}) fuera de rango", i, j);
            }
        }

        println!("Iteración n°: {}.", _k);
    }

    let mut ex: Vec<Vec<f32>> = crear_matriz(m - 4, n - 4); // Componente x del campo eléctrico
    let mut ey: Vec<Vec<f32>> = crear_matriz(m - 4, n - 4); // Componente y del campo eléctrico

    for i in 0..m - 4 {
        let ip = i + 2;
        for j in 0..n - 4 {
            let jp = j + 2;
            ey[i][j] = -1.0
                * (-1.0 * phi[ip + 2][jp] + 16.0 * phi[ip + 1][jp] - 30.0 * phi[ip][jp]
                    + 16.0 * phi[ip - 1][jp]
                    - 1.0 * phi[ip - 2][jp])
                / 12.0;
            ex[i][j] = -1.0
                * (-1.0 * phi[ip][jp + 2] + 16.0 * phi[ip][jp + 1] - 30.0 * phi[ip][jp]
                    + 16.0 * phi[ip][jp - 1]
                    - 1.0 * phi[i][jp - 2])
                / 12.0;
        }
    }

    let mut rho: Vec<Vec<f32>> = crear_matriz(m - 6, n - 6);

    for i in 0..m - 6 {
        for j in 0..n - 6 {
            // Asegúrate de que los índices estén dentro de los límites antes de acceder a rho
            if i + 1 < rho.len() && i >= 1 && j + 1 < rho[0].len() && j >= 1 {
                rho[i][j] = (ey[i + 1][j] - ey[i - 1][j]) + (ex[i][j + 1] - ex[i][j - 1]);
            } 
        }
    }
    

    // Llamar a la función para escribir en el archivo de manera paralela
    if let Err(e) = write_matrix_to_file_parallel(&mut phi, "output.txt") {
        eprintln!("Error al escribir en el archivo: {}", e);
    } else {
        println!("Datos escritos exitosamente en el archivo.");
    }

    if let Err(e) = write_matrix_to_file_parallel(&mut ex, "ex.txt") {
        eprintln!("Error al escribir en el archivo: {}", e);
    } else {
        println!("Datos escritos exitosamente en el archivo.");
    }

    if let Err(e) = write_matrix_to_file_parallel(&mut ey, "ey.txt") {
        eprintln!("Error al escribir en el archivo: {}", e);
    } else {
        println!("Datos escritos exitosamente en el archivo.");
    }

    if let Err(e) = write_matrix_to_file_parallel(&mut rho, "carga.txt") {
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
