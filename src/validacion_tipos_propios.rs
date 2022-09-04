//se puede utilizar el sistema de tipos de Rust para validar
//los requerimientos que son necesarios para que nuestra libreria o programa
//puedan ser utilizado de forma segura

//vamos a tomar el ejemplo del guessing game para hacer que cumpla con
//los requerimientos de solo aceptanumeros enteros positivos
//y en el rango de 1 a 100

pub struct Guess {
    value: i32, //solo numeros positivos
}

impl Guess {
    //funcion asociada que permite construir una instancia de Guess
    //siempre y cuando cumpla con los requerimientos
    //al ser privada no se puede instanciar de otra forma
    //lo que evita construir instancias que no cumplan
    pub fn new(value: i32) -> Guess{
        if value < 1 || value > 100 {
            //si no cumple con el rango, hacemos panic
            panic!("El valor debe de estar entre 1 y 100, valor recibido: {}", value);
        }
        //si cumple con el rango retornamos una instancia de Guess
        Guess {
            value,
        }
    }

    //getter
    pub fn value(&self) -> i32 {
        self.value
    }
}