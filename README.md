Instalación del Compilador de Rust

¡Hola a todes!

Este repositorio contiene un proyecto desarrollado en Rust y este README.md está diseñado para nuestro profesor de física computacional. Para ejecutar este proyecto, necesitará los archivos cargo.toml y cargo.lock, ya que utilizamos una biblioteca para guardar nuestras matrices en un archivo .txt mediante escritura paralela. Recomendamos que clone el repositorio o descargue el archivo .zip y abra la carpeta en un terminal. Luego, utilice el siguiente comando:

```bash
cargo run
```

Este comando compilará el proyecto en su plataforma y generará archivos .txt que podrá utilizar con el script graficadora.py.

Originalmente, teníamos la intención de trabajar en C, pero enfrentamos desafíos significativos, como tiempos de ejecución de hasta 5 minutos con nuestras matrices grandes y numerosos errores. Por esa razón, decidimos migrar a Rust, ya que su compilador es más amigable y proporciona explicaciones más detalladas sobre los errores.

A continuación, le proporcionamos un tutorial sobre cómo instalar Rust para que pueda comenzar a trabajar con nuestro proyecto.

Instalación de Rust
Abra su navegador web y vaya al sitio oficial de Rust. https://www.rust-lang.org/
Busque la sección "Get Started" o "Instalación" en la página principal.
Siga las instrucciones proporcionadas para su sistema operativo específico. Rust proporciona un instalador para sistemas Unix y un instalador específico para Windows.
Una vez completada la instalación, abra una nueva ventana de terminal para verificar que Rust se instaló correctamente. Puede hacerlo ejecutando el siguiente comando:

```bash
rustc --version
```

Si ve la versión de Rust instalada, ¡felicidades! Rust está listo para su uso.

Ahora, puede volver al directorio de nuestro proyecto y ejecutar el comando cargo run como se indicó anteriormente.

Esperamos que disfrute jugando con nuestro proyecto en Rust. Si tiene alguna pregunta o problema, no dude en contactarnos.

¡Gracias y feliz codificación!

Nota: Si trabaja en Windows necesira algunas herramientas de C de Visual Studio, el instalador lo especifica.
