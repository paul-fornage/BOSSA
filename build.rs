fn main() {
    let cpp_files = vec![
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
        #[cfg(target_os = "linux")]
        "src/LinuxPortFactory.cpp",
        #[cfg(target_os = "macos")]
        "src/OSXPortFactory.cpp",
        "src/PortFactory.cpp",
        #[cfg(target_family = "unix")]
        "src/PosixSerialPort.cpp",
        "src/Samba.cpp",
        #[cfg(target_os = "windows")]
        "src/WinPortFactory.cpp",
        #[cfg(target_os = "windows")]
        "src/WinSerialPort.cpp",
        "src/WordCopyApplet.cpp",
        "src/WordCopyArm.cpp",
    ];
    let h_files = vec![
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
        #[cfg(target_os = "linux")]
        "src/LinuxPortFactory.h",
        #[cfg(target_os = "macos")]
        "src/OSXPortFactory.h",
        "src/PortFactory.h",
        #[cfg(target_family = "unix")]
        "src/PosixSerialPort.h",
        "src/Samba.h",
        "src/SerialPort.h",
        #[cfg(target_os = "windows")]
        "src/WinPortFactory.h",
        #[cfg(target_os = "windows")]
        "src/WinSerialPort.h",
        "src/WordCopyApplet.h",
        "src/WordCopyArm.h",
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

    build.flag_if_supported("-std=c++14").compile("bossa");
}
