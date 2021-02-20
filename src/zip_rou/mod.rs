// zip과 unzip을 이용한 압축

use termion::{color,style};

mod zip1;
mod zip2;
mod zip3;
mod zip4;

pub fn roumain() -> ()
{
    let mut base = BaseZip::new();
    base.whattodo = zip1::zip_or_unzip();
    
    match base.whattodo
    {
        Todo::Zip =>
        {
            base.comp_level = zip4::complev();
        },
        _ => {}
    }

    base.target_zip = zip2::targetfile(&base.whattodo);
    base.target_ex = zip3::target_ex(&base.whattodo);

    
    let mut commends: Vec<String> = Vec::new();

    {
        let mut zxc = String::new();
        match base.whattodo
        {
            Todo::Zip => zxc.push_str("-r"),
            Todo::Showlist => zxc.push_str("-l"),
            _ => {}
        };
        commends.push(zxc);
    }

    // 압축레벨 추가
    match base.whattodo
    {
        Todo::Zip => 
        {
            let levelstr: String = format!("-{}",base.comp_level);
            commends.push(levelstr);
        },
        _ => {}
    };

    commends.push(base.target_zip.to_str().unwrap().to_string());

    match base.whattodo
    {
        Todo::Unzip =>
        {
            commends.push("-d".to_string());
        },
        _ => {}
    };

    for iitem in base.target_ex.iter()
    {
        commends.push(iitem.to_str().unwrap().to_string());
    }

    println!("");
    
    let allcomand = commends.join(" ");
    
    match base.whattodo
    {
        Todo::Unzip =>
        {
            println!("{}{}실행할 커맨드: unzip {}{}", color::Bg(color::White), color::Fg(color::Red), allcomand, style::Reset);
        },
        Todo::Showlist =>
        {
            println!("{}{}실행할 커맨드: unzip {}{}", color::Bg(color::White), color::Fg(color::Red), allcomand, style::Reset);
        }
        _ =>
        {
            println!("{}{}실행할 커맨드: zip {}{}", color::Bg(color::White), color::Fg(color::Red), allcomand, style::Reset);
        }
    };

    println!("");

    match base.whattodo
    {
        Todo::Unzip =>
        {
            let mut shcom = std::process::Command::new("unzip");
            shcom.args(&commends);
            shcom.status().expect("command 실행 실패");
        },
        Todo::Showlist =>
        {
            let mut shcom = std::process::Command::new("unzip");
            shcom.args(&commends);
            shcom.status().expect("command 실행 실패");
        },
        _ =>
        {
            let mut shcom = std::process::Command::new("zip");
            shcom.args(&commends);
            shcom.status().expect("command 실행 실패");
        }
    };
}

#[derive(Debug)]
pub enum Todo
{
    None,
    Zip,
    Unzip,
    Showlist
}

#[derive(Debug)]
struct BaseZip
{
    whattodo: Todo,
    target_zip: std::path::PathBuf,
    target_ex: Vec<std::path::PathBuf>,
    comp_level: u8
}

impl BaseZip
{
    fn new() -> BaseZip
    {
        let newst = BaseZip
        {
            whattodo: Todo::None,
            target_zip: std::path::PathBuf::new(),
            target_ex: Vec::new(),
            comp_level: 9
        };
        newst
    }
}