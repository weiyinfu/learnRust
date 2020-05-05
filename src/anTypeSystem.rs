#[cfg(test)]
mod ANDataTypeConvert {
    use std::fmt::{Display, Formatter};
    use core::fmt;
    use std::str::FromStr;

    #[test]
    fn main() {
        //不能把i32转成usize，不能把有符号转成无符号，反之，则可
        let x = -1;
        println!("{}", x as usize);
        let y: usize = 1;
        let z = (y as i32 + x) as usize;
        println!("{}", z);
    }

    #[test]
    fn fromAndTo() {
        struct Node {
            name: String,
            age: i32,
        }
        impl Display for Node {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "name={},age={}", self.name, self.age)
            }
        }
        impl From<i32> for Node {
            fn from(x: i32) -> Self {
                Node { name: "无名氏".to_string(), age: x }
            }
        }
        impl From<String> for Node {
            fn from(x: String) -> Self {
                Node { name: x, age: 0 }
            }
        }
        println!("{}", Node::from("weiyinfu".to_string()));
        println!("{}", Node::from(18));
        let x: Node = 18.into();
        let y: Node = "weiyinfu".to_string().into();
        println!("from number:{}\n from string:{}\n", x, y);
    }


    #[test]
    fn useTryFrom() {
        use std::convert::TryFrom;
        use std::convert::TryInto;

        #[derive(Debug, PartialEq)]
        struct EvenNumber(i32);

        impl TryFrom<i32> for EvenNumber {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNumber(value))
                } else {
                    Err(())
                }
            }
        }
        // TryFrom

        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        // TryInto

        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    }


    #[test]
    fn useToString() {
        use std::string::ToString;

        struct Circle {
            radius: i32
        }

        impl ToString for Circle {
            fn to_string(&self) -> String {
                format!("Circle of radius {:?}", self.radius)
            }
        }
        impl FromStr for Circle {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Circle { radius: s.parse::<i32>().unwrap() })
            }
        }
        println!("{}", Circle { radius: 10 }.to_string());
        let x = "100".parse::<Circle>().unwrap();
        let y: Circle = "1000".parse().unwrap();
        println!("{},{}", x.to_string(), y.to_string());
    }
}

#[test]
fn str2number() {
    //不指定类型就会报错，指定类型有下面两种方式
    // let x = "123".parse().unwrap();
    // println!("{}", x);
    let x = "123".parse::<i32>().unwrap();
    println!("{}", x);
    let x: i32 = "123".parse().unwrap();
    println!("{}", x);
}