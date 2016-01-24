#[derive(Debug, Default)]
pub struct RegConfig {
    // TODO: Better names?
    ep: Ep,
    be: Be
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.ep = Ep::D;
        self.be = Be::BigEndian;
    }
}

// TODO: Better name?
#[derive(Debug)]
enum Ep {
    D, // TODO: Better name?
    DxxDxx, // TODO: Better name?
    RFU
}

impl Default for Ep {
    fn default() -> Ep {
        Ep::D
    }
}

// TODO: Better name?
#[derive(Debug)]
enum Be {
    LittleEndian,
    BigEndian
}

impl Default for Be {
    fn default() -> Be {
        Be::BigEndian
    }
}
