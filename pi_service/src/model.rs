pub enum PiOutput {
    Led { pin: u16, value: i32 },
    Blink { pin: u16, millis: u32 },
}
