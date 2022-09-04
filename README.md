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

### backtrace

el backtrace se activa con la variable de entorno RUST_BACKTRACE = 1

para obtener el backtrace es necesario que debug symbols este activado
se encuentran activados en cargo build y cargo run
pero no cuando se usa --release

### panic or not pacic

es importante analizar las situaciones donde queremos lanzar un panic y detener la ejecucion
y donde es importante controlar el error y establecer medidas (como la propagacion) para
continuar la ejecucion.

Esto es dependiente del contexto, aunque hay escenarios donde esta claro si hacer panic o
manejar el error es preferible, obviamente siempre tenemos la opcion de hacer panic en todo
error, pero esto se puede volver inecesariamente problematico.

imagina que tienes un programa que envia correos y detienes el envio de todos los correos
por que uno de ellos tiene un formato invalido, tendrias que realizar la investigacion en
el backtrace, podrias quitar el correo y guardarlo en algun lugar (lo cual seria ineficiente
y con una tendencia a errores) y esperar contactarte con el dueño del correo por otro medio
para solucionar la situacion, ya que es una informacion que no depende de ti.

mientras todo eso sucede, la lista restante no recibe su correo, o si decides quitar el
correo, puede que se tratara de una persona que debia de recibir ese correo o bien
tienes que purgar la lista (que puede ser numerosa) para que los destinatarios que
ya recibieron su correo no reciban un duplicado en el reproceso (tarea con alta probabilidad de error).

Todo ello se pudo haber evitado con manejar el error, almacenar un log con los correos no procesados y
posteriormente tomar una decision de que se hara con esos correos erroneos.

un caso contrario puede ser cuando es esencial que el correo llegue a todos los destinatarios, con lo
que seria util realizar una validacion previa del formato del conjunto de correos, de existir n
erroneos, seria buena idea hacer un panic y analizar que se realizara en ese escenario.

por que al final del dia, no puedes controlar recibir informacion erronea, bandejas llenas,
servidores de correo no disponibles o intermitencias de conexion, no puedes evitar esos errores
solo puedes diseñar las rutas a seguir y es importante que se realicen de manera consciente
y lo mas fiables y elegantes posibles (para evitar generar nuevos errores mientras arreglas un error).

- Ejemplos, prototipos (de codigo) y tests
    en el caso de los ejemplos puede resultar confuso incluir una estrategia de manejo de errores robusta
    por que el onjetivo es mostrar un ejemplo de como usar una libreria, funcion, etc. por lo que es mas claro
    usar panic, unwrap o expect en ejemplos.
    para los prototipos es mas practico usar unwrap o expect, ya que el objetivo generalmente es obtener la
    funcionalidad, una vez que se ha obtenido la funcionalidad de los componentes, es importante diseñar una
    estrategia robusta.
    Finalmente en el caso de los tests, se usa panic debido a que si algo falla en el componente que estas probando
    muy probablemente quieras que todo ese test falle y la forma de marcar un test como erroneo es con panic (unwrap o expect).

- Cuando se tiene mas informacion que el compilador
    hay situaciones donde siempre obtendras un valor que no es Err, debido a como se ha implementado un codigo, pero el
    compilador no tiene forma de asegurar eso, si posterior a una inspeccion se llega a esa conclusion, el uso de expect
    o unwrap es valido, ademas es muy recomendable documentar el por que es imposible que se obtenga un error en ese codigo.

    por ejemplo, el siguiente codigo con una direccion ip valida hard-coded, el metodo parse() nunca fallara:

    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    
    contrario un escenario donde se obtiene la ip desde una entidad externa (humano o API) y es posible que esta direccion
    no sea valida, en ese caso es importante y preferible no usar expect o unwrap() y diseñar una estrategia de manejo
    del error. incluso si es necesario se puede lanzar un panic explicito si es necesaria esa direccion para las tareas
    posteriores del programa, una vez que ya se ha tratado el error, se ha escrito un error o incluso se ha solicitado 
    nuevamente a la entidad externa (reintentos de consumo del API).

- Guidelines para manejo de errores
    - es remendable hacer panic cuando el codigo podria terminar en "bad taste", es decir cuando es probable que agentes
        externos pasen datos incorrectos, pasado ese punto el programa debe de "confiar" que no esta en ese estado,
        lo cual no se puede asegurar o identificar en cada paso de la ejecucion.
    - si alguien llama tu codigo y le pasa valores que no tienen sentido, es mejor retornar el error y dejar que
        el usuario de la libreria decida que medidas tomar, aunque en casos donde continuar puede ser inseguro
        o provocar un daño es mejor hacer panic y alertar al usuario de la libreria, posteriormente se puede resolver en desarrollo.
        tambien es apropiado usar panic cuando estas utilizando librerias o codigo externo del que no tienes control de su desarrollo
        y por tanto no lo puedes reparar.
    - cuando la falla es posible, es mas apropiado retornar el error que hacer panic, ya que estas situaciones son comunes como
        datos malformed o HTTP requests con status de limit rate, y mediante codigo se pueden manejar reintentos o vias alternas
    - cuando el codigo puede causar daños si se usan valores invalidos o incorrectos, el codigo debe de validar primero que los
        valores son correctos y si no lo son hacer panic, esto por razones de seguridad.
    - las funciones tienen contratos, por lo que su comportamiento solo esta garantizado si los valores de entrada cumplen con
        ciertos requerimientos y siempre que se viole dicho contrato se hara panic, como en el caso de la libreria estandar, un
        ejemplo de este funcionamiento es hacer un index out of bounds que podria exponer locaciones de memoria, ante esa situacion
        se hace panic, ademas de que el error esta del lado de quien usa la libreria y no con un problema que se debe de manejar o
        corregir en el codigo de la funcion.
    - tener muchos error check puede ser verbose o molesto, por lo que el type system de Rust realiza muchos de los checks por ti
        el compilador se asegura de los requerimientos y limitaciones de cada tipo, como evitar un valor negativo en u32, argumentos
        concretos donde se requiere un valor concreto y la posibilidad de las variantes de enum como Option.