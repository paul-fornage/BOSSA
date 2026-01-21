use bossa::lib;
use clap::Parser;
use cxx::{let_cxx_string, CxxString};
use flexi_logger::Logger;
use log::{debug, error};
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

/// Parse integer with support for hex (0x) and octal (0) prefixes, like strtol with base 0
fn parse_int_with_prefix(s: &str) -> Result<u32, String> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).map_err(|e| format!("invalid hex number: {}", e))
    } else if s.len() > 1 && s.starts_with('0') && s.chars().all(|c| c.is_ascii_digit()) {
        u32::from_str_radix(&s[1..], 8).map_err(|e| format!("invalid octal number: {}", e))
    } else {
        s.parse::<u32>().map_err(|e| format!("invalid decimal number: {}", e))
    }
}

/// Parse u16 with support for hex (0x) and octal (0) prefixes
fn parse_u16_with_prefix(s: &str) -> Result<u16, String> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u16::from_str_radix(hex, 16).map_err(|e| format!("invalid hex number: {}", e))
    } else if s.len() > 1 && s.starts_with('0') && s.chars().all(|c| c.is_ascii_digit()) {
        u16::from_str_radix(&s[1..], 8).map_err(|e| format!("invalid octal number: {}", e))
    } else {
        s.parse::<u16>().map_err(|e| format!("invalid decimal number: {}", e))
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Basic Open Source SAM-BA Application (BOSSA)
Flash programmer for Atmel SAM devices.
Copyright (C) 2009-2020 ShumaTech (http://www.shumatech.com/)
Copyright (C) 2018-2023 Jacob Alexander (haata@kiibohd.com)

Examples:
  bossac -e -w -v -b image.bin   # Erase flash, write flash with image.bin,
                                 # verify the write, and set boot from flash
  bossac -r0x10000 image.bin     # Read 64KB from flash and store in image.bin",
    long_about = None
)]
struct Args {
    /// .bin file to operate on
    file: Option<PathBuf>,

    /// erase the entire flash starting at the offset
    #[arg(short, long, default_value = "false")]
    erase: bool,

    /// write FILE to the flash; accelerated when combined with erase option
    #[arg(short, long, requires = "file", default_value = "false")]
    write: bool,

    /// read SIZE from flash and store in FILE;
    /// read entire flash if SIZE not specified
    #[arg(
        short,
        long,
        require_equals = true,
        value_name = "SIZE",
        num_args = 0..=1,
        requires = "file",
        default_value = None,
        default_missing_value = "0"
    )]
    read: Option<u32>,

    /// verify FILE matches flash contents
    #[arg(short, long, requires = "file", default_value = "false")]
    verify: bool,

    /// start erase/write/read/verify operation at flash OFFSET;
    /// OFFSET must be aligned to a flash page boundary
    #[arg(short, long, default_value_t = 0)]
    offset: u32,

    /// use serial PORT to communicate to device;
    /// default behavior is to use first serial port
    #[arg(short, long, default_value = None)]
    port: Option<String>,

    /// boot from ROM if BOOL is false;
    /// boot from FLASH if BOOL is true [default];
    /// option is ignored on unsupported devices
    #[arg(
        short,
        long,
        require_equals = true,
        value_name = "BOOL",
        num_args = 0..=1,
        default_value = None,
        default_missing_value = "true"
    )]
    boot: Option<bool>,

    /// no brownout detection if BOOL is 0;
    /// brownout detection is on if BOOL is 1 [default]
    #[arg(
        short = 'c',
        long,
        require_equals = true,
        value_name = "BOOL",
        num_args = 0..=1,
        default_value = None,
        default_missing_value = "true"
    )]
    bod: Option<bool>,

    /// no brownout reset if BOOL is 0;
    /// brownout reset is on if BOOL is 1 [default]
    #[arg(
        short = 't',
        long,
        require_equals = true,
        value_name = "BOOL",
        num_args = 0..=1,
        default_value = None,
        default_missing_value = "true"
    )]
    bor: Option<bool>,

    /// lock the flash REGION as a comma-separated list;
    /// lock all if not given [default]
    #[arg(
        short,
        long,
        value_name = "REGION",
        value_delimiter = ',',
        num_args = 0..,
        value_parser = clap::value_parser!(u16).range(0..),
        default_value = None
    )]
    lock: Option<Vec<u16>>,

    /// unlock the flash REGION as a comma-separated list;
    /// unlock all if not given [default]
    #[arg(
        short,
        long,
        value_name = "REGION",
        value_delimiter = ',',
        num_args = 0..,
        value_parser = clap::value_parser!(u16).range(0..),
        default_value = None
    )]
    unlock: Option<Vec<u16>>,

    /// set the flash security flag
    #[arg(short, long, default_value = "false")]
    security: bool,

    /// display device information
    #[arg(short, long, default_value = "false")]
    info: bool,

    /// print debug messages
    #[arg(short, long, default_value = "false")]
    debug: bool,

    /// force serial port detection to USB if BOOL is 1 [default] or to RS-232 if BOOL is 0
    #[arg(short = 'U', long, value_name = "BOOL", default_value = "true")]
    usb_port: bool,

    /// reset CPU (if supported)
    #[arg(short = 'R', long, default_value = "false")]
    reset: bool,

    /// erase and reset via Arduino 1200 baud hack
    #[arg(short = 'a', long, default_value = "false")]
    arduino_erase: bool,
}

/// Lite logging setup
fn setup_logging_lite() -> Result<(), ()> {
    match Logger::try_with_env_or_str("")
        .unwrap()
        .format(flexi_logger::colored_default_format)
        .format_for_files(flexi_logger::colored_detailed_format)
        .duplicate_to_stderr(flexi_logger::Duplicate::All)
        .start()
    {
        Err(_) => Err(()),
        Ok(_) => Ok(()),
    }
}

pub fn main() -> std::process::ExitCode {
    let args = Args::parse();
    setup_logging_lite().unwrap();

    let mut samba = lib::new_samba();
    let mut port_factory = lib::new_port_factory();

    if args.debug {
        debug!("Debug mode enabled");
        samba.pin_mut().setDebug(true);
    }

    let port = if let Some(port) = args.port {
        port
    } else {
        debug!("No port specified, using default");
        port_factory.pin_mut().default_name().to_string()
    };
    debug!("Connecting to port: {}", port);

    let_cxx_string!(port = port);
    if args.arduino_erase {
        debug!("Arduino erase");
        let mut port = port_factory.pin_mut().create_port(&port, args.usb_port);

        println!("Arduino 1200 baud reset");
        if !port
            .pin_mut()
            .open(1200, 8, lib::Parity::ParityNone, lib::StopBit::StopBitOne)
        {
            error!("Failed to open port at 1200bps");
            return std::process::ExitCode::FAILURE;
        }

        port.pin_mut().setRTS(true);
        port.pin_mut().setDTR(false);
        port.pin_mut().close();

        // wait for chip to reboot and USB prot to re-appear
        if args.debug {
            println!("Arduino reset done");
        }
    }

    // Attempt to connect to the port
    if !samba.pin_mut().connect(
        port_factory.pin_mut().create_port(&port, args.usb_port),
        115200,
    ) {
        error!("Failed to connect to port: {}", port);
        return std::process::ExitCode::FAILURE;
    }

    // Setup device connection
    let mut device = lib::new_device(samba.pin_mut());
    device.pin_mut().create();

    let log_observer = bossa::ObserverCallback(log_observer_fn);
    let prog_observer_callback = bossa::ProgressCallback(prog_observer_fn);
    let mut observer = unsafe { lib::new_bossa_observer_with_progress(log_observer, prog_observer_callback) };
    let mut flasher = lib::new_flasher(samba.pin_mut(), device.pin_mut(), observer.pin_mut());

    if args.info {
        debug!("Printing device info");
        let mut flasher_info = lib::new_flasher_info();
        flasher.pin_mut().info(flasher_info.pin_mut());
        flasher_info.pin_mut().print();
    }

    if let Some(regions) = args.unlock {
        if regions.is_empty() {
            debug!("Unlocking all regions");
            let_cxx_string!(regions = "");
            flasher.pin_mut().lock(&regions, false);
        } else {
            debug!("Unlocking regions: {:?}", regions);
            let_cxx_string!(
                regions = regions
                    .iter()
                    .map(|region| region.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            flasher.pin_mut().lock(&regions, false);
        }
    }

    if args.erase {
        debug!("Erasing flash at offset: {}", args.offset);
        let start = Instant::now();
        flasher.pin_mut().erase(args.offset);
        println!("Done in {:?}", start.elapsed());
    }

    // Convert PathBuf to const char* (XXX hopefully cross-platform)
    let mut path_buf = Vec::new();
    if let Some(path) = args.file {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            path_buf.extend(path.as_os_str().as_bytes());
            path_buf.push(0);
        }

        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStrExt;
            path_buf.extend(
                path.as_os_str()
                    .encode_wide()
                    .chain(Some(0))
                    .map(|b| {
                        let b = b.to_ne_bytes();
                        b.get(0).map(|s| *s).into_iter().chain(b.get(1).map(|s| *s))
                    })
                    .flatten(),
            );
        }
    }

    if args.write {
        debug!(
            "Writing flash with {:?} at offset: {}",
            path_buf, args.offset
        );
        let start = Instant::now();
        unsafe {
            flasher
                .pin_mut()
                .write(path_buf.as_ptr() as *const i8, args.offset);
        }
        println!("\nDone in {:?}", start.elapsed());
    }

    if args.verify {
        let mut page_errors = 0;
        let mut total_errors = 0;

        debug!(
            "Verifying flash with {:?} at offset: {}",
            path_buf, args.offset
        );
        let start = Instant::now();
        if !unsafe {
            flasher.pin_mut().verify(
                path_buf.as_ptr() as *const i8,
                &mut page_errors,
                &mut total_errors,
                args.offset,
            )
        } {
            error!("Verify failed {:?}", start.elapsed());
            error!("Page errors: {}", page_errors);
            error!("Byte errors: {}", total_errors);
            return std::process::ExitCode::FAILURE;
        }
        println!("\nDone in {:?}", start.elapsed());
    }

    if let Some(size) = args.read {
        debug!(
            "Reading flash into {:?} at offset: {}",
            path_buf, args.offset
        );
        let start = Instant::now();
        unsafe {
            flasher
                .pin_mut()
                .read(path_buf.as_ptr() as *const i8, size, args.offset);
        }
        println!("\nDone in {:?}", start.elapsed());
    }

    if let Some(boot) = args.boot {
        if boot {
            println!("Setting boot: Flash");
        } else {
            println!("Setting boot: ROM");
        }
        flasher.pin_mut().setBootFlash(boot);
    }

    if let Some(brownout) = args.bod {
        println!("Setting brownout detect: {}", brownout);
        flasher.pin_mut().setBod(brownout);
    }

    if let Some(brownout) = args.bor {
        println!("Setting brownout reset: {}", brownout);
        flasher.pin_mut().setBor(brownout);
    }

    if args.security {
        println!("Set security");
        flasher.pin_mut().setSecurity();
    }

    if let Some(regions) = args.lock {
        if regions.is_empty() {
            debug!("Locking all regions");
            let_cxx_string!(regions = "");
            flasher.pin_mut().lock(&regions, true);
        } else {
            debug!("Locking regions: {:?}", regions);
            let_cxx_string!(
                regions = regions
                    .iter()
                    .map(|region| region.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            flasher.pin_mut().lock(&regions, true);
        }
    }

    debug!("Write options");
    flasher.pin_mut().writeOptions();

    if args.reset {
        debug!("Reset");
        flasher.pin_mut().reset();
    }

    debug!("Done");
    std::process::ExitCode::SUCCESS
}



extern "C" fn log_observer_fn(message: &CxxString) {
    match message.to_str(){
        Ok(str) => {debug!("{}", str)},
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
