pub struct SerialPort {
    tx: tokio::sync::mpsc::Sender<Vec<u8>>,
}

impl SerialPort {
    pub fn new(tx: tokio::sync::mpsc::Sender<Vec<u8>>) -> Self {
        Self { tx }
    }
    
    pub fn read_port(&mut self, port: u16) -> u32 {
        if port == 0x3FD {
            // Line Status Register (LSR)
            // Bit 5 (0x20) indicates Transmitter Holding Register Empty
            // Bit 6 (0x40) indicates Transmitter Empty
            return 0x20 | 0x40;
        }
        0
    }
    
    pub fn write_port(&mut self, port: u16, value: u32) {
        if port == 0x3F8 {
            // Transmit Data Register
            let byte = value as u8;
            let _ = self.tx.try_send(vec![byte]);
        }
    }
}
