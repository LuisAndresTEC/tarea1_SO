/*use std::{io, ops::RangeInclusive};
use std::fmt::Display;


use linux_personality::personality;
use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};


//Genera un arreglo con todas las palabras de la entrada por separado
fn fragmentador(opcion: &str) -> Vec<String>{
    let strings = opcion.split_whitespace().map(str::to_string).collect();
    return strings; 
}

fn validador(arr_elementos: Vec<String>) -> bool{
    //Valida que tenga la cantidad de elementos establecidos.
    //Valida que la sentencia empiece con -v o -V 
    //Valida que en la posicion 3
    if (arr_elementos.len()) == 4 && (arr_elementos[0]== "-v" || arr_elementos[0]== "-V") && arr_elementos[2] == "Prog" {
        return true;
    }else {
        return false;
    }
}

fn run_tracee() {
    ptrace::traceme().unwrap();
    personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();

    Command::new("ls").exec();

    exit(0)
}

fn main() {
    println!("ingrese el comando");
    let mut line = String::new();
 io::stdin()
 .read_line(&mut line)
 .expect("Failed to read line");

 let commands: Vec<String> = fragmentador(&line);
 validador(commands);

 run_tracee()
 
}*/
use nix::unistd::{fork, ForkResult, Pid};
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            run_tracee();
        } 

        Ok(ForkResult::Parent { child }) => {
            run_tracer(child);
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
}

fn run_tracer(_child: Pid) {
    loop{}
}

fn run_tracee() {
    Command::new("ls").exec();
}