#[macro_use]
extern crate serde_derive;

mod subscription;

fn main() {
    // let pf_cached_loc = "/home/amichaud/.cache/puckfetcher/puckcache";

    let sub = subscription::Subscription::new("testurl", "testname", None);
    println!("reg sub {:?}", sub);

    let s = subscription::serialize(&sub);
    println!("encoded {:?}", s);

    let re_sub = subscription::deserialize(&s);
    println!("decoded {:?}", re_sub);
}
