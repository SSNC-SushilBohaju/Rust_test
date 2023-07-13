
use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:root@127.0.0.1:3306/testdb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )",
    )?;
    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 200,
            account_name: Some("xxx".into()),
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("yyy".into()),
        },
        Payment {
            customer_id: 5,
            amount: 600,
            account_name: Some("zzz".into()),
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];

    // Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
          VALUES (:customer_id, :amount, :account_name)",
        payments.iter().map(|p| {
            params! {
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }
        }),
    )?;

    // Let's select payments from database. Type inference should do the trick here.
    let selected_payments = conn.query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| Payment {
            customer_id,
            amount,
            account_name,
        },
    )?;

    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows
    // without `ORDER BY`, so assume we are lucky.
    assert_eq!(payments, selected_payments);
    println!("Yay!");

    Ok(())
}
