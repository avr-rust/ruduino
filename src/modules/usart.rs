use crate::Register;

pub trait HardwareUsart {
    /// The USART data register.
    type DataRegister: Register<T=u8>;
    /// USART control and status register A.
    type ControlRegisterA: Register<T=u8>;
    /// USART control and status register B.
    type ControlRegisterB: Register<T=u8>;
    /// USART control and status register C.
    type ControlRegisterC: Register<T=u8>;
    /// USART baud rate register.
    type BaudRateRegister: Register<T=u16>;
}
