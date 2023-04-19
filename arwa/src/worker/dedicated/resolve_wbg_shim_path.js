// Hack, modified from https://github.com/chemicstry/wasm_thread
(function() {
    try {
        throw new Error();
    } catch (e) {
        let parts = e.stack.match(/(?:\(|@)(\S+):\d+:\d+/);

        return parts[1];
    }
})()
