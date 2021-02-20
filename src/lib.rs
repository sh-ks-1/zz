pub mod tar_rou;
pub mod zip_rou;



fn read_user_input_as_string() -> String
{
    let mut userinput = String::new();
    let input_valid = std::io::stdin().read_line(&mut userinput);

    match input_valid
    {
        Ok(_) => {},
        Err(_) =>
        {
            println!("(재입력필요)");
            userinput = read_user_input_as_string();
        }
    }
    userinput = userinput.trim().to_string();
    userinput
}
