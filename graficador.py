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


ruta_archivo = 'potencialrust/output.txt'
phi = archivo_texto_a_matriz(ruta_archivo)

"""
print("Matriz:")
print(phi)


plt.figure(figsize=(8, 6))
#plt.contourf(phi, 100, cmap='inferno')
plt.contour(phi, 8, cmap='inferno')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Potencial eléctrico capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('pothileq.jpg', dpi=500)
plt.savefig('pothilequi.jpg', dpi=500)
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
plt.savefig('pothilheat.jpg', dpi=500)
plt.show()
"""

"""
sub_phi = phi[::10, ::10]

# Calcula los gradientes para el subconjunto
Ex, Ey = np.gradient(sub_phi)

# Crea las coordenadas para el quiver plot
x, y = np.meshgrid(np.arange(0, sub_phi.shape[1], 1), np.arange(0, sub_phi.shape[0], 1))

# Grafica el campo eléctrico con quiver
plt.figure(figsize=(8, 6))
plt.streamplot(x, y, -Ex, -Ey,density=3)
plt.axis=("scaled")
plt.title('Campo eléctrico capacitor coplanar basado en curva de Hilbert [V/m]')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('ehilt.jpg', dpi=500)
plt.show()
"""


file_path_x = Path(__file__).parent / 'potencialrust/ex.txt'
file_path_y = Path(__file__).parent / 'potencialrust/ey.txt'

U = np.loadtxt(file_path_x)
V = np.loadtxt(file_path_y)

U_sub = U[::10,::10]
V_sub = V[::10,::10]
# Check the dimensions of U and V
print("U shape:", U.shape)
print("V shape:", V.shape)

magnitude = np.sqrt(U_sub**2 + V_sub**2)

Exn = U_sub/magnitude
Eyn = V_sub/magnitude
"""
# Create a 2D grid

M = np.hypot(U_sub, V_sub)
# Plot the vector field using streamplot
plt.quiver(X, Y, U_sub, V_sub, M, pivot='tip', width=0.022, scale=20)
plt.xlabel('X')
plt.ylabel('Y')
plt.title('Vector Field using Streamplot')
plt.savefig('ehilt.jpg', dpi=500)
plt.show()
"""


# Crear una cuadrícula de coordenadas
x = np.arange(0, Exn.shape[1])
y = np.arange(0, Eyn.shape[0])

# Crear una malla de coordenadas
X, Y = np.meshgrid(x, y)

# Graficar el campo vectorial
plt.figure(figsize=(10, 8))
plt.quiver(x, y, Exn, Eyn, angles='xy', scale_units='xy', scale=1, color='blue', headwidth=2.5)
plt.axis("scaled")
plt.title('Campo Vectorial')
plt.xlabel('X')
plt.ylabel('Y')
plt.show()


plt.show()


rutacarga = 'potencialrust/cargax.txt'
Q = archivo_texto_a_matriz(rutacarga)
Qn = Q/ np.sum(Q)
plt.figure(figsize=(8, 6))
#plt.contourf(rho, 100, cmap='inferno')
#plt.contour(Qn, 8, cmap='inferno')
plt.imshow(Q, cmap='viridis', interpolation='nearest')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Carga en X, capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('carhilheat.jpg', dpi=500)
plt.show()


rutacarga = 'potencialrust/cargay.txt'
Q = archivo_texto_a_matriz(rutacarga)
Qn = Q/ np.sum(Q)
plt.figure(figsize=(8, 6))
#plt.contourf(rho, 100, cmap='inferno')
#plt.contour(Qn, 8, cmap='inferno')
plt.imshow(Q, cmap='viridis', interpolation='nearest')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Carga en Y, capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('carhilheat.jpg', dpi=500)
plt.show()

"""
rutacarga = 'cargaprueba.txt'
Q = archivo_texto_a_matriz(rutacarga)
plt.figure(figsize=(8, 6))
#plt.contourf(rho, 100, cmap='inferno')
plt.contour(Q, 8, cmap='inferno')
#plt.imshow(Q, cmap='viridis', interpolation='nearest')
plt.colorbar(label='Potencial eléctrico [V]')
#plt.quiver(-Ex,-Ey,scale=5)
plt.title('Carga de prueba, capacitor coplanar basado en curva de Hilbert')
plt.xlabel('x [µm]')
plt.ylabel('y [µm]')
plt.savefig('carhilheat.jpg', dpi=500)
plt.show()
"""