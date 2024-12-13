#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_registrar_usuario() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // Registrar un usuario
    let result = client.registrar_usuario(&String::from_str(&env, "Dev"), &String::from_str(&env, "dev@example.com"));

    // Verificar el resultado
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, "Usuario registrado:"),
            String::from_str(&env, "Dev"),
            String::from_str(&env, "dev@example.com"),
        ]
    );
}



#[test]
fn test_subir_evidencia() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // Subir evidencia
    let result = client.subir_evidencia(
        &String::from_str(&env, "Dev"),
        &String::from_str(&env, "Plástico"),
        &5,  // Pasamos una referencia a 5
    );

    // Verificar el resultado
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, "Evidencia cargada:"),
            String::from_str(&env, "Dev"),
            String::from_str(&env, "Plástico"),
            String::from_str(&env, "5"),
        ]
    );
}




#[test]
fn test_verificar_evidencia() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // Verificar evidencia
    let result = client.verificar_evidencia(
        &String::from_str(&env, "Dev"),
        &String::from_str(&env, "http://example.com/image.png"),
    );

    // Verificar el resultado
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, "Evidencia verificada:"),
            String::from_str(&env, "Dev"),
            String::from_str(&env, "http://example.com/image.png"),
        ]
    );
}

#[test]
fn test_obtener_tokens() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // Obtener tokens del usuario
    let result = client.obtener_tokens(&String::from_str(&env, "Dev"));

    // Verificar el resultado
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, "Tokens para el usuario:"),
            String::from_str(&env, "Dev"),
            String::from_str(&env, "10"),
        ]
    );
}