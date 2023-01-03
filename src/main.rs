
use perf_monitor::cpu::{ThreadStat, ProcessStat, processor_numbers};
use perf_monitor::fd::fd_count_cur;
use perf_monitor::mem::get_process_memory_info;
use perf_monitor::io::get_process_io_stats;
use clearscreen;
use std::{thread,time};
use std::env;

//tipo de objeto
struct UtilitysOfSystem{

    //atributos de para el metodo de la cpu
    core_num: Result<usize>,
    mut objet_cpu: Result<Self>,
    mut objet_thread: Result<Self>,
    //creacion del atributo para obtener informacion de la memoria
    mem_info: Result<usize>, 
    //crea un variable para alojar los datos del file descriptor
    fd_number: Result<usize>,

    //monitor del sistema de entrada/salida
    io_variable: Result<IOStats,IOStatsError>,
	
}

//metodos del sistema
impl UtilitysOfSystem {

    //cpu monitor 
    fn cpu_monitor_intern(&self) {

	//creamos una variable que contenga los datos de la frecuencia
	let usage_cpu = self.objet_cpu.cur().unwrap() * 100f64; 
	//creamos una variable que contenga los datos de los hilos
	let usage_thread = self.objet_thread.cur().unwrap() * 100f64; 

	println!("=#=#=#=#=#=#=#=#=#=#=#=#[CPU]=#=#=#=#=#=#=#=#=#=#=#=#");

	//imprime la carga de la cpu
	println!("[CPU CUR]: {:.2}% \n[THREAD]: {:.2}% \n[NUMBER CPU]:{:?} ",usage_cpu,usage_thread,self.core_num);

	
    }
    //memory monitor
    fn memory_monitor_intern(&self) {

	//imprime el estado de la memoria
	println!("=#=#=#=#=#=#=#=#=#=#=#=#[MEMORY]=#=#=#=#=#=#=#=#=#=#=#=#");

	println!("[MEMORY USED]: {:?} bytes \n[MEMORY VIRTUAL USED]: {:?} bytes",self.mem_info.resident_set_size,self.mem_info.virtual_memory_size);	
	
    }

    fn fd_monitor_intern(&self) {

	println!("[FILE DESCRIPTOR]: {:?}",self.fd_number);
	
    }

    fn io_monitor_intern(&self) {

	//monitor del sistema de entrada/salida
	let io_variable = get_process_io_stats().unwrap();
	println!("[INPUT]: {:?} \n [OUTPUT]: {:?}",self.io_variable.resident_set_size,self.io_variable.virtual_memory_size);
	
	
    }
    
}



fn main() {
    
    //crea un variable tiempo de 200 milisegundo para los refrescos de pantalla
    let mile_seg = time::Duration::from_millis(200);

    //manejo de los argumentos del main
    let args: Vec<String> = env::args().collect();
    
    //instanciacion del objeto
    let impl_objet_system = UtilitysOfSystem {

	//atributos de para el metodo de la cpu
	core_num: processor_numbers().unwrap(),
	objet_cpu: ProcessStat::cur().unwrap(),
	objet_thread: ThreadStat::cur().unwrap(),
	//creacion del atributo para obtener informacion de la memoria
	mem_info: get_process_memory_info().unwrap(), 
	//crea un variable para alojar los datos del file descriptor
	fd_number: fd_count_cur().unwrap(),

	//monitor del sistema de entrada/salida
	io_variable: get_process_io_stats().unwrap(),
	
    };
    
    //imprime la carga de la cpu
    println!("=#=#=#=#=#=#=#=#=#=#=#=#PCdata=#=#=#=#=#=#=#=#=#=#=#=#");

 

    match args[1] {
    
	"-c" => loop {

	    
	    //monitoreo de la cpu
	    impl_objet_system.cpu_monitor_intern();
	    
	    //delay de espera
	    thread::sleep(mile_seg);
	
	    //limpia la pantalla
	    clearscreen::clear().unwrap();
	}

	"-m" => loop {

	    
	    //monitoreo de la cpu
	    impl_objet_system.memory_monitor_intern();
	    
	    //delay de espera
	    thread::sleep(mile_seg);
	
	    //limpia la pantalla
	    clearscreen::clear().unwrap();
	}
	

    }

}
