use std::io::Write;

use super::Todo;

pub fn zip_or_unzip() -> Todo
{
    println!("");
    println!("1) 압축하기");
    println!("2) 압축풀기");
    println!("3) 압축파일 안의 파일 확인하기");
    print!("= ");
    let _ = std::io::stdout().flush();

    let mut userinput = String::new();
    std::io::stdin().read_line(&mut userinput).unwrap();
    userinput = userinput.trim().to_string();

    if (userinput == "1") | (userinput == "2") | (userinput == "3")
    {
        let reenum = if userinput == "1"
        {
            Todo::Zip
        }
        else if userinput == "2"
        {
            Todo::Unzip
        }
        else
        {
            Todo::Showlist
        };
        return reenum;
    }
    else
    {
        return zip_or_unzip();
    }
}