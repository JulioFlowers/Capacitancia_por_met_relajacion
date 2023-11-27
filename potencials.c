#include <stdio.h>
#include <math.h>

#define N 189
#define M 247
#define MAX_ITER 1000
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
 int i, j, k, iter, cuad, esp;
cuad = 39;
esp = 13;

// Condiciones de contorno
for (i = 0; i < N; i++) {
        for (j = 0; j < M; j++) {
            if (i<=esp || i> esp && i<=N-2*esp && j<=cuad || i> esp && i<=N-2*esp && j>M-cuad || i>N-cuad-2*esp && i<=N-2*esp && j<=2*cuad+esp ||i>N-cuad-2*esp && i<=N-2*esp && j>M-2*cuad-esp || i<=cuad+2*esp && j>2*(cuad+esp) && j<=M-2*(cuad+esp) ){
            phi[i][j]= 3.2;      // Voltajes positivos
            }
            else if (i>2*esp && i<= N-(3*esp+cuad)&& j> (cuad + esp) && j<= (2*cuad + esp)|| i>2*esp && i<= N-(3*esp+cuad) && j<= M-(cuad + esp)  && j> M-(2*cuad + esp) || i > 3*esp+cuad && i<= N-3*esp-cuad && j>cuad+esp && j<=M-cuad-esp ||i> N-3*esp-cuad && j>2*(esp+cuad) && j<=M-2*(esp+cuad)||i>= N - esp ){
                  phi[i][j]=0.0;       // Voltajes negativos
            }
            else {  
                phi[i][j]=1.6;
                }
            }            
        }

// calculo del potencial

for(k=0; k<MAX_ITER ; k++){


  for (i = 0; i < N; i++) {
        for (j = 0; j < M; j++) {
            if (i<=esp || i> esp && i<=N-2*esp && j<=cuad || i> esp && i<=N-2*esp && j>M-cuad || i>N-cuad-2*esp && i<=N-2*esp && j<=2*cuad+esp ||i>N-cuad-2*esp && i<=N-2*esp && j>M-2*cuad-esp || i<=cuad+2*esp && j>2*(cuad+esp) && j<=M-2*(cuad+esp) ){
            phi[i][j]= 3.2;      // Voltajes positivos
            }
            else if (i>2*esp && i<= N-(3*esp+cuad)&& j> (cuad + esp) && j<= (2*cuad + esp)|| i>2*esp && i<= N-(3*esp+cuad) && j<= M-(cuad + esp)  && j> M-(2*cuad + esp) || i > 3*esp+cuad && i<= N-3*esp-cuad && j>cuad+esp && j<=M-cuad-esp ||i> N-3*esp-cuad && j>2*(esp+cuad) && j<=M-2*(esp+cuad)||i>= N - esp ){
                  phi[i][j]=0.0;       // Voltajes negativos
            }
            else {  
                 phi[i][j] = 0.25 * (phi[i + 1][j] + phi[i - 1][j] + phi[i][j + 1] + phi[i][j - 1]);
                }
            }            
        }


}

guardarArrayEnArchivo(N, M, phi, "miArchivo.txt");


return 0;
}

