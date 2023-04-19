export function spawn_worker(script, module, memory, pointer) {
    // Create worker
    let worker = new Worker(script, { type: "module" });

    // Initialize worker with the spawn closure
    worker.postMessage({
        module,
        memory,
        pointer,
    })

    return worker;
}
