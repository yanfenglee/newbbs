#[macro_use]
extern crate serde_derive;

pub mod settings;
pub mod controller;
mod util;
mod service;
mod middleware;
mod domain;
mod configure;

lazy_static! {
    pub static ref RB:Rbatis={
        let mut rbatis = Rbatis::new();
        return rbatis;
    };
}