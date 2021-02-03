use lazy_static::lazy_static;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;

use async_once::AsyncOnce;

lazy_static! {
    static ref FOO : AsyncOnce<u32> = AsyncOnce::new(async{
        1
    });
}
fn main() {

    let mut rt =  Builder::new().thread_name("111").build().unwrap();   //Builder::new_current_thread().build().unwrap();
    rt.block_on(async {
        assert_eq!(FOO.get().await , &1)
    });

    #[derive(Debug)]
    struct  abc{
        a:u32,
        b:u32,
    }

    let mut x = abc { a: 12, b: 13 };
    //let mut  y = &mut  x;
    x.a = 14;
   // y.a = 15;
    //println!("{:?}",y);
}