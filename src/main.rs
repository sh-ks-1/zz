use std::env;
use std::process;

use zzlib;

fn main()
{
    let allargs: Vec<String> = env::args().collect();
    println!("args = {:?}", allargs);

    if allargs.len() < 2
    {
        println!("실행인자가 없음");
        process::exit(0x0100);
    }
    
    let main1arg = allargs[1].to_string();

    if (main1arg == "tar") | (main1arg == "targz")
    {
        zzlib::tar_rou::roumain();
    }
    else if main1arg == "zip"
    {

    }
    else if main1arg.contains("압축")
    {

    }


}
