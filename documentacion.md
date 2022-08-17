# Documentación Tarea #1

#### Luis Andrés Rojas Murillo.

#### Carné: 2020124333

## Introducción:

​	En esta tarea se especifica la creación de un rastreador de llamadas del sistema con dos modalidades, una donde simplemente se recibe el comando de llamada y se ejecuta mostrando todos las llamadas subsecuentes generadas a raíz de la llamada primaria, por otro lado en el segunda instrucción se esperan poder desplegar de igual forma todas llamadas subsecuentes generadas por una llamada primaria, no obstante con la particularidad de que la ejecución de este proceso va a ser controlada por el usuario o mejor dicho entrando en un estado de espera posterior a cada llamado, del cual solo puede salir cunado el usuario presione una tecla.

## Ambiente de desarrollo

- **Visual studio code**: como IDLE para la creación del código.
- **GitHub**: como repositorio de trabajo.
- Y otras mas, que se detallarán al finalizar la tarea.

## Estructuras de datos y funciones

- Funciones:
  - `fragmentador(*opcion*: String) -> Vec<String>`: Esta función permite validar las entradas registras por el usuario y las convierte al formato de vector la colección de palabras introducidas.
  - `entrada_texto() -> String`: Esta es una función auxiliar que permite registrar la entrada de texto por parte del usuario y retorna una cadena de String.
  - `ejecuta_binario(*opcion*:String)`esta es una funciona alterna que funciona como enlace final con el sistema operativo a la hora de ejecutar los binarios.
  - `menu_tracker_v(*binario*: String)`: Sub-menú después del main el cual ejecuta los procesos individuales según la elección de procesos dada por el usuario, en esta caso para el proceso con la entrada -v.
  - `menu_tracker_V(*binario*: String)`:Sub-menú después del main el cual ejecuta los procesos individuales según la elección de procesos dada por el usuario, en esta caso para el proceso con la entrada -V.
  - `presiona_tecla()`:Función auxiliar al proceso de -V ya que permite registrar las entradas necesarias para la ejecución de la tarea.
  - `rastreador_V(*child*: Pid)-> Vec<String>` : Función encargada del proceso de impresión de los resultados o subprocesos detonados durante la ejecución de una consulta y de la demás logística que pueda incluir, en este caso esta es la función encarga de la opción -V.
  - `rastreador_v(*child*: Pid)-> Vec<String>`: Función encargada del proceso de impresión de los resultados o subprocesos detonados durante la ejecución de una consulta y de la demás logística que pueda incluir, en este caso esta es la función encarga de la opción -v.
  - `reps_counter_hash`:Función principal para la cuenta de repeticiones en el las llamadas
  - `reps_counter_hash_verifier`:Función secundaria de `reps_counter_hash` que se encarga de verificar que no se vaya a contar 2 veces la misma llamada o llamadas con el mismo nombre
  - `reps_counter_hash_aux`:Función secundaria de `reps_counter_hash` que se encarga de contar las veces que se utilizó una llamada en específico
  - `reps_counter_hash_printer`:Función secundaria de `reps_counter_hash` que se encarga de imprimir la matriz generada sobre la incidencia de uso de las llamadas durante una ejecución.
  - `main()`:función principal del programa y coordinadora general de todas las funcionalidades.

- **Estructuras**
  - **Vectores**: Se utilizaron vectores como valores de retorno de las aplicaciones y como directorios para la manipulación y transferencia de los datos 
    - **String** 
    - **String , enteros** 


## Instrucciones para ejecutar el programa

1. compilar el proyecto.

2. Administrar el comando segun la infomacipon que desee obtener, siguiendo la sintaccions de consulta establecida.

   1. Ejemplo del formato 

      `rastreador [opciones rastreador] Prog [opciones de Prog]`

   2. En caso de haber seleccionado la opción de -V, presionar cualquier tecla para detonar cada uno de los subprocesos.

3. Observar los resultados de la ejecución.

4. Analizar las métricas o cálculos generados a raíz de la ejecución.			

## Actividades realizadas por el estudiante

| Fecha      | Hora de inicio | Hora final | Horas invertidas | Actividad realizada                                          |
| ---------- | -------------- | ---------- | ---------------- | ------------------------------------------------------------ |
| 09/08/2022 | 16:00          | 17:00      | 1:00             | Descarga y configuración del ambiente desarrollo             |
| 09/08/2022 | 17:00          | 21:00      | 4:00             | Investigación básica sobre el funcionamiento y estructura del lenguaje Rust y familiarización con el lenguaje |
| 09/08/2022 | 21:00          | 22:00      | 1:00             | Creación del repositorio de trabajo y comits iniciales       |
| 10/08/2022 | 19:00          | 21:30      | 2:30             | Desarrollo de la estructura de captura de entradas de texto y validaciones para la tarea |
| 10/08/2022 | 21:30          | 22:00      | 0:30             | Inicio de la investigación sobre como aplicar llamadas al sistema mediante la herramienta |
| 11/08/2022 | 19:00          | 22:00      | 3:00             | Continuación de la investigación sobre como aplicar llamadas al sistema mediante la herramienta |
| 12/08/2022 | 16:00          | 18:00      | 2:00             | Continuación de la investigación sobre como aplicar llamadas al sistema mediante la herramienta y experimentación sobre el uso de bibliotecas |
| 13/08/2022 | 9:00           | 11:00      | 2:00             | Actualización y redacción de los avances en la documentación |
| 13/08/2022 | 12:30          | 13:30      | 1:00             | Intentado habilitar la biblioteca Rust en mi computadora     |
| 13/08/2022 | 15:00          | 16:30      | 1:30             | Se agregaron las dependencias al sistema y experimentación   |
| 14/08/2022 | 9:00           | 11:00      | 2:00             | Etapa 1 desarrollo de código                                 |
| 15/08/2022 | 9:00           | 11:00      | 2:00             | Análisis de algoritmos para el desarrollo                    |
| 15/08/2022 | 13:00          | 16:00      | 3:00             | Etapa 2 desarrollo de código                                 |
| 16/08/2022 | 8:00           | 10:00      | 2:00             | Etapa 3 desarrollo de código(commit funcional sin la capacidad de contar las llamdas de un mismo tipo) |
| 13/08/2022 | 12:30          | 14:30      | 2:00             | Etapa 4 desarrollo de código, trabajo en el conteo de las llamadas |
| 13/08/2022 | 14:30          | 16:00      | 1:30             | Actualización parcial de la documentación                    |
| 13/08/2022 | 16:00          | 18:00      | 2:00             | Última etapa de desarrollo del código                        |
| 13/08/2022 | 18:00          | 19:00      | 1:00             | Conclusión documentación                                     |
| **Total**  |                |            | 34:00            |                                                              |

## Auto evaluación

### Estado final: 

​	El programa quedó funcionando de acuerdo a los requerimientos establecidos en la documentación aplicando todos los procedimientos y mostrando toda la información solicitada 

### Limitaciones adicionales: 

Fueron muchisimos los problemas que se presentaron durnate la ejecion y todos fueron solventados, algunos pueden ser:

1. Encontrar la bliblioteca caps de conectarse al SO y hacer el trace necesario.

2. Aprender a usar esa biblioteca.

3. Manipular ese volumen de datos en estructuras que permitieran su manipulación de forma sencilla.

4. Encontrar la forma de hacer que la opción `-V` fuera posible. En este caso quedó funcionando con la tecla `ENTER`

   

### Reporte de commits: 

### Calificación:

| Rubro                      | Calificación |
| -------------------------- | ------------ |
| Opción -v: 10 %            | 10           |
| Opción -V: 20 %            | 15           |
| Ejecución de Prog: 20 %    | 15           |
| Análisis de Syscalls: 30 % | 28           |
| Documentación: 20 %        | 18           |
| **Porcentaje final**       | 86           |

## Lecciones aprendidas: 



## Bibliografía:

- "Implementing strace in Rust". Jakob Waibel. https://jakobwaibel.com/2022/06/06/ptrace (accedido el 16 de agosto de 2022).https://github.com/upenn-cis198/homework4
- "System programming in Rust, take 2". 128nops and counting. https://carstein.github.io/2022/05/29/rust-system-programming-2.html (accedido el 16 de agosto de 2022).
- "ptrace(2) - Linux manual page". Michael Kerrisk - man7.org. https://man7.org/linux/man-pages/man2/ptrace.2.html (accedido el 16 de agosto de 2022).
- "personality(2) - Linux manual page". Michael Kerrisk - man7.org. https://man7.org/linux/man-pages/man2/personality.2.html (accedido el 16 de agosto de 2022).
- "Enlaces a Variables | El Lenguaje de Programacion Rust". Page Not Found Â· GitBook (Legacy). https://goyox86.gitbooks.io/el-libro-de-rust/content/variable-bindings.html (accedido el 16 de agosto de 2022).
