mod hoge;

mod fuga {
    pub fn hello_module() {
        println!("Hello, module fuga!");
    }
}

fn main() {
    hoge::hello_module();
    fuga::hello_module();
}
