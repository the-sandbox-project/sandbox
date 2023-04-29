use crate::id_is_valid;
use crate::environment::open_environment;

pub async fn create_new_environment(environment: String) {
    if id_is_valid(environment.clone()).await {
        open_environment(environment).await
    } else {
        println!("The environment {} does not exist?", environment)
    }
}
