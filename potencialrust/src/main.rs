use std::fs::File;
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
