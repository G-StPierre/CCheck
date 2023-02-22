use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use clap::Parser;
use term_size;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'm', long = "mem")]
    mem_flag: bool,
    #[arg(short = 'c', long ="cpu")]
    cpu_flag: bool,
    #[arg(short = 'd', long="disk")]
    disk_flag: bool,
}


fn main() {
    let args = Cli::parse();
    let mut sys = System::new_all();
     // Cpu refresh
     sys.refresh_cpu();
    let width = term_size::dimensions().unwrap().0;
     // This is pretty bad, it assumes we will never get a None Option
    if args.mem_flag{
        mem_display(&sys, width);
    }
    if args.cpu_flag{
        cpu_display(&sys, width);
    }
    if args.disk_flag{
        disk_display(&sys, width);
    }
    if !args.mem_flag && !args.cpu_flag && !args.disk_flag{
        default_display(sys, width);
    }
}

fn default_display(sys: System, width: usize){
    disk_display(&sys, width);
    cpu_display(&sys, width);
    mem_display(&sys, width);
}

fn disk_display(sys: &System, width: usize){
    format_display(width, String::from("Disk Usage"));
    for disk in sys.disks(){
        let usage: f64 = f64::trunc((disk.available_space() as f64 / u64::pow(1024, 3) as f64) * 100.0) / 100.0;
        let total: f64 = f64::trunc((disk.total_space() as f64 / u64::pow(1024, 3) as f64) * 100.0) / 100.0;
        println!("{:?} | {:?} | {}GB remaining of {}GB", disk.name(), disk.type_(), usage, total);
    }
    format_display(width, String::new());
}

fn mem_display(sys: &System, width: usize){
    // Memory Usage = 12 chars
    format_display(width, String::from("Memory Usage"));
    let usage = f64::trunc((sys.used_memory() as f64 / u64::pow(1024, 3) as f64) * 100.0) / 100.0;
    let total = f64::trunc((sys.total_memory() as f64 / u64::pow(1024, 3) as f64) * 10.0) / 10.0;
    println!("Memory: {} / {}", usage, total);
    format_display(width, String::new());
}

fn cpu_display(sys: &System, width: usize){
    format_display(width, String::from("Cpu Usage"));
    for cpu in sys.cpus() {
        print!("{} : ", cpu.name());
        print!("{} | ", cpu.cpu_usage());
        }
    println!();
    format_display(width, String::new());

}

fn format_display(width: usize, line: String){
    let max_chars = line.len();
    if max_chars > width{
        println!("{}", line);
    } else if max_chars == 0 {
        println!("{}", "=".repeat(width));
        println!("");
    } else  {
        // let size: u32 = (width - max_chars - 2).try_into().unwrap(); // Two whitespaces either side I think?
        let size: usize = (width - max_chars - 2) / 2; // Two whitespaces either side I think?

        println!("{} {} {}", "=".repeat(size), line, "=".repeat(size));
    }
}