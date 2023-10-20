#[cxx::bridge]
pub mod lib {
    #[repr(u32)]
    enum Parity {
        ParityNone,
        ParityOdd,
        ParityEven,
    }

    #[repr(u32)]
    enum StopBit {
        StopBitOne,
        StopBitOneFive,
        StopBitTwo,
    }

    #[derive(Debug, Clone, Default)]
    pub struct FlasherInfoRs {
        name: String,
        chipId: u32,
        extChipId: u32,
        version: String,
        address: u32,
        numPages: u32,
        pageSize: u32,
        totalSize: u32,
        numPlanes: u32,

        security: bool,
        bootFlash: bool,
        bod: bool,
        bor: bool,

        canBootFlash: bool,
        canBod: bool,
        canBor: bool,
        canChipErase: bool,
        canWriteBuffer: bool,
        canChecksumBuffer: bool,

        lockRegions: Vec<bool>,
        uniqueId: Vec<u32>,
    }

    unsafe extern "C++" {
        include!("bossa/src/BossaObserver.h");
        include!("bossa/src/Device.h");
        include!("bossa/src/Flash.h");
        include!("bossa/src/Flasher.h");
        include!("bossa/src/Info.h");
        include!("bossa/src/PortFactory.h");
        include!("bossa/src/Samba.h");
        include!("bossa/src/SerialPort.h");

        /* SerialPort */
        type SerialPort;

        type Parity;
        type StopBit;

        fn open(
            self: Pin<&mut SerialPort>,
            baud: i32,
            data: i32,
            parity: Parity,
            stop: StopBit,
        ) -> bool;
        fn close(self: Pin<&mut SerialPort>);
        fn setDTR(self: Pin<&mut SerialPort>, dtr: bool);
        fn setRTS(self: Pin<&mut SerialPort>, rts: bool);

        /* PortFactory */
        type PortFactory;

        fn new_port_factory() -> UniquePtr<PortFactory>;

        fn default_name(self: Pin<&mut PortFactory>) -> &CxxString;

        fn create_port(
            self: Pin<&mut PortFactory>,
            name: &CxxString,
            is_usb: bool,
        ) -> UniquePtr<SerialPort>;

        /* Device */
        type Device;

        fn new_device(samba: Pin<&mut Samba>) -> UniquePtr<Device>;

        fn create(self: Pin<&mut Device>);
        fn getFlash(self: Pin<&mut Device>) -> &UniquePtr<Flash>;

        /* FlasherObserver */
        type FlasherObserver;

        fn new_bossa_observer() -> UniquePtr<FlasherObserver>;

        /* Flash */
        type Flash;

        /* Flasher */
        type Flasher;

        fn new_flasher(
            samba: Pin<&mut Samba>,
            device: Pin<&mut Device>,
            observer: Pin<&mut FlasherObserver>,
        ) -> UniquePtr<Flasher>;

        fn erase(self: Pin<&mut Flasher>, foffset: u32);
        /// # Safety
        unsafe fn write(self: Pin<&mut Flasher>, filename: *const c_char, foffset: u32);
        /// # Safety
        unsafe fn verify(
            self: Pin<&mut Flasher>,
            filename: *const c_char,
            pageErrors: &mut u32,
            totalErrors: &mut u32,
            foffset: u32,
        ) -> bool;
        /// # Safety
        unsafe fn read(self: Pin<&mut Flasher>, filename: *const c_char, fsize: u32, foffset: u32);
        fn lock(self: Pin<&mut Flasher>, regionArg: &CxxString, enable: bool);
        fn info(self: Pin<&mut Flasher>, info: Pin<&mut FlasherInfo>);

        fn setBod(self: Pin<&mut Flasher>, enable: bool);
        fn setBootFlash(self: Pin<&mut Flasher>, enable: bool);
        fn setBor(self: Pin<&mut Flasher>, enable: bool);
        fn setSecurity(self: Pin<&mut Flasher>);

        fn writeOptions(self: Pin<&mut Flasher>);
        fn reset(self: Pin<&mut Flasher>);

        /* FlasherInfo */
        type FlasherInfo;

        fn print(self: Pin<&mut FlasherInfo>);

        fn new_flasher_info() -> UniquePtr<FlasherInfo>;

        /* FlasherInfoRs */
        fn flasherinfo2flasherinfors(info: &FlasherInfo, rsinfo: &mut FlasherInfoRs);

        /* Samba */
        type Samba;

        pub fn new_samba() -> UniquePtr<Samba>;

        fn connect(self: Pin<&mut Samba>, port: UniquePtr<SerialPort>, bps: i32) -> bool;

        fn setDebug(self: Pin<&mut Samba>, debug: bool);
    }
}

impl lib::FlasherInfoRs {
    /// Print unique id in the same format as bossa does
    pub fn unique_id(&self) -> String {
        let mut id = String::new();
        for i in 0..self.uniqueId.len() {
            id.push_str(&format!("{:08x}", self.uniqueId[i]));
        }
        id
    }

    /// Info as the same format bossa prints
    pub fn info(&self) -> String {
        let mut out = String::new();
        let align = 12;
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Device",
                value = self.name
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Version",
                value = self.version
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value:#x}\n",
                align = align,
                key = "Address",
                value = self.address
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Pages",
                value = self.numPages
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value} bytes\n",
                align = align,
                key = "Page Size",
                value = self.pageSize,
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value} bytes\n",
                align = align,
                key = "Total Size",
                value = self.totalSize
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Planes",
                value = self.numPlanes
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Lock Regions",
                value = self.lockRegions.len()
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Locked",
                value = self.locked_regions(),
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Security",
                value = self.security
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Boot Flash",
                value = self.bootFlash
            )
            .as_str(),
        );
        out.push_str(
            format!(
                "{key:<align$} : {value}\n",
                align = align,
                key = "Unique Id",
                value = self.unique_id()
            )
            .as_str(),
        );
        out
    }

    fn locked_regions(&self) -> String {
        let mut out = String::new();
        let mut any_locked = false;
        for (i, locked) in self.lockRegions.iter().enumerate() {
            if *locked {
                if !any_locked {
                    any_locked = true;
                    out.push_str(format!("{}", i).as_str());
                } else {
                    out.push_str(format!(",{}", i).as_str());
                }
            }
        }

        // No regions locked
        if !any_locked {
            out = "none".to_string();
        }
        out
    }
}
