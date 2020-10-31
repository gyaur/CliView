#[cfg(target_arch = "arm")]
#[macro_export]
macro_rules! gen_proxy_function {
    ( $x:ident,$name:expr,$method:ident,$port:expr) => {
        #[rocket::$method($name)]
        fn $x() -> Redirect {
            Redirect::temporary(format!("http://raspberrypi.local:{}{}", $port, $name))
        }
    };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! gen_proxy_function {
    ( $x:ident,$name:expr,$method:ident,$port:expr) => {
        #[rocket::$method($name)]
        fn $x() -> Redirect {
            Redirect::temporary(format!("http://localhost:{}{}", $port, $name))
        }
    };
}
