# Variables y mutabilidad

Abre esta carpeta como proyecto. Los ejemplos del profesor están en `src/bin/` y tu punto de partida es `src/main.rs`, que contiene una función `main` vacía.

## Ruta de aprendizaje en español

1. Lee la [guía de variables y mutabilidad](GUIA_VARIABLES_MUTABILIDAD.md).
2. Ejecuta `cargo run --example fundamentos_variables`.
3. Ejecuta `cargo run --example estado_infraestructura` y relaciona cada variable con configuración o estado.
4. Resuelve el [ejercicio de despliegue](EJERCICIO_INFRAESTRUCTURA.md) en `examples/ejercicio_despliegue.rs`.

Estos ejemplos son simulaciones locales: solo calculan e imprimen valores.

## Entorno listo

Verificado el 22 de octubre de 2026 en tu Mac con Apple Silicon:

- Homebrew 7.0.10.
- Rust (`rustc`) 1.99.1 y Cargo 1.99.1, canal estable.
- `rustup` instalado mediante Homebrew.
- `rust-analyzer`, `rustfmt` y Clippy instalados.
- Herramientas de compilación de Apple disponibles.

Homebrew y Rust están configurados en `~/.zprofile`. Abre una terminal nueva si tenías una abierta antes de la instalación.

Pasaron `cargo check --bins`, `cargo run` y `cargo run --bin immutable_and_mutable_variables`. Los ejemplos del profesor muestran algunas advertencias de variables sin usar; no impiden compilar.

La instalación con Homebrew sigue la documentación de [rustup](https://rust-lang.github.io/rustup/installation/other.html) y la [fórmula oficial de Homebrew](https://formulae.brew.sh/formula/rustup).

## Empezar la sección

En una terminal nueva, entra al proyecto:

```sh
cd '/Users/angelgutierrez/Documents/ChatGPT/curso de rust/learn-to-code-with-rust/variables-and-mutability'
rustc --version
cargo --version
cargo check --bins
```

Para ejecutar tu archivo `src/main.rs`:

```sh
cargo run
```

Al principio no imprime nada porque `main` está vacía. Se configuró `default-run` para que este comando elija tu programa entre los múltiples ejemplos de la sección.

Para ejecutar los ejemplos del profesor:

```sh
cargo run --bin intro_to_variables
cargo run --bin immutable_and_mutable_variables
```

El primer ejemplo no imprime nada y puede mostrar advertencias de variables sin usar. El segundo imprime primero 10 repeticiones y después 15.

Para dar formato a tu código:

```sh
cargo fmt
```

Cuando llegues al reto, las instrucciones están en `src/coding_challenge.rs`. Es un archivo de comentarios; escribe tu solución en `src/main.rs` y ejecútala con `cargo run`.
