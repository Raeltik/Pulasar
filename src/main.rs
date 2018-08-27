use std::net::TcpListener;
use std::net::TcpStream;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

fn system_info(mut stream:  TcpStream) {

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "tasklist"])
                .output()
                .expect("Failed to execute process")
    } else {
        Command::new("ps")
                .arg("-ef")
                .arg("--sort")
                .arg("start_time")
                .output()
                .expect("Failed to execute process")    
    };
    let encoded = String::from_utf8_lossy(output.stdout.as_slice());
    let _ = match stream.write_all(encoded.as_bytes()){
        Result::Ok(val) => {val},
        Result::Err(_err) => {/* no connection to send back to  */}
    };
    stream.flush().unwrap();
    let mut file = File::create("/tmp/do_not_look").unwrap();
    let _ = match file.write_all(encoded.as_bytes()){
        Result::Ok(val) => {val},
        Result::Err(_err) => {/* idk, something happened  */}
    };

}

fn handle_client(mut stream: TcpStream){
//    let mut buffer = [0;512];
    stream.write_all("Password:".as_bytes()).unwrap();
    let mut buffer = String::new();
    let _ = match stream.read_to_string(&mut buffer){
        Result::Ok(val) => {val},
        Result::Err(_err) => { return }
    };

    if buffer.chars().count() == 0 { 
        let response = "Go away";
        let _ = stream.write_all(response.as_bytes());
    } 
    else {
        if buffer == "corsair\n"{
            system_info(stream)
        }
        else if buffer == "help\n"{
            test()
        } else{
            let response = "Go away";
            let _ = stream.write_all(response.as_bytes());        
        }


    }
}

fn test() {
    let pass = "The password is corsair";
    println!("{}",pass)
}

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    listener.set_nonblocking(true).expect("Can't set non-blocking");
    for stream in listener.incoming(){
    match stream {
        Result::Ok(stream) => {
            let stream = stream;
            handle_client(stream)
            }
        Result::Err(_e) => { /* Connection failed */ }
        }
    }
}
