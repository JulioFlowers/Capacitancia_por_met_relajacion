/*use std::fs::File;
use std::io::{self, Write};

const N: usize = 2000;
const M: usize = 2500;

fn main() -> io::Result<()> {
    // Crear una matriz de N x M en 0
    let mut matriz = vec![vec![0.0; M]; N];

    //crea la L
    for i in 0..1530 {
        for j in 0..383{
            matriz[i][j] = 3.5;
        }
    }

    for i in 1147..1530 {
        for j in 383..893{
            matriz[i][j] = 3.5;
        }
    }

    // Abrir un archivo para escritura
    let nombre_archivo = "mi_archivo.txt";
    let mut archivo = File::create(nombre_archivo)?;

    // Escribir los elementos de la matriz en el archivo
    for fila in &matriz {
        for &elemento in fila {
            write!(archivo, "{:.2} ", elemento)?;
        }
        writeln!(archivo)?;
    }

    println!("Matriz exportada exitosamente en {}", nombre_archivo);

    Ok(())
}

use std::fs::File;
use std::io::{self, Write};

const N: usize = 1890;
const M: usize = 2470;
const MAX_ITER: usize = 1000;
const TOL: f64 = 1e-6;

// Función para guardar un array bidimensional en un archivo
fn guardar_array_en_archivo(filas: usize, columnas: usize, array: &[[f64; M]], nombre_archivo: &str) -> io::Result<()> {
    let mut archivo = File::create(nombre_archivo)?;

    // Escribir los elementos del array
    for i in 0..filas {
        for j in 0..columnas {
            write!(archivo, "{} ", array[i][j])?;
        }
        writeln!(archivo)?;
    }

    Ok(())
}

fn main() {
    let mut phi = [[0.0; M]; N];
    let cuad = 390;
    let esp = 130;

    // Condiciones de contorno
    // ... (código sin cambios)

    // cálculo del potencial
    for _k in 0..MAX_ITER {
        for i in 0..N {
            for j in 0..M {
                if (i <= esp || (i > esp && i <= N - 2 * esp && j <= cuad)
                    || (i > esp && i <= N - 2 * esp && j > M - cuad)
                    || (i > N - cuad - 2 * esp && i <= N - 2 * esp && j <= 2 * cuad + esp)
                    || (i > N - cuad - 2 * esp && i <= N - 2 * esp && j > M - 2 * cuad - esp)
                    || (i <= cuad + 2 * esp && j > 2 * (cuad + esp) && j <= M - 2 * (cuad + esp)))
                {
                    phi[i][j] = 3.2; // Voltajes positivos
                } else if (i > 2 * esp && i <= N - (3 * esp + cuad)
                    && j > (cuad + esp) && j <= (2 * cuad + esp))
                    || (i > 2 * esp && i <= N - (3 * esp + cuad)
                        && j <= M - (cuad + esp) && j > M - (2 * cuad + esp))
                    || (i > 3 * esp + cuad && i <= N - 3 * esp - cuad
                        && j > cuad + esp && j <= M - cuad - esp)
                    || (i > N - 3 * esp - cuad && j > 2 * (esp + cuad)
                        && j <= M - 2 * (esp + cuad))
                    || (i >= N - esp)
                {
                    phi[i][j] = 0.0; // Voltajes negativos
                } else {
                    let i_plus_1 = i.wrapping_add(1);
                    let i_minus_1 = i.wrapping_sub(1);
                    let j_plus_1 = j.wrapping_add(1);
                    let j_minus_1 = j.wrapping_sub(1);

                    phi[i][j] = 0.25 * (phi.get(i_plus_1).and_then(|row| row.get(j))
                        .unwrap_or(&0.0)
                        + phi.get(i_minus_1).and_then(|row| row.get(j))
                            .unwrap_or(&0.0)
                        + phi[i].get(j_plus_1).copied().unwrap_or(0.0)
                        + phi[i].get(j_minus_1).copied().unwrap_or(0.0));
                }
            }
        }
    }

    if let Err(err) = guardar_array_en_archivo(N, M, &phi, "miArchivo.txt") {
        eprintln!("Error al guardar en el archivo: {}", err);
    }
}
*/
use std::fs::File;
use std::io::{self, Write};
use rayon::prelude::*;

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

fn main() /*-> io::Result<()> */{
    // Solicitar al usuario el número de filas y columnas
    let m = 2046;
    let n = 2430;

    // Crear la matriz
    let mut matriz: Vec<Vec<f32>> = crear_matriz(m, n);


    //potencial positivo
    for i in 0..m {

        if i<=194{

            for j  in 0..n{
                matriz[i][j]= 3.5;
            }
        }

        if 194<=i && i<=704{

            for j  in 0..385{
                matriz[i][j]= 3.5;
            }

            for j  in 1021..1404{
                matriz[i][j]= 3.5;
            }

            for j  in 2040..n{
                matriz[i][j]= 3.5;
            }
        }

        if 704<=i && i<= 1340{
            for j  in 0..385{
                matriz[i][j]= 3.5;
            }

            for j  in 2040..n{
                matriz[i][j]= 3.5;
            }
        }

        if 1340<=i && i<= 1723{
            for j  in 0..898{
                matriz[i][j]= 3.5;
            }

            for j  in 1535..n{
                matriz[i][j]= 3.5;
            }
        }
    }

    //potencial 0

    for i in 0..m{

        if 322<=i && i<=832{

            for j in 512..894{
                matriz[i][j]= 0.0;
            }

            for j in 1531..1913{
                matriz[i][j]= 0.0;
            }
        }

        if 832<=i && i<=1214{

            for j in 512..1913{
                matriz[i][j]= 0.0;
            }
        }

        if 1214<=i && i<=1851{

            for j in 1022..1405{
                matriz[i][j]= 0.0;
            }
        }

        if 1851<=i {

            for j in 0..n{
                matriz[i][j]= 0.0;
            }
        }

    }

    /*let nombre_archivo = "mi_archivo.txt";
    let mut archivo = File::create(nombre_archivo)?;

    // Escribir los elementos de la matriz en el archivo
    for fila in &matriz {
        for &elemento in fila {
            write!(archivo, "{:.2} ", elemento)?;
        }
        writeln!(archivo)?;
    }

    println!("Matriz exportada exitosamente en {}", nombre_archivo);

    Ok(())*/

    // Llamar a la función para escribir en el archivo de manera paralela
    if let Err(e) = write_matrix_to_file_parallel(&mut matriz, "output.txt") {
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

// Función para imprimir una matriz
fn imprimir_matriz(matriz: &Vec<Vec<f32>>) {
    println!("Matriz:");

    for fila in matriz {
        for &elemento in fila {
            print!("{} ", elemento);
        }
        println!();
    }
}
