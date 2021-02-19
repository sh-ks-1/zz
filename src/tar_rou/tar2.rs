use std::io::Write;

use super::Gzor;

pub fn tar_or_targz() -> Gzor
{
    println!("");
    println!("1) tar로 묶기/풀기");
    println!("2) tar.gz로 압축/풀기");
    print!("= ");
    let _ = std::io::stdout().flush();

    let mut userinput = String::new();
    std::io::stdin().read_line(&mut userinput).unwrap();
    userinput = userinput.trim().to_string();

    if (userinput == "1") | (userinput == "2")
    {
        let reenum = if userinput == "1"
        {
            Gzor::Tar
        }
        else
        {
            Gzor::Targz
        };
        dbg!(&reenum);
        return reenum;
    }
    else
    {
        return tar_or_targz();
    }
}
