macro_rules! unsupported {
    ($name:ident) => {
        fn $name(self) -> Result<Self::Ok, Self::Error> {
            Err(crate::error::Error::Unsupported(stringify!($name).to_string()))
        }
    };

    ($name:ident, $($types:ty),*) => {
        fn $name(self, $(_: $types),*) -> Result<Self::Ok, Self::Error> {
            Err(crate::error::Error::Unsupported(stringify!($name).to_string()))
        }
    };

    ($name:ident -> $out:ident, $($types:ty),*) => {
        fn $name(self, $(_: $types),*) -> Result<Self::$out, Self::Error> {
            Err(crate::error::Error::Unsupported(stringify!($name).to_string()))
        }
    };
}

pub(crate) use unsupported;
