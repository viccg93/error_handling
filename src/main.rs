use std::fs::{self,File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;
use validacion_tipos_propios::Guess;
mod utilities;
mod validacion_tipos_propios;

//la funcion main tambien puede retornar un Result
//Box<dyn Error> puede entenderse como "cualquier error"
//cuando un main retorna Result, la ejecucion termina en 0 si retorna Ok
//y devuelve un valor integer diferente de 0 si retorna Err
//la funcion main puede retornar cualquier tipo que implemente el trait std::process::Termination
fn main() -> Result<(), Box<dyn Error>> {
    utilities::set_backtrace(); //permite imprimir el backtrace
    //llamada a panic! de forma explicita
    //panic!("crash and burn");

    //codigo que hace un panic! implicito

    let v = vec![1,2,3];
    //v[99]; //este codigo hace panic

    //los errores recuperables con Result
    //Result es un enum que contiene 2 variantes y se ve de la siguiente forma
    //se comenta debido a conflicto con std::result::Result
    //enum Result<T,E> {
        //Ok(T),
        //Err(E),
    //}

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

    //cuando el uso de match es demasiado verbose o simplemente abortaremos
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

    //la propagacion de errores es cuando el error se envia a niveles superiores donde la funcion fue llamada
    //para tomar las deciciones en esa capa superior
    //esto brinda utilidad en situaciones donde se deben tomar distintas decisiones en escenarios distintos

    fn read_username_from_file_v1() -> Result<String,io::Error> {
        let user_file_result = File::open("user_file.txt"); //abrimos archivo

        //evaluamos si se pudo abrir
        let mut user_file = match user_file_result {
            Ok(file) => file,
            Err(e) => return Err(e), //sale de la funcion
        };

        //su se pudo abrir el archivo, procedemos a leer el contenido
        let mut username = String::new();

        match user_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    //operador ?, shortcut para propagacion de errores
    //funciona como match, si la operacion devuelve Err
    //esa varinate se propaga y usa early return
    //el la variante es Okm devuelve el valor de la variante

    fn read_username_from_file_v2() -> Result<String, io::Error> {
        let mut user_file = File::open("user_file.txt")?; //añadimos el operador, en caso de Ok, ya no devuelve Ok(value), devuelve value
        //si se obtuvo la variante Ok, el siguiente codigo se ejecuta, en caso contrario hace return con Err
        let mut username = String::new();
        user_file.read_to_string(&mut username)?;
        Ok(username) //si fue correcta la lectura, regresamos la cadena envuelta en Ok
    }

    //el operador ? usa la funcion from definida en el trait From, que realiza conversiones de tipos
    //lo que significa que el error recibido se convierte al que espera la funcion en la que se encuentra
    //es util cuando una funcion retorna un tipo de error que engloba todas las formas posibles en que 
    //la funcion puede fallar, aun si algunas partes de dicha funcion pueden fallar por otras razones. 
    //esa funcion es util al definir tipos de errores propios que hagan impl, de manera que la conversion
    //es transparente

    //tambien se puede simplificar aun mas la funcion, usando chaining

    fn read_username_from_file_v3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("user_file.txt")?.read_to_string(&mut username)?;
        Ok(username) //si fue correcta la lectura, regresamos la cadena envuelta en Ok
    }

    //una forma aun mas simplificada de la funcion es mediante la funcion (convenient) fs::read_to_string()
    //aunque perdemos la capacidad de explicar con mayor detalle donde fallo la lectura del archivo
    //como en la v1
    fn read_username_from_file_v4() -> Result<String, io::Error> {
        fs::read_to_string("user_file.txt")
    }

    //para utilizar el operador ? la funcion desde donde se llama debe de retornar
    //un enum y este enum debe de ser Result, Option o cualquiera que implemente FromResidual
    //no se puede llamar desde una funcion que retorne (), ya que el comportamiento de error
    //resulta en retornar el enum con error en early return (propagacion)

    //usar el operador ? con Option tiene es similar a Result, solo que el early return retorna None

    fn last_char_of_first_line(text: &str) -> Option<char>{
        //next() devuelve None si no hay una siguiente iteracion
        //last() devuelve tambien una variante de Option
        text.lines().next()?.chars().last()
    }

    let file_on_main = File::open("hello.txt")?; //si la lectura no es exitosa hace early return

    //validacion en tipo propio
    let guess = Guess::new(10);
    //let another_guess = Guess::new(200); //lanza un panic
    println!("El valor de Guess.value es {}", guess.value());

    //no se permite la instanciacion directa por que el campo value es privado
    //let guess2 = Guess { 
        //value: 200,
    //};

    Ok(()) // si la lectura del archivo fue exitosa, retornamos la expresion resultante de Ok() sobre ()
}
