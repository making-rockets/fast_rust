mod sites;

#[async_std::main]
async fn main() ->(){
    let org = sites::this_week_in_rust_org().await;
    println!("{:?}", org);
}


