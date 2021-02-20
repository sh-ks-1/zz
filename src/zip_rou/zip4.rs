pub fn complev() -> u8
{
    println!("");
    println!("1 ~ 9 범위 숫자를 입력하세요. (9, 가장 압축률 높음)");

    loop
    {
        let mut userinput = String::new();
        std::io::stdin().read_line(&mut userinput).unwrap();
        userinput = userinput.trim().to_string();

        let isu8 = userinput.parse::<u8>();
        let levelint = match isu8
        {
            Ok(i) => i,
            Err(_) =>
            {
                println!("숫자를 입력사세요");
                continue
            }
        };
        if (levelint >= 1) & (levelint <= 9)
        {
            return levelint
        }
        else
        {
            println!("1 ~ 9 범위 숫자를 입력");
            continue
        }

    }
}