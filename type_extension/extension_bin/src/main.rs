use extension_lib::password_manager::{Locked, PasswordManager, Unlocked};

// cannot define inherent `impl` for a type outside of the crate where the type is defined
// define and implement a trait or new type instead
// struct SuperLocked;
// impl PasswordManager<SuperLocked>
// {
// }

// only traits defined in the current crate can be implemented for types defined outside of the crate
// impl From<String> for PasswordManager
// {
//     fn from(value: String) -> Self {
//         todo!()
//     }
// }

// Using traits that we define
trait SuperLocked {
    fn super_unlock(self) -> PasswordManager<Unlocked> where Self:Sized;
}

impl SuperLocked for PasswordManager<Locked>
{
    fn super_unlock(self) -> PasswordManager<Unlocked> where Self:Sized
    {
        self.unlock()
    }
}

// newtype
struct NewPasswordManager(PasswordManager);

impl From<&str> for NewPasswordManager
{
    fn from(value: &str) -> Self {
        NewPasswordManager(PasswordManager::new(value.to_string()))
    }
}

fn main() {
    let master_pass = "password123";
    let outer_manager = NewPasswordManager::from(master_pass);
    let NewPasswordManager(manager) = outer_manager;
    println!("{}", manager.version());
}