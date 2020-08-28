/**
 * @param {HTMLElement} elem
 * @param {Response} response 
 */
async function setStatus(elem, response) {
    if (response.status !== 200) {
        elem.innerHTML = "Failed :("
    } else if (await response.text() === 'ok') {
        elem.innerHTML = "Sent!";
    } else {
        elem.innerHTML = response;
    }
}
