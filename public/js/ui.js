/* ============================================================
   Generic info popup (used for websocket close reasons, etc.)
   ============================================================ */
function showPopup(message) {
    const overlay = document.createElement('div');
    overlay.classList.add('popup-overlay');

    const box = document.createElement('div');
    box.classList.add('popup-box');

    const text = document.createElement('p');
    text.textContent = message;

    const okBtn = document.createElement('button');
    okBtn.classList.add('ok-btn');
    okBtn.textContent = 'OK';
    okBtn.addEventListener('click', () => overlay.remove());

    box.appendChild(text);
    box.appendChild(okBtn);
    overlay.appendChild(box);
    document.body.appendChild(overlay);
}
