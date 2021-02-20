use std::env;
use std::process;
use std::io::Write;

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
        zzlib::zip_rou::roumain();
        
    }
    else if main1arg.contains("압축")
    {
        loop
        {
            println!("");
            println!("1) tar/targz 압축, 압축풀기");
            println!("2) zip/unzip 압축, 압축풀기");
            print!("= ");
            let _ = std::io::stdout().flush();
            
            let mut userinput = String::new();
            std::io::stdin().read_line(&mut userinput).unwrap();
            userinput = userinput.trim().to_string();
            
            if userinput == "1"
            {
                zzlib::tar_rou::roumain();
                break
            }
            else if userinput == "2"
            {
                break
            }
        }
    }

}
