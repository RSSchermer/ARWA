import init, {__arwa_init_spawned_worker} from "__WBG_SHIM_SCRIPT_PATH__";

let initialize = function(messageEvent) {
    // Remove the initializer after first event so we don't collide with user-defined
    // message listeners
    removeEventListener("message", initialize);

    let { module, memory, pointer } = messageEvent.data;

    init(module, memory).catch(err => {
        console.log(err);

        // Propagate to main `onerror`:
        setTimeout(() => {
            throw err;
        });
        // Rethrow to keep promise rejected and prevent execution of further commands:
        throw err;
    }).then(function() {
        __arwa_init_spawned_worker(pointer);
    });
}

addEventListener("message", initialize);
