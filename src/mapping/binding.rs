#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Binding {
    pub name: &'static str,
    pub doc: &'static str,
    pub fields: &'static [Field],
    pub args: Option<&'static str>,
}

impl Binding {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            doc: "",
            fields: &[],
            args: Some(name),
        }
    }

    pub const fn doc(mut self, doc: &'static str) -> Self {
        self.doc = doc;
        self
    }

    pub const fn fields(mut self, args: &'static [Field]) -> Self {
        self.fields = args;
        self
    }

    pub const fn args(mut self, args: &'static str) -> Self {
        self.args = if args.is_empty() { None } else { Some(args) };
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub ty: &'static str,
    pub doc: &'static str,
}

impl Field {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            doc: "",
            ty: "",
        }
    }

    pub const fn ty(mut self, ty: &'static str) -> Self {
        self.ty = ty;
        self
    }

    pub const fn doc(mut self, doc: &'static str) -> Self {
        self.doc = doc;
        self
    }
}

#[macro_export]
macro_rules! binding {
    ($(
        #[doc = $doc:expr]
        $name:expr => $args:tt {
            $(
                #[doc = $field_doc:expr]
                $field:tt $ty:expr
            )*
        }
    )*) => {
        $(
            pub const fn binding() -> Binding {
                const FIELDS: &[Field] = &[$(
                    Field::new(stringify!($field))
                        .ty($ty)
                        .doc($field_doc)
                ),*];
                Binding::new($name).doc($doc).args($args).fields(FIELDS)
            }
        )*
    };

    ($(
        #[doc = $doc:expr]
        $name:expr => {
            $(
                #[doc = $field_doc:expr]
                $field:tt $ty:expr
            )*
        }
    )*) => {
        $(
            pub const fn binding() -> Binding {
                const FIELDS: &[Field] = &[$(
                    Field::new(stringify!($field))
                        .ty($ty)
                        .doc($field_doc)
                ),*];
                Binding::new($name).doc($doc).args("").fields(FIELDS)
            }
        )*
    };
}
