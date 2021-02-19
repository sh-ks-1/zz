// tar압축을 위한 함수

mod tar1;
mod tar2;
mod tar3;

pub fn roumain() -> ()
{
    let mut basest = BaseTargz::new();
    basest.whattodo = tar1::zip_or_unzip();
    basest.gz = tar2::tar_or_targz();
    basest.target_targz = tar3::targetfile(&basest.gz, &basest.whattodo);



    
    let mut commend = String::from("tar ");
    {
        let mut zxc = String::from("-z");
        match basest.whattodo
        {
            Todo::Zip => zxc.push_str("c"),
            Todo::Unzip => zxc.push_str("x"),
            Todo::Showlist => zxc.push_str("t"),
            _ => {}
        };
        zxc.push_str("vf");
        commend.push_str(&zxc);
    }
    // tar -zxvf
    // tar -zcvf
    dbg!(&commend);
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
        };
        newst
    }
}