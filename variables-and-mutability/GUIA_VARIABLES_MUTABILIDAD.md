# Variables y mutabilidad: de lo básico a infraestructura

Objetivo: decidir qué datos pueden cambiar, expresar esa decisión en Rust y distinguir la configuración de un servicio de su estado durante la ejecución.

## 1. Una variable da un nombre a un valor

```rust
let servidores = 3;
println!("Tengo {servidores} servidores");
```

`let` crea una vinculación entre un nombre y un valor. `servidores` es el nombre y `3` es su valor inicial. El punto y coma termina la instrucción; `{servidores}` inserta el valor en el mensaje.

Rust conoce el tipo de cada variable al compilar. A menudo lo deduce, pero puedes escribirlo para aclarar su intención:

```rust
let replicas: u32 = 3;
let servicio: &str = "api";
let saludable: bool = true;
```

`u32` representa un entero de 32 bits sin signo; `bool` admite `true` o `false`. Aquí `&str` se usa para un texto literal. Más adelante estudiarás referencias y ownership; no necesitas dominarlos para este ejercicio.

Elegir `u32` evita representar números negativos, pero no impone un límite útil para tu negocio: `0` y un millón también caben. La validación del dominio sigue siendo responsabilidad del programa.

## 2. `let`: inmutable por defecto

```rust
let puerto: u16 = 8080;
println!("Puerto: {puerto}");
```

Después de inicializar `puerto`, no puedes reasignarlo usando esa misma vinculación. Este ejemplo falla al compilar:

```rust,compile_fail
fn main() {
    let puerto: u16 = 8080;
    puerto = 9090; // Error E0384: reasignación de una variable inmutable.
    println!("{puerto}");
}
```

Esto ayuda a detectar una modificación accidental antes de ejecutar el programa. En un servicio, por ejemplo, puedes leer el puerto desde su configuración al arrancar y tratarlo como fijo durante esa ejecución. `let` admite valores calculados en tiempo de ejecución: inmutable no significa conocido de antemano.

## 3. `let mut`: un valor que sí evoluciona

```rust
let mut solicitudes: u32 = 0;
println!("Antes: {solicitudes}");
solicitudes += 1;
println!("Después: {solicitudes}");
```

`mut` permite modificar la variable. `solicitudes += 1` equivale a `solicitudes = solicitudes + 1`. La salida es `Antes: 0` y `Después: 1`.

Un contador de solicitudes, intentos de reconexión o tareas completadas cambia conforme sucede algo. Esa evolución justifica `mut`. Empieza con `let` y añade `mut` cuando exista una modificación concreta.

El tipo permanece fijo. Una variable `u32` mutable puede pasar de `0` a `1`, pero no puedes asignarle después el texto `"uno"`.

## 4. Shadowing: un nuevo nombre que oculta al anterior

```rust
let etiqueta = "api";
println!("Texto: {etiqueta}");
let etiqueta = etiqueta.len();
println!("Longitud en bytes: {etiqueta}");
```

La segunda instrucción `let` crea una nueva vinculación con el mismo nombre. La primera contiene texto; la segunda contiene un número de tipo `usize`. Ambas son inmutables. Para `"api"`, la longitud es 3 bytes; con otros caracteres, bytes y caracteres pueden diferir.

En infraestructura resulta útil al transformar una entrada de configuración en un valor preparado para usar. Usa shadowing si el nombre sigue describiendo bien el resultado. Si necesitas conservar las dos versiones a la vez, usa nombres como `puerto_texto` y `puerto_numero`.

## 5. `const`: una regla fija del programa

```rust
const MAX_INTENTOS: u32 = 3;
```

Una constante exige tipo explícito, usa una expresión evaluable en tiempo de compilación y no admite `mut`. Por convención se escribe en mayúsculas con guiones bajos. Puede declararse fuera de `main`.

Una regla fija de esta demostración puede ser una constante. Un límite que operaciones debe poder cambiar mediante un archivo o una variable de entorno suele leerse al arrancar y guardarse en una variable `let`. Modificar una constante del código requiere recompilar para cambiar ese valor en el ejecutable.

| Forma | Qué expresa | Ejemplo de infraestructura |
| --- | --- | --- |
| `let puerto = 8080;` | Valor fijo después de inicializar esta vinculación | Configuración de arranque |
| `let mut intentos = 0;` | Estado que cambia manteniendo el tipo | Intentos realizados |
| `let dato = ...; let dato = ...;` | Nueva vinculación que oculta la anterior | Transformación de una entrada |
| `const LIMITE: u32 = 3;` | Valor definido mediante una expresión constante | Regla fija incorporada al programa |

## 6. Scope: dónde existe un nombre

```rust
let replicas = 3;
{
    let replicas = 5;
    println!("Dentro: {replicas}");
}
println!("Fuera: {replicas}");
```

Imprime 5 dentro y 3 fuera. Las llaves delimitan un bloque. El nombre interior oculta al exterior durante ese bloque; no lo modifica. Limitar el alcance de un valor hace más fácil seguir el código y evita usar datos temporales fuera de su contexto.

## 7. ¿Para qué sirve en infraestructura?

Imagina un programa que supervisa un despliegue. Necesita saber qué se quiere conseguir y qué está ocurriendo:

| Dato | Elección en esta simulación | Motivo |
| --- | --- | --- |
| Nombre del servicio | `let` | No cambia durante la simulación |
| Réplicas deseadas | `let` | El objetivo permanece fijo en este recorrido |
| Réplicas disponibles | `let mut` | Aumentan al arrancar instancias y disminuyen al fallar |
| Réplicas pendientes | `let` calculado | Es una fotografía calculada después de los eventos |

Ejecuta el ejemplo completo:

```sh
cargo run --example estado_infraestructura
```

El ejemplo empieza con 1 réplica disponible, suma 2 y resta 1. Termina con 2 disponibles de las 4 deseadas; faltan 2.

Hay una diferencia fundamental entre calcular un dato y mantenerlo actualizado automáticamente:

```rust
let deseadas: u32 = 4;
let mut disponibles: u32 = 1;
let pendientes = deseadas - disponibles;
disponibles += 1;
println!("Disponibles: {disponibles}; pendientes anteriores: {pendientes}");
```

`pendientes` sigue valiendo 3. La resta se ejecutó al declararla: no es una fórmula reactiva. Para obtener el estado actual, vuelve a calcularla después del cambio. Esta distinción importa al generar métricas y resúmenes de estado.

El ejemplo usa valores pequeños y conocidos que mantienen `disponibles <= deseadas`. En un sistema real, esa relación puede cambiar, y restar enteros sin signo requiere manejar el caso en que el resultado sería negativo. Más adelante puedes estudiar validación, `checked_sub` y `saturating_sub`; no se debe ocultar un estado inválido sin decidir cómo tratarlo.

## 8. Lo que estas variables hacen en el sistema

Son datos en la memoria del proceso. Cambiar `disponibles` en el ejemplo solo modifica ese número: para consultar o modificar infraestructura real necesitarías llamadas a sus APIs y tratamiento de sus respuestas.

Tampoco se guarda el valor en disco automáticamente. Al ejecutar de nuevo el ejemplo, los contadores vuelven a sus valores iniciales.

`mut` tampoco equivale a un bloqueo, una operación atómica o un permiso de administración. Si varios hilos necesitan modificar un contador compartido, entran en juego ownership y mecanismos como `Mutex` o los tipos atómicos. Si son procesos o máquinas diferentes, hacen falta mecanismos de coordinación entre ellos. Estos temas se estudiarán después.

Estas decisiones ayudan a expresar intención, reducir cambios accidentales y revisar con más claridad qué partes del estado pueden variar. No garantizan por sí solas que la lógica del despliegue sea correcta ni que el programa sea más rápido.

## 9. Prueba y practica

```sh
cargo run --example fundamentos_variables
cargo run --example estado_infraestructura
cargo check --examples
```

Después resuelve el [ejercicio de infraestructura](EJERCICIO_INFRAESTRUCTURA.md). Los ejemplos propios están en `examples/`; los originales del curso siguen en `src/bin/`.

## Referencias y siguiente paso

- [The Rust Book: variables, mutabilidad, constantes y shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).
- [The Rust Book: tipos de datos](https://doc.rust-lang.org/book/ch03-02-data-types.html).
- [The Rust Book: estado compartido y Mutex](https://doc.rust-lang.org/book/ch16-03-shared-state.html), para profundizar más adelante.

Cuando conozcas condicionales y bucles, puedes extender el ejercicio para simular varios intentos. Al estudiar `Result`, puedes leer y validar la configuración. Al llegar a concurrencia, puedes explorar un contador compartido. Cada ampliación debe conservar un ejemplo ejecutable, su salida esperada y una explicación de sus límites.
