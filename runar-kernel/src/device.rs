/// Contains the data structures required to interface with devices outside the cpu.
/// 
pub mod storage {

}

pub mod network {
    pub trait NetworkDevice {

    }

    pub trait WirelessDevice: NetworkDevice {

    }

}

pub mod input {
    pub trait KeyboardDevice {

    }

    pub trait PointerDevice {

    }

    pub trait TouchscreenDevice {

    }
}

pub mod display {

}

pub mod bus {
    pub trait I2CDevice {

    }

    pub trait USBDevice {

    }

    pub trait PCIDevice {

    }
}

pub mod audio {

}