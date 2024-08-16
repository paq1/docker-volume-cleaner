use std::ops::Add;
use ssh;

fn main() {

    dotenv::dotenv().ok();

    let user = std::env::var("USER_SSH").unwrap();
    let pwd = std::env::var("PWD_SSH").unwrap();
    let host = std::env::var("HOST_SSH").unwrap();
    let special_chars = vec!['&', '$', '|'];
    let escape_pwd = pwd.escape(special_chars.clone());

    let mut session = ssh::create_session()
        .username(&user)
        .password(&pwd)
        .private_key_path("./id_rsa")
        .connect(host)
        .unwrap();


    let utf8_ls = session.run_local()
        .open_channel()
        .unwrap()
        .exec()
        .unwrap()
        .send_command(&format!("echo {} | sudo -S docker volume ls", escape_pwd))
        .unwrap()
        .to_vec();

    let chaine = std::str::from_utf8(&utf8_ls).unwrap();
    //
    println!("{}", chaine);
}

impl CanEscape for String {}

trait CanEscape {
    fn escape(&self, chars: Vec<char>) -> String
    where Self: ToString
    {
        self
            .to_string()
            .chars()
            .fold("".to_string(), |acc, current| {
                if chars.contains(&current) {
                    acc.add(&format!("\\{current}"))
                } else {
                    acc.add(&current.to_string())
                }
            })
    }
}