#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;
    use mc_service::account::MinecraftAccount;

    #[test]
    fn get_profile() {
        dotenv().ok();

        let access_token = env::var("ACCESS_TOKEN").unwrap();
        let account = MinecraftAccount::new(&access_token);

        let blocklist = account.get_blocklist().unwrap();
        assert_eq!(blocklist.blocked_profiles.len(), 0);
    }
}
