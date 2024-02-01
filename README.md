# FlySky Receiver  
> Receive FlySky signals

This project uses embedded rust and RTIC to receive throttle data from the FlySky Receiver + Transmitter.

## Project Overview

The FlySky Receiver sends data packets consisting of 32 bytes. The structure of each packet is as follows:

- **Byte 1:** Protocol length
- **Byte 2:** Command code
- **Bytes 3-31:** Channel data (every pair of bytes corresponds to a channel)
- **Byte 31:** Checksum

### Understanding the Code

Within the `main.rs` file, the core functionality for processing data from the FlySky Receiver is orchestrated. The UART and DMA is configured to allow communication between the STM32 and the FlySky Receiver . The packet parsing logic extracts pertinent information from the received data packet, adhering to the predefined structure that designates the first byte as the protocol length, the second as the command code, and the subsequent bytes as channel data. 

### Resources

- https://dronebotworkshop.com/radio-control-arduino-car/
- https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/rtic-serial-dma-rx-idle.rs
