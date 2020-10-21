#[macro_export]
macro_rules! gen_proxy_function {
    ( $x:ident,$name:expr,$method:ident,$port:expr) => {
        #[rocket::$method($name)]
        fn $x() -> Redirect {
            Redirect::permanent(format!("http://localhost:{}{}", $port, $name))
        }
    };
}
