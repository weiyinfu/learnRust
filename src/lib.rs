#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables, unused_assignments, unused_mut)]
#![allow(non_camel_case_types)]
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
mod baArrayTransfer;
mod bcOwnership;
mod beMove;
mod biSlice;
mod caModule;
mod ccInterface;
mod cdGeneric;
mod ceLifetime;
mod cfSmartPointer;
mod cgOperator;
mod chConcurrent;
mod ciAnnotations;
mod cjFunction;
mod ckClosure;
mod clMacro;
mod cmErrorHandling;
mod cnTest;
mod daSingleton;
mod maUseString;
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
mod mlHttpClient;
mod mmChrono;
