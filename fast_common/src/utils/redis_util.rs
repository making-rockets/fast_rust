
use redis::RedisResult;


///缓存服务
pub struct RedisUtil {}



lazy_static! {

    pub static ref CLIETN:redis::Client = redis::Client::open(String::from("redis://localhost:6379")).unwrap();
}


impl RedisUtil {


    pub async fn set(key: String, value: String) -> RedisResult<String> {
        let mut multiplexed_connection = CLIETN.get_multiplexed_async_std_connection().await.unwrap();
        let result = redis::cmd("SET").arg(key).arg(value).query_async::<_, String>(&mut multiplexed_connection).await;
        return result;
    }
}
