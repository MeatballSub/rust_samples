use std::collections::HashMap;

pub struct Locked;
pub struct Unlocked;

pub struct PasswordManager<State = Locked>
{
    master_pass: String,
    passwords: HashMap<String, String>,
    state: std::marker::PhantomData<State>
}

impl PasswordManager<Locked>
{
    pub fn unlock(self) -> PasswordManager<Unlocked>
    {
        PasswordManager { master_pass: self.master_pass, passwords: self.passwords, state: std::marker::PhantomData::<Unlocked> }
    }
}

impl PasswordManager<Unlocked>
{
    pub fn lock(self) -> PasswordManager<Locked>
    {
        PasswordManager { master_pass: self.master_pass, passwords: self.passwords, state: std::marker::PhantomData::<Locked> }
    }

    pub fn list_passwords(&self) -> &HashMap<String, String>
    {
        &self.passwords
    }

    pub fn add_password(&mut self, username:String, password:String)
    {
        self.passwords.insert(username, password);
    }
}

impl<State> PasswordManager<State>
{
    pub fn encryption(&self) -> String
    {
        todo!()
    }

    pub fn version(&self) -> String
    {
        todo!()
    }
}

impl PasswordManager
{
    pub fn new(master_pass: String) -> Self
    {
        PasswordManager { master_pass, passwords: Default::default(), state: Default::default() }
    }
}