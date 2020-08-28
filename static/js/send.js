/**
 * @param {object<string, string>} queryPair 
 */
function createSearchParam(queryPair) {
    const searchParam = new URLSearchParams();
    Object.keys(queryPair).map((k) => {
        const v = queryPair[k];
        searchParam.append(k, v);
    })

    return searchParam.toString();
}

/**
 * @param string message 
 */
async function sendMessage(message) {
    return await fetch(`/api/publish?${createSearchParam({message})}`);
}
