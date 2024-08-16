// use std::io::Read;
// use std::net::TcpStream;
// use ssh::TerminalSize;
// use ssh2::Session;

use ssh;

fn main() {

    dotenv::dotenv().ok();

    println!("{:?}", std::env::vars());


    let user = std::env::var("USER_SSH").unwrap();
    let pwd = std::env::var("PWD_SSH").unwrap();
    let host = std::env::var("HOST_SSH").unwrap();

    let mut session = ssh::create_session()
        .username(&user)
        .password(&pwd)
        .private_key_path("./id_rsa")
        .connect(host)
        .unwrap();

    // let utf8_ls = session.run_local()
    //     .open_exec()
    //     .unwrap()
    //     // .send_command("docker volume -ls")
    //     .send_command("ls")
    //     .unwrap();


    let utf8_ls = session.run_local()
        .open_channel()
        .unwrap()
        .exec()
        .unwrap()
        .send_command("ls")
        .unwrap()
        .to_vec();

    let chaine = std::str::from_utf8(&utf8_ls).unwrap();
    //
    println!("{}", chaine);
}
