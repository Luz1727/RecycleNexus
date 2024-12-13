#![no_std]
extern crate alloc; // Necesario para usar alloc::format!
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};
use alloc::format;  // Importa el macro format! desde alloc

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // Método para registrar un usuario
    pub fn registrar_usuario(env: Env, username: String, email: String) -> Vec<String> {
        vec![
            &env,
            String::from_str(&env, "Usuario registrado:"),
            username,
            email,
        ]
    }

       // Método para subir evidencia de reciclaje
       pub fn subir_evidencia(env: Env, username: String, material_type: String, amount: u32) -> Vec<String> {
        vec![
            &env,
            String::from_str(&env, "Evidencia cargada:"),
            username,
            material_type,
            String::from_str(&env, &format!("{}", amount)),  // Usamos format! desde alloc para convertir u32 a String
        ]
    }


    // Método para verificar evidencia de reciclaje
    pub fn verificar_evidencia(env: Env, username: String, image_url: String) -> Vec<String> {
        vec![
            &env,
            String::from_str(&env, "Evidencia verificada:"),
            username,
            image_url,
        ]
    }

    // Método para obtener la cantidad de tokens de un usuario
    pub fn obtener_tokens(env: Env, username: String) -> Vec<String> {
        vec![
            &env,
            String::from_str(&env, "Tokens para el usuario:"),
            username,
            String::from_str(&env, "10"),  // Supongamos que el usuario tiene 10 tokens
        ]
    }

}

mod test;