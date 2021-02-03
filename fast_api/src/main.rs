/*use mysql::prelude::*;
use mysql::*;*/

type t = Vec<i32>;

fn main() {
    let x: Vec<i32> = vec![1, 2, 3, 45, 6, 7, 8, 9];




    /* let dsn = "mysql://root:root@localhost:3306/go";
     let pool = Pool::new(dsn).unwrap();
     let mut conn = pool.get_conn().unwrap();

     let result:Option<(i32,String)> = conn.query_first("select  age,user_name from user order by age asc limit 1").unwrap();
     println!("{:?}", result.unwrap());

     let ret:QueryResult<Binary> =
         "insert into user(id,user_name,age) values(?,?,?)".with((22112145,"list",10)).run(conn).unwrap();
     println!("{:?}",ret.last_insert_id())*/
}