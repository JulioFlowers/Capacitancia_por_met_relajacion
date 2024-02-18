import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

def normalize_matrix(matrix):
    matrix = np.array(matrix, dtype=float)
    magnitudes = np.linalg.norm(matrix, axis=1)  # Calcular magnitudes a lo largo de las filas

    normalized_matrix = matrix / magnitudes[:, np.newaxis]  # Divide cada fila por su magnitud
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

#Leer el ouput, almacenar phi y convertirlo en una matriz. 
ruta_archivo = 'Datos usados en el articulo/output.txt'
phi = archivo_texto_a_matriz(ruta_archivo)


print("Matriz:")
print(phi)

#se usan las matrices phi, Ex y Ey para generar las visualizaciones

#Graficar potencial
plt.figure(figsize=(8, 6))
#plt.contourf(phi, 100, cmap='inferno')
plt.contour(phi, 8, cmap='inferno')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Lineas equipotenciales, capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('graficas/pothileq.jpg', dpi=600)
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
plt.savefig('graficas/pothilheat.jpg', dpi=600)
plt.show()


file_path_x = Path(__file__).parent / 'Datos usados en el articulo/ex.txt'
file_path_y = Path(__file__).parent / 'Datos usados en el articulo/ey.txt'

Ex = np.loadtxt(file_path_x)
Ey = np.loadtxt(file_path_y)

Ex_sub = Ex[::10,::10]
Ey_sub = Ey[::10,::10]


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
plt.savefig('graficas/campovectorialfull.jpg', dpi=600)
plt.show()

Ex_sub = Ex[::30,::30]
Ey_sub = Ey[::30,::30]


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
plt.savefig('graficas/campovectorial.jpg', dpi=600)
plt.show()

#Graficar densidad de carga
rutacarga = 'Datos usados en el articulo/phip.txt'
Q = archivo_texto_a_matriz(rutacarga)
Qn = Q[::50,::50]

nonzero_indices = np.nonzero(Qn)

# Extraer los valores distintos de cero y sus índices correspondientes
nonzero_values = Qn[nonzero_indices]

# Crear una malla para el gráfico de contornos
x, y = np.meshgrid(range(Qn.shape[1]), range(Qn.shape[0]))

unique_nonzero_values = np.unique(nonzero_values)# Crear el gráfico de contorno
plt.contour(x, y, Qn, levels=np.linspace(unique_nonzero_values.min(), unique_nonzero_values.max(), len(unique_nonzero_values)), cmap='inferno')
plt.title('Distribución de Carga')
plt.xlabel('x [1:50 µm]')
plt.ylabel('y [1:50 µm]')
plt.colorbar(label='Carga [C/ε]')
plt.savefig('graficas/carga.jpg', dpi=600)
plt.show()
