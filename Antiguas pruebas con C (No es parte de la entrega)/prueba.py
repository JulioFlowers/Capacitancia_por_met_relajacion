import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

def normalize_matrix(matrix):
    matrix = np.array(matrix, dtype=float)
    magnitudes = np.linalg.norm(matrix, axis=1)  # Calculate magnitudes along rows

    normalized_matrix = matrix / magnitudes[:, np.newaxis]  # Divide each row by its magnitude
    return 100 * normalized_matrix

def archivo_texto_a_matriz(ruta_archivo):
    # Abrir el archivo de texto
    path = Path(__file__).parent / ruta_archivo
    with path.open('r') as archivo:
        # Leer líneas del archivo
        lineas = archivo.readlines()

        # Procesar las líneas y crear una matriz
        matriz = [list(map(float, linea.strip().split( ))) for linea in lineas]

        # Convertir la lista de listas a un array de NumPy
        matriz = np.array(matriz)

    return matriz

rutacarga = 'potencialrust/phip.txt'
Q = archivo_texto_a_matriz(rutacarga)

# Create a meshgrid for the contour plot
x, y = np.meshgrid(range(Q.shape[1]), range(Q.shape[0]))

# Create a contour plot
plt.contourf(x, y, Q, levels=np.unique(Q))
plt.colorbar()
plt.title('Contour Plot excluding 0 values')
plt.xlabel('X-axis')
plt.ylabel('Y-axis')
plt.colorbar(label='Values')
plt.show()
