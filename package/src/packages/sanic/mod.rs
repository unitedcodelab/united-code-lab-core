pub fn run() {
    println!("RUNNING SANIC PROJECT");
    handle::run_shell("src/bin/run.sh")
}

pub fn build(){
    println!("BUILDING SANIC PROJECT");
    handle::run_shell("src/bin/build.sh")
}

mod handle {
    use std::fs;
    use std::process::Command;

    pub fn run_shell(path: &str){
        if fs::metadata(path).is_ok() {
            Command::new("sh")
                .arg(path)
                .output()
                .unwrap();
            return println!("File was found at {}", path)
        }
        println!("ERROR: You are not at the root of the project.")
    }
}
