use Register;

pub trait HardwareUsart {
    /// The USART data register.
    type UDR: Register<u8>;
    /// USART control and status register A.
    type UCSRA: Register<u8>;
    /// USART control and status register B.
    type UCSRB: Register<u8>;
    /// USART control and status register C.
    type UCSRC: Register<u8>;
    /// USART baud rate register.
    type UBRR: Register<u16>;
}
