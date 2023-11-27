import numpy as np
import matplotlib.pyplot as plt

# Read data from the text file
f = open("/home/jflws/Documentos/Proyecto Final Fis. Comp/potencialrust/mi_archivo.txt")
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