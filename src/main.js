//const { invoke } = window.__TAURI__.core;

const genEncBtn = document.querySelector("#generate_encrypt_btn");

const copyEncBtn = document.querySelector("#copy_output_btn");

genEncBtn.addEventListener('mousedown', () => {
    genEncBtn.classList.add("button_press");

});
genEncBtn.addEventListener('mouseup', () => {
    genEncBtn.classList.remove("button_press");

});
copyEncBtn.addEventListener('mousedown', () => {
    copyEncBtn.classList.add("button_press");

});
copyEncBtn.addEventListener('mouseup', () => {
    copyEncBtn.classList.remove("button_press");

});

/*
    decTabBtn.addEventListener('click', () => {
    decFrm.style.zIndex = 2;
    encFrm.style.zIndex = 1;
    decTabBtn.classList.add("pressed");
    encTabBtn.classList.add("unpressed");
    decTabBtn.classList.remove("unpressed");

});

document.addEventListener('DOMContentLoaded', () => {

   
});*/