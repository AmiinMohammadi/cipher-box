//const { invoke } = window.__TAURI__.core;

const encTabBtn = document.querySelector("#encrypt_tab");
const decTabBtn = document.querySelector("#decrypt_tab");
const encFrm = document.querySelector("#encrypt_frm");
const decFrm = document.querySelector("#decrypt_frm");
encTabBtn.addEventListener('click', () => {
    decFrm.style.zIndex = 1;
    encFrm.style.zIndex = 2;
});
decTabBtn.addEventListener('click', () => {
    decFrm.style.zIndex = 2;
    encFrm.style.zIndex = 1;
});

/*tab_btn.addEventListener('click', () => {

    decrypt_frm.classList.toggle("decrypt_frm_rotate");
    encrypt_frm.classList.toggle("encrypt_frm_rotate");

});
*/
document.addEventListener('DOMContentLoaded', () => {
    tabBtn.classList.toggle("stable_btn");

});