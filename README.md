# MC Service

Minecraft HTTP services wrapper for Rust.

## Why?

To obtain or update data from a minecraft account, it is necessary to interact with its services through its rest api. With this library you can interact with the API in a simple way.

## Prepare

You need to authenticate the user before using this library and get an access_token. You can use our [authentication library](https://github.com/minecraft-rs/auth) for that.

## Usage

```rust
use mc_service::account::MinecraftAccount;

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
```

## Contribution

Feel free to contribute to the development of the library.
