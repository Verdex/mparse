
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

fn data_field(rule : &str, data : Data) -> Result<Data, ()> {
    let rule = rule.to_string();
    Ok(Data::Field( Box::new(Field { rule, data })))
}

// anything
// match string
//  invoke rule, zero or more, one or more, zero or one 
// and
// or
#[macro_export]
macro_rules! parse_rules {
    {$b:block} => {{

        fn i(s : &str) -> mparse::input::Input {
            mparse::input::Input::new(s)
        }

        #[allow(unused_macro)]
        macro_rules! define {
            ($name:ident) => {
                fn $name(input : &mut mparse::input::Input) -> Result<mparse::Data, ()> {
                    Err(())
                }
            };
        }

        $b
    }};
}

//#[allow(dead_code)]
//#[allow(unused_macro)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
