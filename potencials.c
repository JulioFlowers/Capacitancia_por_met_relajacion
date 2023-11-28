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



void main() {

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





// Calcular el campo eléctrico
    double Ex[N][M]; // Componente x del campo eléctrico
    double Ey[N][M]; // Componente y del campo eléctrico
    double dx = 1.0; // Espaciado en x
    double dy = 1.0; // Espaciado en y

    for (i = 1; i < N - 1; i++) {
        for (j = 1; j < M - 1; j++) {
            Ey[i][j] = -(phi[i + 1][j] - phi[i - 1][j]) / (2 * dx);
            Ex[i][j] = -(phi[i][j + 1] - phi[i][j - 1]) / (2 * dy);
        }
    }

// Se calcula la carga
 double rho[N][M];
for (i = 1; i < N - 1; i++) {
        for (j = 1; j < M - 1; j++) {
            rho[i][j]=(Ey[i+1][j]-Ey[i-1][j])+(Ex[i][j+1]-Ex[i][j-1]);
        }
    }
guardarArrayEnArchivo(N, M, rho, "carga.txt");


return 0;
}

