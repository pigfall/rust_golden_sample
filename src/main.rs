#[rust_pg_macro::expand(logger)]
fn raw_main() {
    println!("raw main");
    pf_age_macro_rule::debug!();

}
