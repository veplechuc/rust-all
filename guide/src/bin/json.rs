// option for creating Json
use serde::{Deserialize, Serialize};

use serde_json::{from_str, json, to_string_pretty};
// use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct Account {
    id: uuid::Uuid,
    user_name: String,
    login_date: Option<String>,
}

impl Account {
    fn new() -> Self {
        Account {
            id: uuid::Uuid::new_v4(),
            user_name: "New".to_string(),
            login_date: Some("2023-08-15".to_string()),
        }
    }
}

impl Account {
    fn to_json(&self) -> String {
        //r# specifies the raw format no need to scape ""
        //manually
        // format!(
        //     r#"{{"id": "{:?}", "user_name":{:?} }}"#,
        //     self.id, self.user_name
        // )

        //json! serde
        // json!({
        //     "id": self.id.to_string(), // need to convert to string if not error
        //     "user_name": self.user_name
        // })
        // .to_string() //this returns a type Value

        // serde::Serialize (include derive feature on cargo for this)
        // for uuid include de "serde" on features
        serde_json::to_string_pretty(self).unwrap()
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct OtherAccount {
    user_id: uuid::Uuid,
    user_name: String,
    login_date: Option<String>,
    acc: Account,
}

fn main() {
    let acct = Account {
        id: uuid::Uuid::new_v4(),
        user_name: "Vale".to_string(),
        login_date: Some("2023-08-15".to_string()),
    };
    println!("An Account -> {}", acct.to_json());

    let acc_defa = Account::default();

    println!("Default impl -> {}", acc_defa.to_json());

    println!("----------- deser --------");

    let mut accu: Account;

    match deserialize() {
        Ok(ac) => {
            println!("Account user name ->{}", ac.user_name);
            accu = ac;
        }

        Err(_) => {
            println!("Error");
            // Rust will complain if I dont initialize th accu var because it is used before
            accu = Account::default();
        }
    };

    println!(
        " user deserialized  -> {:?}, ID -> {}",
        accu.user_name, accu.id
    );

    //////// UNSING ANOTHER ACCOUNT /////////
    println!("----------- Using Another Acc --------");
    let a_acc = OtherAccount {
        user_id: uuid::Uuid::new_v4(),
        user_name: "Some_Other".to_string(),
        login_date: Some("2023-08-30".to_string()),
        acc: Account {
            id: uuid::Uuid::new_v4(),
            user_name: "Inside_Acc".to_string(),
            login_date: Some("2023-08-15".to_string()),
        },
    };
    let a_ser = to_string_pretty(&a_acc);
    if a_ser.is_ok() {
        println!(
            "Another Json from to_string_pretty --> {}",
            a_ser.ok().unwrap()
        );
    }
    println!("----------- calling deser for Another Account --------");
    let another_acc = deser_another();
    let mut from_main: Account = Account::default();
    let from_main2: OtherAccount;
    if another_acc.is_ok() {
        from_main2 = another_acc.ok().unwrap();
        println!(" printing from another Acc -> {:#?}", from_main2);
        from_main = from_main2.acc
    } else {
        print!("{:#?}", another_acc.err());
    }

    println!("Info of instace account --> {:?}", from_main);
    //////// UNSING JSON /////////
    println!("----------- Using JSON --------");
    json_exampl();
}

fn deserialize() -> Result<Account, serde_json::Error> {
    let deser = r#"
    {
        "id": "1f600718-c85f-4e49-b1e0-37009ff9999f",
        "user_name": "Other_User_Name",
        "login_date": "2023-08-15"
      }"#;
    from_str::<Account>(deser)

    // if acc.is_ok() {
    //     println!("{:#?}", acc.ok().unwrap());
    // } else {
    //     print!("{:#?}", acc.err());
    // }
}

fn deser_another() -> Result<OtherAccount, serde_json::Error> {
    let deser = r#"
    {
        "UserId": "1f600718-d6b3-474a-8131-19aa32db5327",
        "UserName": "Some_Other",
        "LoginDate": "2023-08-30",
        "Acc": {
          "id": "0336015d-721a-4d51-ab0a-cf6665c1a71d",
          "user_name": "Inside_Acc",
          "login_date": "2023-08-15"
        }
      }"#;
    from_str::<OtherAccount>(deser)
}

fn json_exampl() {
    let v = json!(
    {
        "id": "9999bf88-c85f-4e49-b1e0-37009ff9999f",
        "user_name": "Other_User_Name",
        "login_date": "2023-08-15"
      });

    println!(
        "Result using Json ---> {:?}, {:?}, {:?}",
        v["id"], v["user_name"], v["login_date"]
    );
}
