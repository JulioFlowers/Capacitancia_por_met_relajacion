using DelimitedFiles
using Plots

function archivo_texto_a_matriz(ruta_archivo)
    # Leer directamente los valores del archivo usando readdlm
    matriz = readdlm(ruta_archivo)

    return matriz
end

ruta_archivo = "potencialrust/output.txt"
phi = archivo_texto_a_matriz(ruta_archivo)


heatmap(phi, c=:inferno, color=:blues, xlabel="x [µm]", ylabel="y [µm]", title="Potencial eléctrico capacitor coplanar basado en curva de Hilbert")

savefig("pothileq.svg")

heatmap(phi, c=:coolwarm, color=:blues, xlabel="x [µm]", ylabel="y [µm]", title="Potencial eléctrico capacitor coplanar basado en curva de Hilbert")

savefig("pothilheat.svg")

file_path_x = "potencialrust/ex.txt"
file_path_y = "potencialrust/ey.txt"

U = readdlm(file_path_x)
V = readdlm(file_path_y)

U_sub = U[1:end:10, 1:end:10]
V_sub = V[1:end:10, 1:end:10]

# Usar la función meshgrid para crear las matrices X e Y
X, Y = meshgrid(1:size(U_sub, 2), 1:size(U_sub, 1))

quiver(X, Y, quiver=(U_sub, V_sub), xlabel="X", ylabel="Y", title="Vector Field using Streamplot")

savefig("ehilt.svg")

rutacarga = "potencialrust/cargax.txt"
Q = archivo_texto_a_matriz(rutacarga)
contour(Q, levels=8, c=:inferno, xlabel="x [µm]", ylabel="y [µm]", title="Carga en X, capacitor coplanar basado en curva de Hilbert")

savefig("carhilheat_x.svg")

rutacarga = "potencialrust/cargay.txt"
Q = archivo_texto_a_matriz(rutacarga)
contour(Q, levels=8, c=:inferno, xlabel="x [µm]", ylabel="y [µm]", title="Carga en Y, capacitor coplanar basado en curva de Hilbert")

savefig("carhilheat_y.svg")

rutacarga = "cargaprueba.txt"
Q = archivo_texto_a_matriz(rutacarga)
contour(Q, levels=8, c=:inferno, xlabel="x [µm]", ylabel="y [µm]", title="Carga de prueba, capacitor coplanar basado en curva de Hilbert")

savefig("carhilheat_prueba.svg")
