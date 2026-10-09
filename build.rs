use std::env;

enum TargetOs {
    Windows,
    Linux,
    MacOs,
    Bsd,
}

impl TargetOs {
    fn from_cargo() -> Self {
        let target_os = env::var("CARGO_CFG_TARGET_OS").expect("Cargo must provide the target OS");
        match target_os.as_str() {
            "windows" => Self::Windows,
            "linux" => Self::Linux,
            "macos" => Self::MacOs,
            "freebsd" | "openbsd" => Self::Bsd,
            other => panic!("unsupported BOSSA target OS: {other}"),
        }
    }
}

fn main() {
    // Build scripts run on the host, so cfg!(target_os) does not describe the
    // platform that the C++ library must be compiled for.
    let target_os = TargetOs::from_cargo();
    let target_env =
        env::var("CARGO_CFG_TARGET_ENV").expect("Cargo must provide the target environment");

    let mut cpp_files = vec![
        "src/Applet.cpp",
        "src/BossaObserver.cpp",
        "src/D2xNvmFlash.cpp",
        "src/D5xNvmFlash.cpp",
        "src/Device.cpp",
        "src/EefcFlash.cpp",
        "src/EfcFlash.cpp",
        "src/Flash.cpp",
        "src/Flasher.cpp",
        "src/Info.cpp",
        "src/PortFactory.cpp",
        "src/Samba.cpp",
        "src/WordCopyApplet.cpp",
        "src/WordCopyArm.cpp",
    ];
    let mut h_files = vec![
        "src/Applet.h",
        "src/BossaObserver.h",
        "src/D2xNvmFlash.h",
        "src/D5xNvmFlash.h",
        "src/Device.h",
        "src/EefcFlash.h",
        "src/EfcFlash.h",
        "src/Flash.h",
        "src/Flasher.h",
        "src/Info.h",
        "src/PortFactory.h",
        "src/Samba.h",
        "src/SerialPort.h",
        "src/WordCopyApplet.h",
        "src/WordCopyArm.h",
    ];

    match target_os {
        TargetOs::Windows => {
            cpp_files.extend(["src/WinPortFactory.cpp", "src/WinSerialPort.cpp"]);
            h_files.extend(["src/WinPortFactory.h", "src/WinSerialPort.h"]);
            println!("cargo:rustc-link-lib=setupapi");
        }
        TargetOs::Linux => {
            cpp_files.extend(["src/LinuxPortFactory.cpp", "src/PosixSerialPort.cpp"]);
            h_files.extend(["src/LinuxPortFactory.h", "src/PosixSerialPort.h"]);
        }
        TargetOs::MacOs => {
            cpp_files.extend(["src/OSXPortFactory.cpp", "src/PosixSerialPort.cpp"]);
            h_files.extend(["src/OSXPortFactory.h", "src/PosixSerialPort.h"]);
        }
        TargetOs::Bsd => {
            cpp_files.extend(["src/BSDPortFactory.cpp", "src/PosixSerialPort.cpp"]);
            h_files.extend(["src/BSDPortFactory.h", "src/PosixSerialPort.h"]);
        }
    }

    let mut build = cxx_build::bridge("src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    for cpp in cpp_files {
        build.file(cpp);
        println!("cargo:rerun-if-changed={cpp}");
    }
    for h in h_files {
        println!("cargo:rerun-if-changed={h}");
    }

    if target_env == "msvc" {
        build.include("src/msvc");
        println!("cargo:rerun-if-changed=src/msvc");
    }

    build.std("c++14").compile("bossa");
}
