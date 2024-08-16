use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::ops::Add;

fn main() {

    dotenv::dotenv().ok();

    // get env vars
    let user = std::env::var("USER_SSH").unwrap();
    let pwd = std::env::var("PWD_SSH").unwrap();
    let host = std::env::var("HOST_SSH").unwrap();
    let special_chars = vec!['&', '$', '|'];
    let escape_pwd = pwd.escape(special_chars.clone());



    let tcp = TcpStream::connect(host).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&user, &pwd).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec(&format!("echo {} | sudo -S ls", escape_pwd)).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("message :");
    println!("{s}");
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
                    acc.add(format!("\\{current}").as_str())
                } else {
                    acc.add(&current.to_string())
                }
            })
    }
}