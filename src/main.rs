use sysinfo::{CpuExt, System, SystemExt,NetworkExt,NetworkData, NetworksExt, ProcessExt};
use clearscreen;
use std::{thread,time,iter};
use std::env;
use colored::Colorize;

//tipo de objeto
struct UtilitysOfSystem{

    sys_mon_all: System,
    
}

//metodos del sistema
impl UtilitysOfSystem {

    //nonitoreo de la red
    fn network_minitor_intern(&self,b: &mut System) {

	loop {

	    println!("=#=#=#=#=#=#=#=#=#=#=#=#[NETWORK]=#=#=#=#=#=#=#=#=#=#=#=#");
	    //transmicion de los datos 
	    for (name_info, data_network) in self.sys_mon_all.networks() {

		println!("[NETWORKS INERFACE]: {:?}",name_info);
		println!("[DOWNLOAD]: {:?} Bytes",data_network.received());
		println!("[UPLOAD]: {:?} Bytes",data_network.transmitted());

		//refresca las interface de la red
		let net = b.networks_mut();
		net.refresh();
		}

	    //tiempo de refresco de la imprecion.
	    thread::sleep(time::Duration::from_millis(500));
	    //refresca la terminal
	    clearscreen::clear().unwrap();
	}
	
    }

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

	println!("[TOTAL MEMORY]: {} Bytes",self.sys_mon_all.total_memory());
	println!("[USED  MEMORY]: {} Bytes",self.sys_mon_all.used_memory());
	println!("[TOTAL  SWAP ]: {} Bytes",self.sys_mon_all.total_swap());
	println!("[ USED  SWAP ]: {} Bytes",self.sys_mon_all.used_swap());
	
	
		
    }

    

    fn disk_monitor_intern(&self) {

	//monitor del sistema de entrada/salida
	
	println!("[INPUT]:  \n [OUTPUT]: ",);
	
	
    }
    
}



fn main() {

    //manejo de los argumentos del main
    let args: Vec<String> = env::args().collect();

    let mut sys_ma_2 = System::new_all();
    
    //instanciacion del objeto
    let impl_objet_system = UtilitysOfSystem{

	sys_mon_all: System::new_all()
	
    };
    
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

	"-n" => {
	    //monitor de la memoria
	    impl_objet_system.network_minitor_intern(&mut sys_ma_2);

	}

	"-d" => {

	    //monitor del disco
	    impl_objet_system.disk_monitor_intern();
	    
	}

	&_ => {

	    println!("need a option that execute corectly the program");
	    println!("[HELP]: option -h for view the options");
	}
	

    }

}
