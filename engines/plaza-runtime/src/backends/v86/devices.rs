use crate::backends::v86::serial::SerialPort;

pub struct DeviceState {
    pub serial: SerialPort,
}

impl DeviceState {
    pub fn new(serial_tx: tokio::sync::mpsc::Sender<Vec<u8>>) -> Self {
        Self {
            serial: SerialPort::new(serial_tx),
        }
    }

    pub fn io_port_read8(&mut self, port: u16) -> u8 {
        match port {
            // COM1 (0x3F8 - 0x3FF)
            0x3F8..=0x3FF => self.serial.read_port(port) as u8,
            _ => 0,
        }
    }

    pub fn io_port_write8(&mut self, port: u16, value: u8) {
        match port {
            // COM1
            0x3F8..=0x3FF => self.serial.write_port(port, value as u32),
            _ => {}
        }
    }
}
