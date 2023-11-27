#include <stdio.h>
#include <math.h>

#define N 200
#define M 280
#define MAX_ITER 15000
#define TOL 1e-6

// Función para guardar un array bidimensional en un archivo
void guardarArrayEnArchivo(int filas, int columnas, double array[filas][columnas], const char *nombreArchivo) {
    FILE *archivo = fopen(nombreArchivo, "w");

    if (archivo == NULL) {
        fprintf(stderr, "No se pudo abrir el archivo %s\n", nombreArchivo);
        return;
    }

    // Escribir las dimensiones del array en la primera línea
    fprintf(archivo, "%d %d\n", filas, columnas);

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
double phinuevo[N][N];
 int i, j, iter, cuad, esp;
cuad = N/5;
esp = cuad/2;


for (i = 0; i < N; i++) {
        for (j = 0; j < M; j++) {
            if (j<=cuad && i<=4*cuad || j<=3*cuad && i>=4*cuad ||j>=6*cuad && i<=4*cuad || j>=4*cuad && i>=4*cuad || j>=3*cuad && j<=4*cuad && i<=cuad ){
            phi[i][j]= 5.0;      // Voltajes positivos
            }
            else if (j>= cuad + esp && j<=2*cuad + esp && i<= 2*cuad+esp || j>= 3*(cuad + esp) && j<=4*cuad + 3*esp && i<= 2*cuad+esp || j>= cuad +esp && j<= 5*cuad+esp && i>2*cuad+esp && i<=3*cuad+esp || j>=3*cuad+esp && j<=4*cuad+esp && i>=3*cuad+esp ){
                  phi[i][j]=-5;       // Voltajes negativos
            }
            else {  
                phi[i][j]=0.0;
                }
            }

            
        }

guardarArrayEnArchivo(N, M, phi, "miArchivo.txt");

return 0;
}

