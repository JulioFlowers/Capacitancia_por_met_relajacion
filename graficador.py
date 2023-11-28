import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

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

# Ejemplo de uso
ruta_archivo = 'miArchivo.txt'
m1 = archivo_texto_a_matriz(ruta_archivo)

print("Matriz:")
print(m1)

[Ey,Ex] = np.gradient(m1)

plt.figure(figsize=(8, 6))
#plt.contourf(m1, 100, cmap='inferno')
plt.contour(m1, 8, cmap='inferno')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Potencial eléctrico con círculo interno y marco externo')
plt.xlabel('x [cm]')
plt.ylabel('y [cm]')
plt.savefig('potcirc.jpg', dpi=500)
plt.show()

"""
# Read data from the text file
f = open("mi_archivo.txt")
data = np.loadtxt(f)

# Create a heatmap using matplotlib
plt.imshow(data, cmap='viridis', interpolation='nearest')
plt.colorbar(label='Values')  # Add a colorbar for reference

# Add labels to axes
plt.xlabel('X-axis')
plt.ylabel('Y-axis')

# Display the plot
plt.title('Heatmap from Text File')
plt.show()
"""

