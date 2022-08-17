use std::{io, ops::RangeInclusive};
use std::io::{stdin, stdout, Read, Write};
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
    println!("Ingrese la sentencia para el rastreador: ");
    io::stdin().read_line(&mut entrada).unwrap();
    return entrada;
}

fn ejecuta_binario(opcion:String) {
    ptrace::traceme().unwrap();
    personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();
    Command::new(opcion).exec();

    exit(0)
}




fn menu_tracker_v(binario: String) {
    //duplica el proceso
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            ejecuta_binario(binario);
        }

        Ok(ForkResult::Parent { child }) => {
            reps_counter_hash(rastreador_v(child));
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
            ejecuta_binario(binario);
        }

        Ok(ForkResult::Parent { child }) => {
            reps_counter_hash(rastreador_V(child));
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
    
}

fn presiona_tecla(){
    let mut entrada = String::new();
    print!("Presione alguna tecla para continuar: ");
    io::stdin().read_line(&mut entrada).unwrap();
}

fn rastreador_V(child: Pid)-> Vec<String> {
    let mut calls_repo:Vec<String> = Vec::new();
    loop {
        wait().unwrap();
        match ptrace::getregs(child) {

            Ok(x) => print!(
                "{:?} {:?}\n",
                system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize],
                x               
            ),
                Err(_) => break,

        };
        match ptrace::getregs(child) {

            Ok(x) =>
            calls_repo.push((system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize]).to_string()),
            Err(_) => break,
        }
        presiona_tecla();
        match ptrace::syscall(child, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
        
        
    }
    return calls_repo;
}

fn rastreador_v(child: Pid)-> Vec<String> {
    let mut calls_repo:Vec<String> = Vec::new();
    loop {
        wait().unwrap();
        match ptrace::getregs(child) {

            Ok(x) =>
                println!("{:?} {:?}\n",system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize],x),
                Err(_) => break,
        
        };
        match ptrace::getregs(child) {

            Ok(x) =>
            calls_repo.push((system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize]).to_string()),
            Err(_) => break,
        }
        match ptrace::syscall(child, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
    return calls_repo;
}

fn reps_counter_hash(calls_repo: Vec<String>){
    //crea un hashmap para contar las repeticiones de las llamadas
    let mut calls_counter: Vec<(String, i128)> = Vec::new();
    let mut calls_copied: Vec<String> = Vec::new();
    //recorre el arreglo de llamadas
    for i in calls_repo.iter(){
        //verifica si el hashmap contiene la llamada
        if (reps_counter_hash_verifier(i.to_string(), calls_copied.to_vec())) {
            //si contiene la llamada.
        }else{
            //si no contiene la llamada.
            
            let mut count:i128 = reps_counter_hash_aux(i.to_string(), calls_repo.to_vec());
            calls_copied.push(i.to_string());
            calls_counter.push((i.to_string(), count));
           
        }
        
    }
    //imprime el hashmap
    reps_counter_hash_printer(calls_counter);
}

fn reps_counter_hash_verifier(i: String, calls_copied: Vec<String>)-> bool{
    
    for j in calls_copied.iter(){
        
        if i == j.to_string(){

            return true;
        }
    }
    return false;
}


fn reps_counter_hash_aux(call:String, calls_repo: Vec<String>)->i128{
   
    let mut counter:i128 = 0;
    for i in calls_repo{
        if i == call{
            counter += 1;
        }
    }
    
    return counter;
}

fn reps_counter_hash_printer(calls_counter: Vec<(String, i128)>){
    for i in calls_counter{
        
        println!("Llamada: {}, Cantidad: {}", i.0 , i.1);
    }
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







