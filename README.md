## Manejo de errores en Rust

al igual que otros lenguajes de programacion, Rust permite controlar el flujo de las acciones que
suceden una vez que sucedio un error.

Algunos lenguajes como Java tienen el uso de Excepciones, en el caso de Rust no existen excepciones
pero tenemos el tipo Result<T,E> para errores de los que el sistema se puede recuperar y panic! para
los errores en los que el sistema debe de abortar.

Recordemos que un error recuperable es el que generalmente es alterno al funcionamiento del sistema,
puede ser una entrada del usuario incorrecta, la falta de disponibilidad de un recurso, bloqueos.
Por lo que el sistema no tiene que parar completamente en la mayoria de los casos y es mas conveniente
tomar acciones como reintentos de conexion o reprocesos posteriores solo a los casos fallidos.

Los errores irrecuperables son lo que requieren que el sistema aborte cuando sucedan, deteniendo la ejecucion,
generalmente estan relacionados con bugs o defectos al construir el sistema, como acceder a un indice inexistente
de un arreglo, por lo que debemos de detener el sistema y corregir el error.

### macro panic

esta macro se llama cuando se programa una accion que causa un panic (como acceder a un indice inexistente) y
tambien se puede llamar explicitamente

por defecto panic seguira el siguiente proceso:

imprimir error -> unwind -> limpiar el stack -> terminar la ejecucion

ademas mediante una variable de entorno se puede mostrar el call stack de cuando sucedio el panic


### unwinding vs abort

como vimos la configuracion por defecto de Rust hace unwinding (limpieza), pero este comportamiento
se puede cambiar mediante la directiva panic = 'abort' en el archivo Cargo.toml baje la seccion
[profile] adecuada.

[profile.release]
panic = 'abort'

lo que hace esta directiva es indicar que al suceder un panic, el sistema termina sin ejecutar la limpieza,
la limpieza debe ser realizada por el sistema operativo

### backetrace

el backtrace se activa con la variable de entorno RUST_BACKTRACE = 1

para obtener el backtrace es necesario que debug symbols este activado
se encuentran activados en cargo build y cargo run
pero no cuando se usa --release