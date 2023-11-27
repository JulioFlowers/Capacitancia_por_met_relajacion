#include <stdio.h>
#include <math.h>

#define N 1800
#define M 2470
#define MAX_ITER 15000
#define TOL 1e-6

// Función para guardar un array bidimensional en un archivo
void guardarArrayEnArchivo(int filas, int columnas, double array[filas][columnas], const char *nombreArchivo) {
    FILE *archivo = fopen(nombreArchivo, "w");

    if (archivo == NULL) {
        fprintf(stderr, "No se pudo abrir el archivo %s\n", nombreArchivo);
        return;
    }


    // Escribir los elementos del array
    for (int i = 0; i < filas; i++) {
        for (int j = 0; j < columnas; j++) {
            fprintf(archivo, "%f ", array[i][j]);
        }
        fprintf(archivo, "\n"); // Nueva línea después de cada fila
    }

    fclose(archivo);
}

int main() {

double phi[N][M];
double phinuevo[N][M];
 int i, j, iter, cuad, esp;
cuad = 390;
esp = 130;

for (i = 0; i < N; i++) {
        for (j = 0; j < M; j++) {
            if (j<=esp  ){
            phi[i][j]= 3.2;      // Voltajes positivos
            }
            else if (j>=cuad ){
                  phi[i][j]=0;       // Voltajes negativos
            }
            else {  
                phi[i][j]=0.0;
                }
            }

            
        }

guardarArrayEnArchivo(N, M, phi, "miArchivo.txt");

return 0;
}


