extern crate secret_service;

#[derive(Debug)]
pub enum Error {
    SecretServiceError,
}

derror!(secret_service::SsError, Error::SecretServiceError);

pub trait PasswordStorage {
    fn delete_pass(&self, key: &str) -> Result<(), Error> {
        ss_storage::delete_pass(key)
    }

    fn store_pass(&self, username: String, password: String, server: String) -> Result<(), Error> {
        ss_storage::store_pass(username, password, server)
    }

    fn get_pass(&self) -> Result<(String, String, String), Error> {
        ss_storage::get_pass()
    }

    fn store_token(&self, uid: String, token: String) -> Result<(), Error> {
        ss_storage::store_token(uid, token)
    }

    fn get_token(&self) -> Result<(String, String), Error> {
        ss_storage::get_token()
    }
}


mod ss_storage {
    use super::Error;

    use super::secret_service::SecretService;
    use super::secret_service::EncryptionType;

    pub fn delete_pass(key: &str) -> Result<(), Error> {
        let ss = SecretService::new(EncryptionType::Dh)?;
        let collection = ss.get_default_collection()?;

        // deleting previous items
        let allpass = collection.get_all_items()?;
        let passwds = allpass
            .iter()
            .filter(|x| x.get_label().unwrap_or_default() == key);
        for p in passwds {
            p.delete()?;
        }

        Ok(())
    }

    pub fn store_token(uid: String, token: String) -> Result<(), Error> {
        let ss = SecretService::new(EncryptionType::Dh)?;
        let collection = ss.get_default_collection()?;
        let key = "fractal-token";

        // deleting previous items
        delete_pass(key)?;

        // create new item
        collection.create_item(
            key,                 // label
            vec![("uid", &uid)], // properties
            token.as_bytes(),    //secret
            true,                // replace item with same attributes
            "text/plain",        // secret content type
        )?;

        Ok(())
    }

    pub fn get_token() -> Result<(String, String), Error> {
        let ss = SecretService::new(EncryptionType::Dh)?;
        let collection = ss.get_default_collection()?;
        let allpass = collection.get_all_items()?;
        let key = "fractal-token";

        let passwd = allpass
            .iter()
            .find(|x| x.get_label().unwrap_or_default() == key);

        if passwd.is_none() {
            return Err(Error::SecretServiceError);
        }

        let p = passwd.unwrap();
        let attrs = p.get_attributes()?;
        let secret = p.get_secret()?;
        let token = String::from_utf8(secret).unwrap();

        let attr = attrs
            .iter()
            .find(|&ref x| x.0 == "uid")
            .ok_or(Error::SecretServiceError)?;
        let uid = attr.1.clone();

        Ok((token, uid))
    }

    pub fn store_pass(username: String, password: String, server: String) -> Result<(), Error> {
        let ss = SecretService::new(EncryptionType::Dh)?;
        let collection = ss.get_default_collection()?;
        let key = "fractal";

        // deleting previous items
        delete_pass(key)?;

        // create new item
        collection.create_item(
            key,                                                // label
            vec![("username", &username), ("server", &server)], // properties
            password.as_bytes(),                                //secret
            true,                                               // replace item with same attributes
            "text/plain",                                       // secret content type
        )?;

        Ok(())
    }

    pub fn migrate_old_passwd() -> Result<(), Error> {
        let ss = SecretService::new(EncryptionType::Dh)?;
        let collection = ss.get_default_collection()?;
        let allpass = collection.get_all_items()?;

        // old name password
        let passwd = allpass
            .iter()
            .find(|x| x.get_label().unwrap_or(strn!("")) == "guillotine");

        if passwd.is_none() {
            return Ok(());
        }

        let p = passwd.unwrap();
        let attrs = p.get_attributes()?;
        let secret = p.get_secret()?;

        let mut attr = attrs
            .iter()
            .find(|&ref x| x.0 == "username")
            .ok_or(Error::SecretServiceError)?;
        let username = attr.1.clone();
        attr = attrs
            .iter()
            .find(|&ref x| x.0 == "server")
            .ok_or(Error::SecretServiceError)?;
        let server = attr.1.clone();
        let pwd = String::from_utf8(secret).unwrap();

        // removing old
        for p in passwd {
            p.delete()?;
        }

        store_pass(username, pwd, server)?;

        Ok(())
    }

    pub fn get_pass() -> Result<(String, String, String), Error> {
        migrate_old_passwd()?;

        let ss = SecretService::new(EncryptionType::Dh)?;
        let collection = ss.get_default_collection()?;
        let allpass = collection.get_all_items()?;
        let key = "fractal";

        let passwd = allpass
            .iter()
            .find(|x| x.get_label().unwrap_or_default() == key);

        if passwd.is_none() {
            return Err(Error::SecretServiceError);
        }

        let p = passwd.unwrap();
        let attrs = p.get_attributes()?;
        let secret = p.get_secret()?;

        let mut attr = attrs
            .iter()
            .find(|&ref x| x.0 == "username")
            .ok_or(Error::SecretServiceError)?;
        let username = attr.1.clone();
        attr = attrs
            .iter()
            .find(|&ref x| x.0 == "server")
            .ok_or(Error::SecretServiceError)?;
        let server = attr.1.clone();

        let tup = (username, String::from_utf8(secret).unwrap(), server);

        Ok(tup)
    }
}
