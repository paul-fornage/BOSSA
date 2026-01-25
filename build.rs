fn main() {
    let cpp_files = vec![
        "src/lib/Applet.cpp",
        "src/gui/BossaObserver.cpp",
        "src/lib/D2xNvmFlash.cpp",
        "src/lib/D5xNvmFlash.cpp",
        "src/lib/Device.cpp",
        "src/lib/EefcFlash.cpp",
        "src/lib/EfcFlash.cpp",
        "src/lib/Flash.cpp",
        "src/lib/Flasher.cpp",
        "src/lib/Info.cpp",
        #[cfg(target_os = "linux")]
        "src/lib/serial/LinuxPortFactory.cpp",
        #[cfg(target_os = "macos")]
        "src/lib/serial/OSXPortFactory.cpp",
        "src/lib/serial/PortFactory.cpp",
        #[cfg(target_family = "unix")]
        "src/lib/serial/PosixSerialPort.cpp",
        "src/lib/Samba.cpp",
        #[cfg(target_os = "windows")]
        "src/lib/serial/WinPortFactory.cpp",
        #[cfg(target_os = "windows")]
        "src/lib/serial/WinSerialPort.cpp",
        "src/lib/WordCopyApplet.cpp",
        "src/lib/WordCopyArm.cpp",
    ];
    let h_files = vec![
        "src/lib/Applet.h",
        "src/gui/BossaObserver.h",
        "src/lib/D2xNvmFlash.h",
        "src/lib/D5xNvmFlash.h",
        "src/lib/Device.h",
        "src/lib/EefcFlash.h",
        "src/lib/EfcFlash.h",
        "src/lib/Flash.h",
        "src/lib/Flasher.h",
        "src/lib/Info.h",
        #[cfg(target_os = "linux")]
        "src/lib/serial/LinuxPortFactory.h",
        #[cfg(target_os = "macos")]
        "src/lib/serial/OSXPortFactory.h",
        "src/lib/serial/PortFactory.h",
        #[cfg(target_family = "unix")]
        "src/lib/serial/PosixSerialPort.h",
        "src/lib/Samba.h",
        "src/lib/serial/SerialPort.h",
        #[cfg(target_os = "windows")]
        "src/lib/serial/WinPortFactory.h",
        #[cfg(target_os = "windows")]
        "src/lib/serial/WinSerialPort.h",
        "src/lib/WordCopyApplet.h",
        "src/lib/WordCopyArm.h",
    ];
    let rs_files = vec!["src/lib.rs"];

    // Setup cxx build
    //  cpp files to watch + build
    //  h files to watch
    //  rs files to watch
    let mut build = cxx_build::bridges(rs_files.clone());
    for cpp in cpp_files {
        build.file(cpp);
        println!("cargo:rerun-if-changed={}", cpp);
    }
    for h in h_files {
        println!("cargo:rerun-if-changed={}", h);
    }
    for rs in rs_files {
        println!("cargo:rerun-if-changed={}", rs);
    }

    // Extra includes needed for msvc environments
    #[cfg(target_env = "msvc")]
    build.include("src/msvc");

    build
        .include("src")
        .include("src/lib")
        .include("src/lib/serial")
        .include("src/gui");

    build.flag_if_supported("-std=c++14").compile("bossa");
}
