import { interactuarConContrato } from './contract.js';

async function main() {
  try {
    // Invoca el contrato
    await interactuarConContrato();
  } catch (error) {
    console.error('Error en la aplicación:', error);
  }
}

// Llama la función principal
main();
