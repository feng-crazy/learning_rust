// use std::fs::File;
// use json::{array, object};
// // use serde::ser::{Serialize, SerializeStruct, Serializer};
//
// extern crate serde;
// extern crate serde_json;
// extern crate serde_derive;
//
//
// #[cfg(test)]
// mod tests {
//     use crate::json_test::json_test::{sample_deserialization, sample_serialization, serde_json_test};
//     #[test]
//     fn it_works() {
//         sample_deserialization();
//         sample_serialization();
//         serde_json_test();
//     }
// }
//
// fn sample_deserialization(){
//     use json;
//     let parsed = json::parse(r#"
//         {
//             "code": 200,
//             "success": true,
//             "payload": {
//                 "features": [
//                     "awesome",
//                     "easyAPI",
//                     "lowLearningCurve"
//                 ]
//             }
//         }
//         "#).unwrap();
//
//     let instantiated = json::object! {
//             // quotes on keys are optional
//             "code": 200,
//             success: true,
//             payload: {
//                 features: [
//                     "awesome",
//                     "easyAPI",
//                     "lowLearningCurve"
//                 ]
//             }
//         };
//
//     assert_eq!(parsed, instantiated);
//     println!("解析输出字段：code={}",parsed["code"]);
//     println!("解析输出字段：success={}",instantiated["success"]);
//     println!("解析输出对象：payload={:?}",instantiated["payload"]);
//     println!("解析输出数组：features={:?}",instantiated["payload"]["features"]);
//     println!("解析输出数组元素：0={}",instantiated["payload"]["features"][0]);
//     println!("解析输出数组元素：1={}",instantiated["payload"]["features"][1]);
//     println!("解析输出数组元素：2={}",instantiated["payload"]["features"][2]);
// }
//
// fn sample_serialization(){
//     let features_0 : Option<String> = Some("美貌".to_string());
//     let features_1 : Option<String> = None;
//     let score = vec![67,78,87];
//     let score_other = vec![Some(87),Some(89),None];
//     let hobby = json::Null;
//
//     let mut sub_items = json::JsonValue::new_object();
//     sub_items["跳高"] = 2.into();
//     sub_items["跳远"] = 3.into();
//
//     let mut subject = json::JsonValue::new_array();
//     subject.push(100);
//     subject.push(99);
//     subject.push(sub_items);
//
//
//
//     let data = array!["123",true,json::Null,300];
//     let students = object!{
//         "name" => "zhangsan",
//         "sex" => 15,
//         "height" => 156,
//         "weight" => 45,
//         "hobby1" => "吹牛逼".to_string(),
//         "hobby2" => hobby,
//         "ke_mu" => subject,
//         "features" => array![features_0,features_1],
//         "score_main" => score,
//         "score_branch" => score_other,
//         "others"=> data
//     };
//     let response = students.dump();
//     println!("[返回数据]：{}",response)
// }
//
//
//
// fn serde_json_test(){
//     let f = File::open("./sample.json").unwrap();
//     let values:serde_json::Value = serde_json::from_reader(f).unwrap();
//     println!("整个字符串：{:?}",values);
//
//     println!("name:{}",values["name"]);
//     println!("age:{}",values["age"]);
//     println!("address-city:{}",values["address"]["city"]);
//     println!("address-street:{}",values["address"]["street"]);
//     println!("phones-0:{}",values["phones"][0]);
//     println!("phones-1:{}",values["phones"][1]);
//
//     // 解析为脏类型
//     println!("{:?}", values["name"].as_str().unwrap());
//     println!("{:?}", values["age"].as_i64().unwrap());
// }
//
// fn serde_json_struct_test(){
//     // #[derive(Debug, Serialize, Deserialize)]
//     struct Address{
//         street: String,
//         city: String,
//     }
//
//     // #[derive(Debug, Serialize, Deserialize)]
//     struct Person{
//         name: String,
//         age:u8,
//         address:Address,
//         phones:Vec<String>,
//     }
//
//
//     impl Person {
//         fn default() -> Self {
//             Person{
//                 name: "zhangsan".to_string(),
//                 age: 18u8,
//                 address:Address{
//                     street: "East nanjing road".to_string(),
//                     city: "shanghai".to_string(),
//                 },
//                 phones:vec!["13562958755".to_string(),"15963695569".to_string()],
//             }
//         }
//     }
//
//     let f = File::open("./sample.json").unwrap();
//     let values:Person = serde_json::from_reader(f).unwrap();
//     println!("强类型解析输出：{:?}",values);
//
//     let name = &values.name;
//     let age = &values.age;
//     let city = &values.address.city;
//     let phones = &values.phones;
//     println!("== name =={}",name);
//     println!("== age =={}",age);
//     println!("== city =={}",city);
//     println!("== phones =={:?}",phones);
//     println!("==address-street=={}",&values.address.street);
//
//
//     let person = Person::default();
//     let person_json = serde_json::to_string(&person).expect("Couldn't serialize config");
//     let person_json_pretty = serde_json::to_string_pretty(&person).expect("Couldn't serialize config");
//     println!("person_json 直接转成json:\n {}", person_json);
//     println!("person_json_pretty 转成格式化json:\n {}", person_json_pretty);
// }