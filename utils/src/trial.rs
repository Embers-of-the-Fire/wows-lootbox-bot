#[macro_export]
macro_rules! tri {
    ($keyw:stmt; $ident:ident $expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(e) => {
                ::log::$ident!("{}", &e);
                $keyw;
            }
        }
    };
}

#[macro_export]
macro_rules! tridbg {
    ($keyw:stmt; $ident:ident $expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(e) => {
                ::log::$ident!("{:?}", &e);
                $keyw;
            }
        }
    };
}

#[macro_export]
macro_rules! tristr {
    ($ident:ident $expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(e) => {
                ::log::$ident!("{:?}", &e);
                return Err(e.to_string());
            }
        }
    };
}

#[macro_export]
macro_rules! triany {
    ($ident:ident $expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(e) => {
                ::log::$ident!("{:?}", &e);
                return Err(e.into());
            }
        }
    };
}
