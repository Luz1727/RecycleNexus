// contract.js
const StellarSdk = require('stellar-sdk');  // O la librería que uses
const Freighter = window.Freighter; // Accediendo a la extensión Freighter

// Aquí va el código para verificar la conexión con Freighter Wallet
if (typeof Freighter !== 'undefined') {
    console.log('Freighter Wallet is available');

    async function checkWalletConnection() {
        try {
            const account = await Freighter.getPublicKey();
            console.log('Connected with account:', account);

            // Aquí puedes invocar tu contrato o realizar cualquier otra acción
            invokeContract(account); // Función que invoca tu contrato inteligente
        } catch (error) {
            console.error('Error connecting to Freighter:', error);
            alert('Por favor, conecta tu billetera Freighter.');
        }
    }

    checkWalletConnection();
} else {
    console.log('Freighter Wallet is not available. Please install Freighter.');
    alert('Por favor, instala y conecta Freighter Wallet.');
}

// Función para invocar el contrato
async function invokeContract(account) {
    try {
        const contractId = "SAOYOOW5SV5NIL4LREJR7LIKLHVPX4LLU6HRIM3OQHVXBLQJOAABVFZB";  // ID de tu contrato
        const transaction = await StellarSdk.TransactionBuilder.fromXDR(contractId); // Ejemplo de invocación

        // Realiza la invocación del contrato usando Freighter
        await Freighter.signTransaction(transaction);
        console.log('Transaction successful!');
    } catch (error) {
        console.error('Error invoking contract:', error);
    }
}
