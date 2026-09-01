use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UuidRequest { pub count: usize, pub hyphens: bool, pub uppercase: bool }
#[derive(Serialize)]
pub struct UuidResponse { pub uuids: Vec<String> }
pub struct UuidConfig { pub count: usize, pub hyphens: bool, pub uppercase: bool }

#[derive(Deserialize)]
pub struct PasswordRequest { pub length: usize, pub uppercase: bool, pub lowercase: bool, pub numbers: bool, pub symbols: bool }
#[derive(Serialize)]
pub struct PasswordResponse { pub password: String }

#[derive(Deserialize)]
pub struct TokenRequest { pub length: usize, pub uppercase: bool, pub lowercase: bool, pub numbers: bool, pub symbols: bool }
#[derive(Serialize)]
pub struct TokenResponse { pub token: String }

#[derive(Deserialize)]
pub struct Md5Request { pub text: String }
#[derive(Serialize)]
pub struct Md5Response { pub md5_32_lower: String, pub md5_32_upper: String, pub md5_16_lower: String, pub md5_16_upper: String }

#[derive(Deserialize)]
pub struct QrRequest { pub text: String }
#[derive(Serialize)]
pub struct QrResponse { pub svg: String, pub error: Option<String> }

#[derive(Deserialize)]
pub struct LoremRequest { pub count: usize, pub mode: String }

#[derive(Deserialize)]
pub struct FakeUserRequest { pub count: usize, pub locale: String }
#[derive(Serialize)]
pub struct FakeUser { pub name: String, pub email: String, pub address: String, pub phone: String }
#[derive(Serialize)]
pub struct FakeUserResponse { pub users: Vec<FakeUser> }

#[derive(Deserialize)]
pub struct CreditCardRequest { pub count: usize, pub issuer: String }
#[derive(Serialize)]
pub struct CreditCard { pub number: String, pub issuer: String, pub expiry: String, pub cvv: String }
#[derive(Serialize)]
pub struct CreditCardResponse { pub cards: Vec<CreditCard> }
