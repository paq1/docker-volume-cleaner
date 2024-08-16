use std::ops::Add;
use ssh;

fn main() {

    dotenv::dotenv().ok();

    let user = std::env::var("USER_SSH").unwrap();
    let pwd = std::env::var("PWD_SSH").unwrap();
    let host = std::env::var("HOST_SSH").unwrap();
    let special_chars = vec!['&', '$', '|'];
    let escape_pwd = pwd.escape(special_chars.clone());
    let folder_docker = "/usr/local/bin/";

    let mut session = ssh::create_session()
        .username(&user)
        .password(&pwd)
        .private_key_path("./id_rsa")
        .connect(host.clone())
        .unwrap();


    let utf8_docker_volume = session.run_local()
        .open_channel()
        .unwrap()
        .exec()
        .unwrap()
        .send_command(&format!("cd {folder_docker} && echo {escape_pwd} | sudo -S ./docker volume ls"))
        // .send_command(&format!("cd ../../docker && ls && echo {} | sudo -S docker volume ls", escape_pwd))
        .unwrap();

    let docker_volume_str = std::str::from_utf8(&utf8_docker_volume)
        .unwrap()
        .to_string();


    let volumes = docker_volume_str
        .split("\n")
        .collect::<Vec<_>>()
        [1..].to_vec()
        .iter().map(|x| x.replace("local ", "").clone().trim().to_string())
        .collect::<Vec<_>>();

    let keep_volumes: Vec<&str> = vec![
        "elastic_elasticsearch-data",
        "mongo-dev",
        "mongo-prd",
        "mongo-stg",
        "mongo-local",
        "phasmonkey-backend_phasmonkey_backend_db_data",
        "authn-monkey_authn_monkey_db_data",
        "authz-api_authz_db_data"
    ];

    let must_be_delete_volumes = volumes.into_iter().filter(|id_volume| {
        !keep_volumes.contains(&id_volume.as_str()) && *id_volume != "".to_string()
    })
        .collect::<Vec<_>>();


    let mut session_delete = ssh::create_session()
        .username(&user)
        .password(&pwd)
        .private_key_path("./id_rsa")
        .connect(host.clone())
        .unwrap();


    let utf8_response_delete = session_delete.run_local()
        .open_channel()
        .unwrap()
        .exec()
        .unwrap()
        .send_command(&create_delete_command(&must_be_delete_volumes, folder_docker, &escape_pwd))
        // .send_command(&format!("cd ../../docker && ls && echo {} | sudo -S docker volume ls", escape_pwd))
        .unwrap();

    let response_delete = std::str::from_utf8(&utf8_response_delete).unwrap();

    println!("delete response : {response_delete}");
}

fn create_delete_command(ids: &Vec<String>, folder_docker: &str, pwd: &str) -> String {
    let command_without_auth = ids.iter()
        .map(|current_id| create_one_delete_command(current_id.as_str()))
        .collect::<Vec<_>>()
        .join(" && ");



    let cmd = format!("cd {folder_docker} && echo {pwd} | sudo -S {command_without_auth}");

    println!("{cmd}");

    cmd
}

fn create_one_delete_command(id_volume: &str) -> String {
    format!("./docker volume rm {id_volume}")
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