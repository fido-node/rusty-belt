pub mod codec;
pub mod helpers;
pub mod rusty {
    pub mod belt {
        include!(concat!(env!("OUT_DIR"), "/rusty.belt.rs"));
    }
}
