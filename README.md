# Secure Password Generator

[![Rust Version](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

Secure Password Generator es una herramienta de línea de comandos (CLI) desarrollada en Rust que permite generar contraseñas seguras y personalizables. Este proyecto está diseñado para ser eficiente, seguro y flexible, demostrando las capacidades de Rust en el manejo de generación aleatoria y seguridad básica.

## Características

- **Longitud personalizable:** Define el número de caracteres de la contraseña generada.
- **Opciones de complejidad:** Incluye o excluye símbolos, números y letras mayúsculas.
- **Generación segura:** Utiliza generación aleatoria basada en el crate `rand`, asegurando contraseñas robustas.
- **Uso sencillo:** Interfaz CLI intuitiva con `clap`.

---

## Tabla de contenidos

1. [Requisitos](#requisitos)
2. [Instalación](#instalación)
3. [Uso](#uso)
4. [Ejemplos](#ejemplos)
5. [Arquitectura del Proyecto](#arquitectura-del-proyecto)
6. [Contribuciones](#contribuciones)
7. [Licencia](#licencia)
8. [Autor](#autor)

---

## Requisitos

- **Rust:** Versión 1.70 o superior.
- **Sistema operativo:** Compatible con Linux, macOS y Windows.

Verifica que Rust y Cargo están instalados ejecutando:

```bash
rustc --version
cargo --version
```

---

## Instalación

Clona este repositorio en tu máquina local y compila el proyecto con Cargo:

```bash
git clone https://github.com/tuusuario/secure-password-generator.git
cd secure-password-generator
cargo build --release
```

Esto generará un binario en el directorio `target/release`.

---

## Uso

Ejecuta el binario desde la línea de comandos para generar una contraseña. Ejemplo básico:

```bash
./secure-password-generator
```

### Opciones disponibles

- **`-l, --length`**: Especifica la longitud de la contraseña (por defecto, 16 caracteres).
- **`-s, --symbols`**: Incluye símbolos en la contraseña.
- **`-n, --numbers`**: Incluye números en la contraseña.
- **`-u, --uppercase`**: Incluye letras mayúsculas en la contraseña.

Ejemplo de uso con todas las opciones:

```bash
./secure-password-generator -l 20 -s -n -u
```

---

## Ejemplos

Generar una contraseña de 12 caracteres con números y letras mayúsculas:

```bash
./secure-password-generator -l 12 -n -u
```

Generar una contraseña de 16 caracteres con símbolos:

```bash
./secure-password-generator -s
```

Generar una contraseña simple de 8 caracteres:

```bash
./secure-password-generator -l 8
```

---

## Arquitectura del Proyecto

Este proyecto está estructurado de la siguiente manera:

- **`src/main.rs`**: Contiene el código principal del programa.
- **`Cargo.toml`**: Archivo de configuración del proyecto que incluye dependencias y metadatos.
- **`LICENSE`**: Licencia del proyecto.

---

## Contribuciones

¡Las contribuciones son bienvenidas! Por favor, sigue estos pasos:

1. Haz un fork del repositorio.
2. Crea una rama nueva para tu función o corrección.
3. Realiza tus cambios y haz un commit.
4. Envía un pull request detallando los cambios.

Ejemplo:

```bash
git checkout -b nueva-funcion
git add .
git commit -m "Agrega nueva función de ejemplo"
git push origin nueva-funcion
```

---

## Licencia

Este proyecto está licenciado bajo la Licencia MIT. Consulta el archivo [LICENSE](./LICENSE) para más detalles.

---

## Autor

Desarrollado por Alberto Mier (<info@albertomier.com>).