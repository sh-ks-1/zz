// use std::io::Write;

pub fn target_ex(whattodo: &super::Todo) -> Vec<std::path::PathBuf>
{
    println!("");

    let return_vec = match whattodo
    {
        super::Todo::Zip =>
        {
            toziplist()
        },
        super::Todo::Showlist =>
        {
            // 반환이 필요없음, 압축파일 안의 내용을 보기위함
            let tv: Vec<std::path::PathBuf> = Vec::new();
            tv
        },
        super::Todo::Unzip =>
        {
            vec!(extract_where())
        }
        _ =>
        {
            let tv: Vec<std::path::PathBuf> = Vec::new();
            tv
        }
    };
    return_vec
}

// 압축할 목록
fn toziplist() -> Vec<std::path::PathBuf>
{
    let current_exe_exuct_dir = std::env::current_dir().unwrap();
    println!("압축할 파일/폴더를 입력하세요");
    println!("입력을 끝내고자 한다면 빈칸을 그대로 엔터");
    println!("직전 입력 대상 취소는 -1");

    let mut listv: Vec<std::path::PathBuf> = Vec::new();

    loop
    {
        let mut userinput = String::new();
        std::io::stdin().read_line(&mut userinput).unwrap();
        userinput = userinput.trim().to_string();

        // 중지
        if userinput == ""
        {
            break
        }
        // 하니씩제외
        match userinput.parse::<i32>()
        {
            Ok(a) =>
            {
                if a == -1
                {
                    let popted = listv.pop();
                    match popted
                    {
                        Some(popitem) =>
                        {
                            println!("{:?} 제외됨",popitem);
                        },
                        None =>
                        {
                            println!("제외될 성분 없음");
                        }
                    };
                    continue
                }
            },
            Err(_) => {}
        }

        // 입력
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
                continue
            }
        };

        inputpath_abs = inputpath_abs.join(input_filetarget.file_name().unwrap());

        if !(inputpath_abs.exists())
        {
            println!("(입력한 대상이 존재하지 않음)");
            continue
        };

        listv.push(inputpath_abs);
        // dbg!(&listv);
        println!("목록");
        for (iidx, iitem) in listv.iter().enumerate()
        {
            println!("{} - {:?}",iidx, iitem);
        }
    }

    listv.sort();
    listv.dedup();
    println!("압축을 하려는 대상은 다음과 같습니다.\n{:?}",listv);
    listv
}

// 압축 풀 위치
fn extract_where() -> std::path::PathBuf
{
    let current_exe_exuct_dir = std::env::current_dir().unwrap();
    println!("압축을 풀어낼 폴더를 입력");

    let mut userinput = String::new();
    
    loop
    {
        std::io::stdin().read_line(&mut userinput).unwrap();
        userinput = userinput.trim().to_string();
        if (userinput == "") | (userinput == ".") | (userinput == "./") 
        {
            println!("현제폴더에 풀기");
            userinput = current_exe_exuct_dir.to_str()
                .unwrap()
                .to_string();
            break
        }
        else
        {
            break
        }
    } 

    let input_filetarget: std::path::PathBuf = std::path::PathBuf::from(userinput);
    let inputparent = match input_filetarget.parent()
    {
        Some(a) =>
        {
            // dbg!(&a);
            if a.to_str() == Some("")
            {
                &current_exe_exuct_dir
            }
            else
            {
                a
            }
        },
        None => &current_exe_exuct_dir
    };
    let mut inputpath_abs = inputparent.canonicalize().unwrap();

    // 확인과정
    // let mut inputpath_abs = match inputparent.canonicalize()
    // {
    //     Ok(path) => 
    //     {
    //         path
    //     },
    //     Err(_) => 
    //     {
    //         println!("(재입력)");
    //         extract_where()
    //     }
    // };

    inputpath_abs = inputpath_abs.join(input_filetarget.file_name().unwrap());
    dbg!(&inputpath_abs);

    // if !(inputpath_abs.exists())
    // {
    //     println!("(입력한 대상이 존재하지 않음)");
    //     inputpath_abs = extract_where();
    // };

    inputpath_abs
}