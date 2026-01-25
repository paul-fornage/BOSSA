#include "PortFactory.h"

std::unique_ptr<PortFactory> new_port_factory() {
    return std::make_unique<PortFactory>();
}

std::unique_ptr<SerialPort> PortFactoryBase::create_port(const std::string& name, bool is_usb) {
    return create(name, is_usb);
}

const std::string &PortFactoryBase::default_name() {
    port_name = def();
    return port_name;
}
