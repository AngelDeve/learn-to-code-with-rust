# Práctica: estado de un despliegue

Tu tarea es simular el arranque de réplicas de una API. Solo usarás variables, operaciones aritméticas, comparación con `==` y `println!`. No necesitas servicios externos, bucles ni condicionales.

## Archivo y comando

Escribe tu solución en `examples/ejercicio_despliegue.rs`, dentro de `fn main()`. El archivo inicial compila y muestra una indicación; todavía no resuelve el ejercicio.

Desde la carpeta `variables-and-mutability` ejecuta:

```sh
cargo run --example ejercicio_despliegue
```

## Requisitos

1. Declara una constante `MAX_INTENTOS` de tipo `u32` con valor 3, fuera de `main`. Representa el máximo permitido en esta simulación.
2. Declara `servicio` con el texto `"api-pedidos"` y `replicas_deseadas` de tipo `u32` con valor 5. Mantén ambos inmutables.
3. Declara `replicas_disponibles` de tipo `u32` con valor 2 e `intentos_realizados` de tipo `u32` con valor 0. Decide qué variables necesitan `mut`.
4. Imprime el estado inicial.
5. Simula un primer intento: suma 2 a las réplicas disponibles y suma 1 al contador de intentos. Imprime el resultado.
6. Simula un segundo intento: pierde 1 réplica disponible y suma 1 al contador de intentos. Imprime el resultado.
7. Después de los cambios, calcula `replicas_pendientes` con una resta y `objetivo_cumplido` con `==`. Imprime el resumen.

`a == b` produce un `bool` que indica si ambos valores son iguales; `=` asigna un valor. Imprime el booleano directamente, sin un `if`.

El límite de intentos se muestra como configuración: declararlo no impide que el programa haga más intentos. Aplicar ese límite mediante control de flujo será una ampliación posterior.

## Salida esperada

Compara estas líneas con la salida de tu programa, sin contar los mensajes de compilación de Cargo:

```text
Servicio: api-pedidos
Inicial: 2/5 replicas disponibles
Intento 1: 4/5 replicas disponibles
Intento 2: 3/5 replicas disponibles
Pendientes: 2
Intentos: 2/3
Objetivo cumplido: false
```

## Antes de darlo por terminado

- El programa compila y coincide con la salida esperada.
- `mut` aparece solo en los dos contadores que cambian.
- Calculas las pendientes después del último cambio de estado.
- Puedes explicar por qué guardar la constante no aplica el límite automáticamente.
- Puedes explicar por qué este programa no crea réplicas reales ni conserva el contador al cerrarlo.

Para revisar y dar formato únicamente a este archivo:

```sh
cargo check --example ejercicio_despliegue
rustfmt --edition 2024 examples/ejercicio_despliegue.rs
```

## Pistas opcionales

<details>
<summary>Necesito recordar cómo modificar un contador</summary>

`contador += 1;` suma uno al valor actual; `contador -= 1;` resta uno. Una variable que se reasigna necesita `mut`.

</details>

<details>
<summary>No sé cómo imprimir dos valores juntos</summary>

Puedes usar `println!("Disponibles: {actuales}/{objetivo}");`, sustituyendo los nombres por tus variables.

</details>

## Reto adicional: shadowing y scope

En un bloque nuevo, declara `let zona = "mx";`, imprime el texto y vuelve a declarar `zona` con `zona.len()`. Imprime su longitud en bytes. Explica por qué esto usa dos vinculaciones inmutables y por qué el segundo nombre deja de estar disponible al salir del bloque.

No hay una solución completa incluida: el objetivo es que practiques tomando las decisiones. Si se atasca algo, conserva el mensaje de error del compilador para revisarlo junto con tu código.
