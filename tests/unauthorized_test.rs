#[cfg(test)]
mod tests {
    use mc_service::account::MinecraftAccount;

    #[test]
    #[should_panic]
    fn unauthorized_test() {
        let account = MinecraftAccount::new("INVALID ACCESS TOKEN");
        let profile = account.get_profile().unwrap();
        assert_eq!(profile.name, "");
    }
}
