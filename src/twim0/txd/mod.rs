#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Data pointer"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in transmit buffer"]
pub struct MAXCNT {
    register: VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in transmit buffer"]
pub mod maxcnt;
#[doc = "Number of bytes transferred in the last transaction"]
pub struct AMOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Number of bytes transferred in the last transaction"]
pub mod amount;
#[doc = "EasyDMA list type"]
pub struct LIST {
    register: VolatileCell<u32>,
}
#[doc = "EasyDMA list type"]
pub mod list;