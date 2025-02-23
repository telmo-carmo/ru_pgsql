fn main() {
    let mut pargs = pico_args::Arguments::from_env();

    let port = pargs.opt_value_from_str("-port").unwrap().unwrap_or(5432);
    let user= pargs.opt_value_from_str("-usr").unwrap().unwrap_or_else(|| "scott".to_string());
    let pwd= pargs.opt_value_from_str("-pwd").unwrap().unwrap_or_else(|| "tiger".to_string());
    let db= pargs.opt_value_from_str("-db").unwrap().unwrap_or_else(|| "scottdb".to_string());
    let host= pargs.opt_value_from_str("-host").unwrap().unwrap_or_else(|| "localhost".to_string());
    let remaining_args = pargs.finish();

    println!("args: {:?}",remaining_args);
}
