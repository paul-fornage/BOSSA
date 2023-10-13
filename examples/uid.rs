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
    let mut oliberver = lib::new_bossa_observer();
    let mut flasher = lib::new_flasher(samba.pin_mut(), device.pin_mut(), oliberver.pin_mut());

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
    println!("Unique Id: {}", info.unique_id());

    std::process::ExitCode::SUCCESS
}
