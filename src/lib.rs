#[cxx::bridge]
pub mod bossa {
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

    unsafe extern "C++" {
        include!("bossa/src/BossaObserver.h");
        include!("bossa/src/Device.h");
        include!("bossa/src/Flash.h");
        include!("bossa/src/Flasher.h");
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

        /* Samba */
        type Samba;

        pub fn new_samba() -> UniquePtr<Samba>;

        fn connect(self: Pin<&mut Samba>, port: UniquePtr<SerialPort>, bps: i32) -> bool;

        fn setDebug(self: Pin<&mut Samba>, debug: bool);
    }
}
