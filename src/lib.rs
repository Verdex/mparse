
pub mod input;

pub struct Field {
    pub rule : String,
    pub data : Data,
}

pub enum Data {
    Nil,
    Char(char),
    Field(Box<Field>),
    Table { list : Vec<Data>, structure : Vec<Field> },
}

#[macro_export]
macro_rules! define {
    ($name:ident, $s:ident blah $e:expr) => { 
        fn $name($s : &mut mparse::input::Input) -> Result<mparse::Data, ()> {
            $e
        }
    };
}

#[macro_export]
macro_rules! and {
    { $( $es:expr ),* } => {
        {
            let mut list = vec![];
            let mut structure = vec![];

            $(
                match $es { 
                    Ok( mparse::Data::Field(f) ) => structure.push(*f),
                    Ok( other ) => list.push(other),
                    Err(e) => return Err(e),
                }
            )*

            Ok( mparse::Data::Table{ list, structure } )
        }
    };
}

#[macro_export]
macro_rules! or {
    { $( $es:expr ),* } => {
        {
            let mut ret = Err(());

            $(
                if matches!(ret, Err(_)) {
                    ret = $es;
                }
            )*

            ret
        }
    };
}

#[macro_export]
macro_rules! zero_or_one {
    ( $e:expr ) => {
        match $e {
            Ok(data) => Ok(mparse::Data::Table { list: vec![data], structure: vec![] }),
            Err(_) => Ok(mparse::Data::Table { list: vec![], structure: vec![] })
        }
    };
}


#[macro_export]
macro_rules! exact {
    ($s:ident, $value:literal) => {
        match $s.match_string($value) {
            Ok(_) => Ok(mparse::Data::Nil),
            Err(e) => Err(e),
        }
    };
}

#[macro_export]
macro_rules! any {
    ($s:ident) => {
        Ok(mparse::Data::Char($s.get_char()?))
    };
}

        /*enum ParseRule {
            Any,                                                            // Char(char) 
            MatchString(String),                                            // NIL 
            InvokeRule(String),                                             // Field
            ZeroOrMore(Box<ParseRule>),                                     // Table { list }
            OneOrMore(Box<ParseRule>),                                      // Table { list }
            ZeroOrOne(Box<ParseRule>),                                      // Table { list }
            Or(Vec<ParseRule>),                                             // Data
            And(Vec<ParseRule>),                                            // Table { list, structure }
        }*/
#[macro_export]
macro_rules! parse_rules {
    {$b:block} => {{


        fn i(s : &str) -> mparse::input::Input {
            mparse::input::Input::new(s)
        }

        $b
    }};
}

//#[allow(dead_code)]
//#[allow(unused_macro)]
