export function spawn_worker(module, memory, pointer) {
    const WORKER_SCRIPT = `
        let initialize = function(messageEvent) {
            let { module, memory, pointer } = messageEvent.data;
            
            let imports = { wbg: { memory } };
            
            WebAssembly.instantiate(module, imports).then(function(instance) {
                instance.exports.__arwa_init_spawned_worker(pointer);
            });
  
            // Remove the initializer after first event so we don't collide with user-defined 
            // message listeners
            removeEventListener("message", initialize);
        }
        
        addEventListener("message", initialize);
    `;

    // Create worker
    let worker = new Worker(URL.createObjectURL(new Blob(
        [WORKER_SCRIPT],
        {type: 'application/javascript'}
    )));

    console.log(memory);

    // Initialize worker with the spawn closure
    worker.postMessage({
        module,
        memory,
        pointer,
    })

    return worker;
}
