#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;
    use mc_service::MinecraftAccount;

    #[test]
    fn get_profile() {
        dotenv().ok();

        let access_token = env::var("ACCESS_TOKEN").unwrap();
        let username = env::var("MC_USERNAME").unwrap();

        let account = MinecraftAccount::new(&access_token);
        let profile = account.get_profile().unwrap();
        assert_eq!(profile.name, username);
    }

    #[test]
    #[should_panic]
    fn get_profile_invalid() {
        dotenv().ok();

        let account = MinecraftAccount::new("NVALID ACCESS TOKEN");
        let profile = account.get_profile().unwrap();
        assert_eq!(profile.name, "Sammwy_");
    }
}
