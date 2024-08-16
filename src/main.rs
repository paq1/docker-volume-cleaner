use std::io::Read;
use std::net::TcpStream;
use ssh2::Session;

fn main() {

    dotenv::dotenv().ok();

    // get env vars
    let user = std::env::var("USER_SSH").unwrap();
    let pwd = std::env::var("PWD_SSH").unwrap();
    let host = std::env::var("HOST_SSH").unwrap();


    let tcp = TcpStream::connect(host).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&user, &pwd).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("docker volume ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{s}");
}
