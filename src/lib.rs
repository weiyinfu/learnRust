#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables, unused_assignments, unused_mut)]
#![allow(non_camel_case_types)]
// 这个 crate 是一个库文件，下面这两句话可以省略，因为cargo会自动判断
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "learnRust"]
extern crate chrono;
#[macro_use]
extern crate lazy_static;
//为了使用序列化，需要引入下列crate。
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod abPrint;
mod adDataTypes;
mod aeArray;
mod aeEnum;
mod afStruct;
mod amVariable;
mod anTypeSystem;
mod aoExpression;
mod apMatch;
mod aqUsefulStruct;
mod axIfLet;
mod ayIterate;
mod bcOwnership;
mod beMove;
mod bfLifetime;
mod bhSingleton;
mod biSlice;
mod caModule;
mod ccTrait;
mod cdOperator;
mod ceGeneric;
mod cfSmartPointer;
mod chConcurrent;
mod ciAnnotations;
mod cjFunction;
mod ckClosure;
mod clMacro;
mod cmErrorHandling;
mod cnTest;
mod maString;
mod mbPath;
mod mcFile;
mod mdHashMap;
mod meRandom;
mod mfTime;
mod mgConsole;
mod mhVec;
mod miLinkedList;
mod mjLog;
mod mkSerde;
mod mmChrono;
mod mnBinaryHeap;
mod useFolder;
