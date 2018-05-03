var forge = require('node-forge');
//forge.options.usePureJavaScript=true;

var randomstring = require('randomstring');
const js = import("./app_wasm");
js.then(js => {



    // Client: Creates public/private key
    // Client sends public key and text to service
    // Service returns encrypted text, and encrypted symmetric key
    // Client decrypts symmetric key using private key
    // Client descrypts text using decrypted symmetric key

    var text = generateText();
    setFromText(text);
    // We generate a public/private key

    var rsa = forge.pki.rsa;
    var keypair = rsa.generateKeyPair({bits: 2048, e: 0x10001});

    // we send the public key and text
    var publicKey_pem = forge.pki.publicKeyToPem(keypair.publicKey);
    var encryptedData = Encrypt_Javascript(publicKey_pem, text);

    var encryptedSymmetricKey = encryptedData.encryptedSymmetricKey;
    var symmetricKey = keypair.privateKey.decrypt(encryptedSymmetricKey);
    //var symmetricKey = encryptedData.symmetricKey;


    var plainText="xx";
    var a = performance.now()
    console.time('decrypt');
    for (var i = 0; i < 500; i++) {
        var buffer = forge.util.createBuffer(encryptedData.encryptedText.bytes(), 'raw');
        plainText = Decrypt_Javascript(symmetricKey, encryptedData.iv, buffer);
    }
    var b = performance.now();
    setToText(i, plainText);
    setTimer(b-a);


    /*
    function process() {
        let from = $('#from').val();
        let to = $('#to').val();
        let text = $('#text').val();
        js.text_replace(from, to, text);
    }

    $(function() {
        $("input").keyup(process);
    });
    */
})

function generateText() {

    var s = "";
    for (var i = 0; i < 10000; i++)
    s = s + randomstring.generate({
        length: 8,
        charset: 'alphabetic'
    }) +' '
    return s;
}

function setFromText(str) {
    $('#from').text(str);
}

function setToText(iteration, str) {
    $('#iteration').text(iteration);
    $('#to').text(str);
}

function setTimer(timer) {
    $('#timer').text(timer)
}

function Encrypt_Javascript(publicKey_pem, text) {
    // generate a random key and IV
    // Note: a key size of 16 bytes will use AES-128, 24 => AES-192, 32 => AES-256
    var symmetricKey = forge.random.getBytesSync(32);
    var iv = forge.random.getBytesSync(16);

    var publicKey = forge.pki.publicKeyFromPem(publicKey_pem);
    var encryptedSymmetricKey = publicKey.encrypt(symmetricKey);

    // encrypt some bytes using CBC mode
    // (other modes include: CFB, OFB, CTR, and GCM)
    var cipher = forge.cipher.createCipher('AES-CBC', symmetricKey);
    cipher.start({iv: iv});
    cipher.update(new forge.util.ByteBuffer(text, 'utf8'));
    cipher.finish();
    var encryptedText = cipher.output;
    return { symmetricKey: symmetricKey, encryptedSymmetricKey: encryptedSymmetricKey, encryptedText: encryptedText, iv: iv };
}

function Decrypt_Javascript(symmetricKey, iv, encryptedText)
{
    var decipher = forge.cipher.createDecipher('AES-CBC', symmetricKey);
    decipher.start({iv: iv});
    decipher.update(encryptedText);
    decipher.finish();
    var plainText = decipher.output.toString('utf8')
    return plainText;
}

export function update_text(s) {
    $('#result').text(s);
}

