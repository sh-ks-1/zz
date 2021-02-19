use std::io::Write;

pub fn targetfile(tar_or_targz: &super::Gzor, whattodo: &super::Todo) -> std::path::PathBuf
{
    println!("");
    match whattodo
    {
        super::Todo::Zip => {
            println!("압축파일로 만들어질 결과물의 경로(이름)");
        },
        _ =>
        {
            println!("대상으로 하는 압축파일 경로(이름)");
        }
    }
    let targetpath = readinput(tar_or_targz);
    targetpath
}

fn readinput(_tar_or_targz: &super::Gzor) -> std::path::PathBuf
{
    let current_exe_exuct_dir = std::env::current_dir().unwrap();

    print!("= ");
    let _ = std::io::stdout().flush();

    let mut userinput = String::new();
    std::io::stdin().read_line(&mut userinput).unwrap();
    userinput = userinput.trim().to_string();

    let input_filetarget: std::path::PathBuf = std::path::PathBuf::from(userinput);

    let _infname_str = input_filetarget.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut inputparent = input_filetarget.parent().unwrap();
    if inputparent == std::path::PathBuf::from("")
    {
        println!("(현재폴더를 대상으로 합니다.)");
        inputparent = &current_exe_exuct_dir;
    }

    let mut inputpath_abs = inputparent.canonicalize().unwrap();
    inputpath_abs = inputpath_abs.join(input_filetarget.file_name().unwrap());

    dbg!(&inputpath_abs);

    input_filetarget
}