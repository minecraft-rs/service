use mc_service::MinecraftAccount;

fn main() {
    let account = MinecraftAccount::new("{your access token}");
    let profile = account.get_profile().unwrap();

    println!("Username: {}", profile.name);
    println!("UUID: {}", profile.id);
    println!("Skin: {}", profile.skins[0].url);
    println!("Skin Variant: {}", profile.skins[0].variant);
    println!("Cape: {}", profile.capes[0].alias);
    println!("Cape URL: {}", profile.capes[0].url);
}
