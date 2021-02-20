use std::io::Write;

pub fn targetfile(whattodo: &super::Todo) -> std::path::PathBuf
{
    println!("");
    let targetpath = match whattodo
    {
        super::Todo::Zip => {
            println!("압축파일로 만들어질 결과물의 경로(이름)");
            tarfile1()
        },
        _ =>
        {
            println!("대상으로 하는 압축파일");
            tarfile2()
        }
    }; 
    targetpath
}

fn tarfile1() -> std::path::PathBuf
{
    let current_exe_exuct_dir = std::env::current_dir().unwrap();

    print!("= ");
    let _ = std::io::stdout().flush();
    let userinput: String = crate::read_user_input_as_string();

    let input_filetarget: std::path::PathBuf = std::path::PathBuf::from(userinput);

    let _infname_str = input_filetarget.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut inputparent = input_filetarget.parent().unwrap();
    if inputparent == std::path::PathBuf::from("")
    {
        println!("(현재폴더에 압축파일을 생성합니다.)");
        inputparent = &current_exe_exuct_dir;
    }

    let mut inputpath_abs = inputparent.canonicalize().unwrap();
    inputpath_abs = inputpath_abs.join(input_filetarget.file_name().unwrap());

    println!("만들어질 압축파일: {:#?}",&inputpath_abs);
    // dbg!(&inputpath_abs);

    input_filetarget
}

fn tarfile2() -> std::path::PathBuf
{
    let current_exe_exuct_dir = std::env::current_dir().unwrap();

    print!("= ");
    let _ = std::io::stdout().flush();

    let userinput: String = crate::read_user_input_as_string();

    let input_filetarget: std::path::PathBuf = std::path::PathBuf::from(userinput);

    let mut inputparent = input_filetarget.parent().unwrap();
    if inputparent == std::path::PathBuf::from("")
    {
        inputparent = &current_exe_exuct_dir;
    }

    // 확인과정
    let mut inputpath_abs = match inputparent.canonicalize()
    {
        Ok(path) => 
        {
            path
        },
        Err(_) => 
        {
            println!("(재입력)");
            tarfile2()
        }
    };
    
    inputpath_abs = inputpath_abs.join(input_filetarget.file_name().unwrap());

    // 파일이 있는지 확인
    if !(inputpath_abs.exists())
    {
        println!("파일이 존재하지 않습니다. 재입력 필요");
        inputpath_abs = tarfile2();
    }

    inputpath_abs
}