pub fn login(creds: models::Credentials) {
    crate::database::get_user();
}

pub fn logout() {
}

pub mod models; //  maps module to filename models