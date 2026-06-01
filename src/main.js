const { invoke } = window.__TAURI__.core;

const encBtn = document.querySelector("#encrypt_btn");
const decBtn = document.querySelector("#decrypt_btn");
const copyBtn = document.querySelector("#copy_btn");
const textIn = document.querySelector("#text_input");
const textOut = document.querySelector("#text_output");
const key = document.querySelector("#key");
const errMassage = document.querySelector("#error_msg");


copyBtn.addEventListener('click', async () => {
    errMassage.innerHTML = "";
    const textToCopy = textOut.value;
    if (!textToCopy) {

        errMassage.innerHTML = "result is empty!!";
    } else {
        try {
            await navigator.clipboard.writeText(textToCopy);
        } catch (e) {
            alert("Error: ${e}")
        }
    }
})

decBtn.addEventListener('click', async () => {

    errMassage.innerHTML = "";
    const textValue = textIn.value.trim();
    const keyValue = key.value.trim();

    if (!textValue || !keyValue) {
        errMassage.innerHTML = "the fields are empty!";
    } else {
        try {
            const result = await invoke("decrypt_func", {
                text: textValue, key: keyValue,
                algorithm: document.querySelector("input[name='encrypt_algo']:checked").value.toLowerCase()
            });

            textOut.value = result;
        } catch (e) {
            alert(`Error:${e}`);
        }

    }

});
encBtn.addEventListener("click", async () => {

    errMassage.innerHTML = "";
    const textValue = textIn.value.trim();
    const keyValue = key.value.trim();

    if (!textValue || !keyValue) {
        errMassage.innerHTML = "the fields are empty!";
    } else {
        try {
            const result = await invoke("encrypt_func", {
                text: textValue, key: keyValue,
                algorithm: document.querySelector("input[name='encrypt_algo']:checked").value.toLowerCase()
            });

            textOut.value = result;
        } catch (e) {
            alert(`Error:${e}`);
        }

    }
})


/*encBtn.addEventListener('click', async () => {

    const textValue = textIn.value.trim();
    const keyValue = key.value.trim();


    const result = await invoke("encryptFunc", {
        text: textValue, key: keyValue
    });

    textOut.value = result;

});

function pressStyle(btn, cssClass) {
    btn.classList.add(cssClass);
}

encBtn.addEventListener('mousedown', async () => {
    encBtn.classList.add("button_press");
});
encBtn.addEventListener('mouseup', async () => {
    encBtn.classList.remove("button_press");

});
copyBtn.addEventListener('mousedown', async () => {
    copyBtn.classList.add("button_press");
});
copyBtn.addEventListener('mouseup', async () => {
    copyBtn.classList.remove("button_press");
});
decBtn.addEventListener('mousedown', async () => {
    decBtn.classList.add("button_press");
});
decBtn.addEventListener('mouseup', async () => {
    decBtn.classList.remove("button_press");
});



document.addEventListener('DOMContentLoaded', () => {

   
});*/