
use rbatis::rbatis::Rbatis;

lazy_static! {
    pub static ref RB: Rbatis = {
        let rbatis = Rbatis::new();
        return rbatis;
    };
}