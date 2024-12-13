import StellarSdk from 'stellar-sdk';

// Esta función carga la cuenta de Freighter
export async function cargarCuentaFreighter() {
  try {
    // Verifica si Freighter está disponible en el navegador
    if (!window.StellarSdk || !window.StellarSdk.Freighter) {
      throw new Error('Freighter no está disponible en este navegador.');
    }

    // Usa Freighter para cargar la cuenta
    const account = await StellarSdk.Freighter.loadAccount();

    console.log('Cuenta cargada:', account);

    return account;
  } catch (error) {
    console.error('Error al cargar la cuenta desde Freighter:', error);
    throw error;
  }
}
