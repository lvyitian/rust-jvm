#[derive(Debug)]
pub enum Type {
    Char,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    Reference(Class)
}

impl Type {
    pub fn new(type_str: &str) -> Option<Self> {
        Self::new_partial(type_str).0
    }

    fn new_partial(type_str: &str) -> (Option<Self>, usize) {
        let (specifier, rem) = type_str.split_at(1);
        match specifier {
            "C" => (Some(Type::Char), 1),
            "B" => (Some(Type::Byte), 1),
            "S" => (Some(Type::Short), 1),
            "I" => (Some(Type::Int), 1),
            "J" => (Some(Type::Long), 1),
            "F" => (Some(Type::Float), 1),
            "D" => (Some(Type::Double), 1),
            "Z" => (Some(Type::Boolean), 1),
            "L" => {
                let end = rem.find(';').unwrap();
                let (name, rem) = rem.split_at(end);
                (Some(Type::Reference(Class::new(name))), end + 2)
            },
            "V" => {
                (None, 1)
            },
            _ => {
                panic!("Unknown type: {}", type_str);
            }
        }
    }
}
#[derive(Debug)]
pub struct Class {
    name: String
}

impl Class {
    pub fn new(name: &str) -> Self {
        Class {
            name: String::from(name)
        }
    }
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub params: Vec<Type>,
    pub return_type: Option<Type>
}

impl Method {
    pub fn new(name: String, descriptor: String) -> Self {
        if !descriptor.starts_with('(') {
            panic!("Invalid method descriptor");
        }
        let end_param = match descriptor.find(')') {
            Some(res) => {
                res
            },
            None => {
                panic!("Invalid method descriptor");
            }
        };
        let mut params = &descriptor[1..end_param];
        let len = params.len();
        let mut types = vec![];
        let mut i = 0;

        while i < len {
            let (ty, used) = Type::new_partial(params);
            i += used;
            params = params.split_at(used).1;
            types.push(ty.unwrap());
        }
        
        let return_type = Type::new_partial(&descriptor[(end_param + 1)..]).0;
        
        Method {
            name: name,
            params: types,
            return_type: return_type
        }
    }
}

#[derive(Debug)]
pub struct Field {
    name: String,
    ty: Type
}

impl Field {
    pub fn new(name: String, ty: Type) -> Self {
        Field {
            name: name,
            ty: ty
        }
    }
}

