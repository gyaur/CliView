#[cfg(target_arch = "arm")]
#[macro_export]
macro_rules! gen_proxy_function {
    ( $x:ident,$name:expr,$method:ident,$port:ident) => {
        #[rocket::$method($name)]
        fn $x(state: State<CliViewConfig>) -> Redirect {
            Redirect::temporary(format!("http://raspberrypi.local:{}{}", state.$port, $name))
        }
    };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! gen_proxy_function {
    ( $x:ident,$name:expr,$method:ident, $port:ident) => {
        #[rocket::$method($name)]
        fn $x(state: State<CliViewConfig>) -> Redirect {
            Redirect::temporary(format!("http://localhost:{}{}", state.$port, $name))
        }
    };
}
