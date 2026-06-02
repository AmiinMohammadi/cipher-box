const { invoke } = window.__TAURI__.core;

const encBtn = document.querySelector("#encrypt_btn");
const decBtn = document.querySelector("#decrypt_btn");
const copyBtn = document.querySelector("#copy_btn");
const textIn = document.querySelector("#text_input");
const textOut = document.querySelector("#text_output");
const key = document.querySelector("#key");
const errMassage = document.querySelector("#error_msg");
const errBox = document.querySelector(".form-group:last-child");
const hintIcon = document.querySelector("#img_hint");

function showMassage(srcImage, textColor, text, time = 2500) {
    hintIcon.setAttribute("src", srcImage);
    errMassage.style.color = textColor;
    errMassage.innerText = text;
    errBox.style.opacity = 1;
    errBox.style.translate = "0 -15px 0 ";
    setTimeout(() => {
        errBox.style.translate = "";
        errBox.style.opacity = 0;
    }, time);
}

copyBtn.addEventListener('click', async () => {


    const textToCopy = textOut.value;
    try {
        await navigator.clipboard.writeText(textToCopy);
        showMassage("assets/paste.png", "black", "Copied!");
    } catch (e) {
        showMassage("assets/mark.png", "derkred", `Copy operation failed.\nerror: ${e}`, 5000);
    }

})

decBtn.addEventListener('click', async () => {


    const textValue = textIn.value.trim();
    const keyValue = key.value.trim();

    if (!textValue || !keyValue) {
        showMassage("assets/mark.png", "derkred", `The fields are empty!`);
    } else {
        try {
            const result = await invoke("decrypt_func", {
                text: textValue, key: keyValue,
                algorithm: document.querySelector("input[name='encrypt_algo']:checked").value.toLowerCase()
            });

            textOut.value = result;
            showMassage("assets/check.png", "green", "Successful decryption");
            copyBtn.disabled = false;

        } catch (e) {
            showMassage("assets/mark.png", "derkred", `Decryption failed.\nerror: ${e}`, 5000);
        }

    }

});
encBtn.addEventListener("click", async () => {


    const textValue = textIn.value.trim();
    const keyValue = key.value.trim();

    if (!textValue || !keyValue) {
        showMassage("assets/mark.png", "derkred", `The fields are empty!`);
    } else {
        try {
            const result = await invoke("encrypt_func", {
                text: textValue, key: keyValue,
                algorithm: document.querySelector("input[name='encrypt_algo']:checked").value.toLowerCase()
            });

            textOut.value = result;
            showMassage("assets/check.png", "green", "Successful encryption");
            copyBtn.disabled = false;

            copyBtn.style.cursor = "pointer";
        } catch (e) {
            showMassage("assets/mark.png", "derkred", `Encryption failed.\nerror: ${e}`, 5000);
        }
    }
})
encBtn.addEventListener("mousedown", async () => {
    encBtn.style.boxShadow = "var(--down-side)";
    encBtn.style.scale = "0.95";
});
encBtn.addEventListener("mouseup", async () => {
    encBtn.style.boxShadow = "var(--up-side)";
    encBtn.style.scale = "1";
});
decBtn.addEventListener("mousedown", async () => {
    decBtn.style.boxShadow = "var(--down-side)";
    decBtn.style.scale = "0.95";
});
decBtn.addEventListener("mouseup", async () => {
    decBtn.style.boxShadow = "var(--up-side)";
    decBtn.style.scale = "1";
});
copyBtn.addEventListener("mousedown", async () => {
    copyBtn.style.boxShadow = "var(--down-side)";
    copyBtn.style.scale = "0.95";
});
copyBtn.addEventListener("mouseup", async () => {
    copyBtn.style.boxShadow = "var(--up-side)";
    copyBtn.style.scale = "1";
});