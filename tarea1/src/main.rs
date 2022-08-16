use std::ops::Add;
use std::{io, ops::RangeInclusive};
mod system_call_names;

use linux_personality::personality;
use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};



//Genera un arreglo con todas las palabras de la entrada por separado y valida.
fn fragmentador(opcion: String) -> Vec<String>{
    opcion.as_str();
    let mut strings: Vec<String> = opcion.split_whitespace().map(|x| x.to_string()).collect();
    if (strings[1]== "-v" || strings[1]== "-V") && (strings[0]== "rastreador"){
        return strings;
    }else {
        print!("ERROR!");
        exit(1);
    }
}

fn entrada_texto() -> String{
    let mut entrada = String::new();
    print!("Ingrese la sentencia para el rastreador: ");
    io::stdin().read_line(&mut entrada).unwrap();
    return entrada;
}

fn run_tracee(opcion:String) {
    ptrace::traceme().unwrap();
    personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();
    Command::new(opcion).exec();

    exit(0)
}




fn menu_tracker_v(binario: String) {
    //duplica el proceso
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            run_tracee(binario);
        }

        Ok(ForkResult::Parent { child }) => {
            rastreador_v(child);
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
}

fn menu_tracker_V(binario: String){
    //duplica el proceso
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            run_tracee(binario);
        }

        Ok(ForkResult::Parent { child }) => {
            rastreador_V(child);
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
    
}

fn rastreador_V(child: Pid)-> Vec<String> {
    loop {
        wait().unwrap();
        //let mut calls_repo:Vec<String, String> = Vec::new();
        match ptrace::getregs(child) {

            Ok(x) => print!(
                "{:?} {:?}\n",
                system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize],
                x               
            ),
                Err(_) => break,

        };
        match ptrace::syscall(child, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
    return Vec::new();
}


fn rastreador_v(child: Pid)-> Vec<String> {
    loop {
        wait().unwrap();
        //let mut calls_repo:Vec<String> = Vec::new();
        match ptrace::getregs(child) {

            Ok(x) => println!(
                "{:?} {:?}\n",
                system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize],
                x               
            ),
                Err(_) => break,

        };
        match ptrace::syscall(child, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
    return Vec::new();
}




fn main() {
    let entrada: String = entrada_texto();
    let entradas_list:Vec<String>=fragmentador(entrada);
    
    if entradas_list[1] == "-v" {
        //rastrea sin pausa
        menu_tracker_v(entradas_list[2].to_string());
     }
     else{
        //rastrear con pausa 
        menu_tracker_V(entradas_list[2].to_string());
     }

}







