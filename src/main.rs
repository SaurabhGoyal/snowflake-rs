use snowflake_uid::{Config, Generator};

fn main() {
    let cfg = Config::default();
    cfg.pprint();
    let mut gen = Generator::from(cfg, 12);
    let mut i = 0;
    loop {
        let id = gen.get();
        println!("Id #{i}: {id}");
        i += 1;
        if i > 20 {
            break;
        }
    }
}
