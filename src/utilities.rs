use std::env;
//activa el backtrace, solo aplica a la ejecucion de el programa que lo invoca
pub fn set_backtrace(){
    env::set_var("RUST_BACKTRACE", "1");
}