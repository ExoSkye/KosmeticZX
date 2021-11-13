use crate::bus::Bus;

use std::sync::*;

pub struct Cpu {
    bus: Arc<RwLock<Bus>>
}

impl Cpu {

}