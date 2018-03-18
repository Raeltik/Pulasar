use std::net::TcpListener;
use std::net::TcpStream;
use std::process::Command;
use std::io::Write;

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
    stream.write_all(encoded.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client(stream: TcpStream){
//    let mut buffer = [0;512];
    println!("Handle clients");
    system_info(stream);
}

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("Listening");
    listener.set_nonblocking(true).expect("Can't set non-blocking");
    for stream in listener.incoming(){
    match stream {
        Ok(stream) => {
            println!("New Client!");
            let stream = stream;
            handle_client(stream)
            }
        Err(_e) => { /* Connection failed */ }
        }
    }
}
