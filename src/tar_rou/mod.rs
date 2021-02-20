// tar압축을 위한 함수

use termion::{color,style};

mod tar1;
mod tar2;
mod tar3;
mod tar4;

pub fn roumain() -> ()
{
    let mut basest = BaseTargz::new();
    basest.whattodo = tar1::zip_or_unzip();
    basest.gz = tar2::tar_or_targz();
    basest.target_targz = tar3::targetfile(&basest.gz, &basest.whattodo);
    basest.target_ex = tar4::target_ex(&basest.whattodo);

    let mut commends: Vec<String> = Vec::new();
    {
        let mut zxc = match basest.gz
        {
            Gzor::Tar =>
            {
                String::from("-")
            },
            _ =>
            {
                String::from("-z")
            },
        };

        match basest.whattodo
        {
            Todo::Zip => zxc.push_str("c"),
            Todo::Unzip => zxc.push_str("x"),
            Todo::Showlist => zxc.push_str("t"),
            _ => {}
        };
        zxc.push_str("vf");
        commends.push(zxc);
    }
    commends.push(basest.target_targz.to_str().unwrap().to_string());

    match basest.whattodo
    {
        Todo::Unzip =>
        {
            commends.push("-C".to_string());
        },
        _ => {}
    };

    for iitem in basest.target_ex.iter()
    {
        commends.push(iitem.to_str().unwrap().to_string());
    }

    println!("");
    
    let allcomand = commends.join(" ");
    println!("{}{}실행할 커맨드: tar {}{}", color::Bg(color::White), color::Fg(color::Red), allcomand, style::Reset);

    println!("");

    let mut shcom = std::process::Command::new("tar");
    shcom.args(&commends);
    shcom.status().expect("command 실행 실패");
    // shcom.spawn().expect("command 실행 실패");

}

#[derive(Debug)]
pub enum Todo
{
    None,
    Zip,
    Unzip,
    Showlist,
}

#[derive(Debug)]
pub enum Gzor
{
    None,
    Tar,
    Targz
}

#[derive(Debug)]
struct BaseTargz
{
    whattodo: Todo,
    gz: Gzor,
    target_targz: std::path::PathBuf,
    target_ex: Vec<std::path::PathBuf>,
}

impl BaseTargz
{
    fn new() -> BaseTargz
    {
        let newst = BaseTargz
        {
            whattodo: Todo::None,
            gz: Gzor::None,
            target_targz: std::path::PathBuf::new(),
            target_ex: Vec::new(),
        };
        newst
    }
}