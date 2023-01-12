use sysinfo::{CpuExt, System, SystemExt};
use clearscreen;
use std::{thread,time,iter};
use std::env;

//tipo de objeto
struct UtilitysOfSystem;

//metodos del sistema
impl UtilitysOfSystem {

    //cpu monitor 
    fn cpu_monitor_intern(&self) {	

	//instanciaciones de la cpu de los objetos de la cpu
	let mut sys_monitor = System::new();
	
	loop {
	    println!("=#=#=#=#=#=#=#=#=#=#=#=#[CPU]=#=#=#=#=#=#=#=#=#=#=#=#");

	    //refresca el objeto de la cpu
	    sys_monitor.refresh_cpu();
	    
	    for cpu_dat in sys_monitor.cpus(){

		
		//imprime la carga de la cpu y modelo(brand)
		println!("[BRAND]: {:?} \n[CPU {}]: {:?}%",cpu_dat.brand(),cpu_dat.name(),cpu_dat.cpu_usage());

		 
		
	    }
	    //tiempo de refresco de la imprecion
	    thread::sleep(time::Duration::from_millis(500));
	    //refresca la terminal
	    clearscreen::clear().unwrap();
	}
	
    }
    //memory monitor
    fn memory_monitor_intern(&self) {

	//imprime el estado de la memoria
    	println!("=#=#=#=#=#=#=#=#=#=#=#=#[MEMORY]=#=#=#=#=#=#=#=#=#=#=#=#");

	println!("[MEMORY USED]:  bytes \n[MEMORY VIRTUAL USED]:  bytes",);	
	
    }

    fn fd_monitor_intern(&self) {

	//descriptor de archivos
	

	println!("[FILE DESCRIPTOR]: ",);
	
    }

    fn io_monitor_intern(&self) {

	//monitor del sistema de entrada/salida
	
	println!("[INPUT]:  \n [OUTPUT]: ",);
	
	
    }
    
}



fn main() {

    //manejo de los argumentos del main
    let args: Vec<String> = env::args().collect();
    
    //instanciacion del objeto
    let impl_objet_system = UtilitysOfSystem;
    
    //imprime la carga de la cpu
    println!("=#=#=#=#=#=#=#=#=#=#=#=#PCdata=#=#=#=#=#=#=#=#=#=#=#=#");

 

    match args[1].as_str() {
    
	"-c" => {

	    
	    //monitoreo de la cpu
	    impl_objet_system.cpu_monitor_intern();
	}

	"-m" => {

	    
	    //monitoreo de la cpu
	    impl_objet_system.memory_monitor_intern();
	    
	}

	&_ => {

	    println!("need a option that execute corectly the program");
	    println!("[HELP]: option -h for view the options");
	}
	

    }

}
