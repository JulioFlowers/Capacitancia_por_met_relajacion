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


ruta_archivo = 'potencialrust/output.txt'
phi = archivo_texto_a_matriz(ruta_archivo)


print("Matriz:")
print(phi)


plt.figure(figsize=(8, 6))
#plt.contourf(phi, 100, cmap='inferno')
plt.contour(phi, 8, cmap='inferno')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Lineas equipotenciales, capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('pothileq.jpg', dpi=600)
plt.show()

plt.figure(figsize=(8, 6))
#plt.contourf(rho, 100, cmap='inferno')
#plt.contour(rho, 8, cmap='inferno')
plt.imshow(phi, cmap='coolwarm', interpolation='nearest')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Potencial eléctrico capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('pothilheat.jpg', dpi=600)
plt.show()


file_path_x = Path(__file__).parent / 'potencialrust/ex.txt'
file_path_y = Path(__file__).parent / 'potencialrust/ey.txt'

Ex = np.loadtxt(file_path_x)
Ey = np.loadtxt(file_path_y)

Ex_sub = Ex[::10,::10]
Ey_sub = Ey[::10,::10]
# Check the dimensions of U and V

magnitude = np.sqrt(Ex_sub**2 + Ey_sub**2)

Exn = Ex_sub/magnitude
Eyn = Ey_sub/magnitude

# Crear una cuadrícula de coordenadas
x = np.arange(0, Exn.shape[1])
y = np.arange(0, Eyn.shape[0])

# Crear una malla de coordenadas
X, Y = np.meshgrid(x, y)

# Graficar el campo vectorial
plt.figure(figsize=(10, 8))
plt.quiver(x, y, Exn, Eyn, angles='xy', scale_units='xy', scale=1, color='blue', headwidth=2.5)
plt.axis("scaled")
plt.title('Campo Electrico Normalizado [u.a]')
plt.xlabel('x [1:10 µm]')
plt.ylabel('y [1:10 µm]')
plt.savefig('campovectorialfull.jpg', dpi=600)
plt.show()

Ex_sub = Ex[::30,::30]
Ey_sub = Ey[::30,::30]
# Check the dimensions of U and V

magnitude = np.sqrt(Ex_sub**2 + Ey_sub**2)

Exn = Ex_sub/magnitude
Eyn = Ey_sub/magnitude

# Crear una cuadrícula de coordenadas
x = np.arange(0, Exn.shape[1])
y = np.arange(0, Eyn.shape[0])

# Crear una malla de coordenadas
X, Y = np.meshgrid(x, y)

# Graficar el campo vectorial
plt.figure(figsize=(10, 8))
plt.quiver(x, y, Exn, Eyn, angles='xy', scale_units='xy', scale=0.5, color='blue', headwidth=2.5)
plt.axis("scaled")
plt.title('Campo Electrico Normalizado [u.a]')
plt.xlabel('x [1:30 µm]')
plt.ylabel('y [1:30 µm]')
plt.savefig('campovectorial.jpg', dpi=600)
plt.show()


rutacarga = 'potencialrust/phip.txt'
Q = archivo_texto_a_matriz(rutacarga)
Qn = Q[::50,::50]

nonzero_indices = np.nonzero(Qn)

# Extract non-zero values and their corresponding indices
nonzero_values = Qn[nonzero_indices]

# Create a meshgrid for the contour plot
x, y = np.meshgrid(range(Qn.shape[1]), range(Qn.shape[0]))


unique_nonzero_values = np.unique(nonzero_values)# Create a contour plot
plt.contour(x, y, Qn, levels=np.linspace(unique_nonzero_values.min(), unique_nonzero_values.max(), len(unique_nonzero_values)), cmap='inferno')
plt.title('Distribución de Carga')
plt.xlabel('x [1:50 µm]')
plt.ylabel('y [1:50 µm]')
plt.colorbar(label='Carga [C/ε]')
plt.savefig('carga.jpg', dpi=600)
plt.show()
