export function js_serialize(
    wasm_memory,
    pointer,
    size
) {
    // Create a view the relevant region of the WASM linear memory buffer.
    let view = new Uint8Array(wasm_memory.buffer, pointer, size);

    // Copy it to a new non-view Uint8Array and return
    return new Uint8Array(view);
}

export function js_deserialize(
    wasm_memory,
    pointer,
    custom_element_data
) {
    let buffer_view = new Uint8Array(wasm_memory.buffer);

    buffer_view.set(custom_element_data, pointer);
}
