// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use libc::{c_int};

extern "C" {
    fn init_cap_dns() -> c_int;
}

pub fn init() -> Result<(), &'static str> {

    unsafe {
        let errcode = init_cap_dns();
        match errcode {
            0 => { Ok(()) } // Do nothing
	        1 => { Err("Failed to initialise casper") }
            2 => { Err("Failed to open service") } 
	        _ => { Err("Unknown Error") }
        }            
    }
}
