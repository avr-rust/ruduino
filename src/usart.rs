use Register;

pub trait HardwareUsart {
    /// The USART data register.
    type DataRegister: Register<u8>;
    /// USART control and status register A.
    type ControlRegisterA: Register<u8>;
    /// USART control and status register B.
    type ControlRegisterB: Register<u8>;
    /// USART control and status register C.
    type ControlRegisterC: Register<u8>;
    /// USART baud rate register.
    type BaudRateRegister: Register<u16>;
}
