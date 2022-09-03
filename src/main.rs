use std::fs::File;
use std::io::ErrorKind;
mod utilities;
fn main() {
    utilities::set_backtrace(); //permite imprimir el backtrace
    //llamada a panic! de forma explicita
    //panic!("crash and burn");

    //codigo que hace un panic! implicito

    let v = vec![1,2,3];
    //v[99]; //este codigo hace panic

    //los errores recuperables con Result
    //Result es un enum que contiene 2 variantes y se ve de la siguiente forma
    enum Result<T,E> {
        Ok(T),
        Err(E),
    }

    //Result es un generico, es decir el parametro E es el Tipo devuelto y E el tipo del error
    //una funcion (entre tantas) que devuelve un result es File::open

    let greeting_file_result = File::open("hello.txt");

    //en esta implementacion de file::open(), los tipos devueltos en el enum son
    //T -> fs::File y E -> io::Error
    //hay que desenvolver con match (una opcion)

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            //panic!("Error al abrir/leer archivo: {:?}", error)
            //alternativo al panic, podemos tomar medidas, evaluando el error
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("El archivo no existe, se procedera a crear el archivo");
                    match File::create("hello.txt") {
                        Ok(fc) => {
                            println!("archivo creado con exito");
                            fc
                        },
                        Err(e) => panic!("Error en la creacion del archivo: {:?}",e),
                    }
                },
                other_error => {
                    panic!("El archivo existe, pero no se puede abrir/leer: {:?}", other_error);
                }
            }
        },
    };

    //aunque el uso de match es claro, una forma mas elegante es usar closures
    //si el Result no es Ok(T), toma el error y permite tomar acciones en caso de Err(E)

    let greeting_file_2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problema creando el archivo {:?}", error);
            })
        } else {
            panic!("problema al abrir el archivo {:?}", error);
        }
    });

    //atajos a panic en errores: unwrap y expect

    //cuando el usao de match es demasiado verbose o simplemente abortaremos
    //enviando un panic en el result Err() cuando la operacion sea erronea
    //es conveniente usar unwrap()
    //unwarap() devuelve el valor si la funcion o metodo retorno Ok(valor)
    //en caso contario, lanza un panic!

    let another_file = File::open("another_file.txt").unwrap();

    //expect funcion de la misma manera que unwrap() con el distintivo
    //de que se puede enviar un mensaje relacionado al error
    //algo que puede ser util para brindar mas contexto
    //unwrap() lanza un panic con un mensaje generico mientas
    //expect() permite enviar un mensaje personalizado

    //en el codigo grado produccion la mayoria de los rustaceans
    //eligen expect() para brindar mayor contexto

    let config_file = File::open("cfg.txt").expect("cfg.txt deberia estar incluido en este proyecto");
}
