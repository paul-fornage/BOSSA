use bossa::lib;
use cxx::let_cxx_string;

pub fn main() -> std::process::ExitCode {
    let mut samba = lib::new_samba();
    let mut port_factory = lib::new_port_factory();

    // Use default port (e.g. /dev/ttyACM0 on linux)
    let_cxx_string!(port = port_factory.pin_mut().default_name().to_string());

    // Attempt to connect to the usb port
    if !samba
        .pin_mut()
        .connect(port_factory.pin_mut().create_port(&port, true), 115200)
    {
        println!("Failed to connect to port: {}", port);
        return std::process::ExitCode::FAILURE;
    }

    // Setup device connection
    let mut device = lib::new_device(samba.pin_mut());
    device.pin_mut().create();
    let log_observer_callback = bossa::ObserverCallback(log_observer_fn);
    let prog_observer_callback = bossa::ProgressCallback(prog_observer_fn);
    let mut observer = unsafe { lib::new_bossa_observer_with_progress(log_observer_callback, prog_observer_callback) };
    let mut flasher = lib::new_flasher(samba.pin_mut(), device.pin_mut(), observer.pin_mut());

    // Get FlasherInfo
    let mut flasher_info = lib::new_flasher_info();
    flasher.pin_mut().info(flasher_info.pin_mut());
    // Print FlasherInfo using C++ api
    flasher_info.pin_mut().print();

    // Convert to FlasherInfoRs
    let mut info = lib::FlasherInfoRs::default();
    lib::flasherinfo2flasherinfors(&flasher_info.pin_mut(), &mut info);
    // Print struct and unique id
    println!("{:#?}", info);
    println!("{}", info.info());


    std::process::ExitCode::SUCCESS
}

extern "C" fn log_observer_fn(message: &cxx::CxxString) {
    match message.to_str(){
        Ok(str) => {eprint!("{}", str)},
        _ => {eprint!("Failed to convert message to utf8 string");}
    }
}

extern "C" fn prog_observer_fn(current: i32, total: i32) {
    const PROGRESS_BAR_WIDTH: usize = 32;
    let mut progress = current as f64 / total as f64;
    if progress < 0.0 { progress = 0.0; }
    if progress > 1.0 { progress = 1.0; }

    let percent = progress * 100.0;

    let progress_bar_len = (progress * PROGRESS_BAR_WIDTH as f64).round() as usize;

    let mut progress_bar: [u8; PROGRESS_BAR_WIDTH] = [b' '; PROGRESS_BAR_WIDTH];
    progress_bar[..progress_bar_len].fill(b'=');
    // Safety: progress_bar is initialized with '=' and ' '
    let progress_str = str::from_utf8(&progress_bar).unwrap();

    // bossa c prints use eprint, and otherwise they desync from OS buffering
    eprint!("\r[{progress_str}] {percent:>6.2}% ({current}/{total})");
}
