export function __arwa_js_serialize(
    wasm_memory,
    pointer,
    size
) {
    // Create a view the relevant region of the WASM linear memory buffer.
    let view = new Uint8Array(wasm_memory.buffer, pointer, size);

    // Copy it to a new non-view Uint8Array and return
    return new Uint8Array(view);
}

export function __arwa_js_deserialize(
    wasm_memory,
    pointer,
    custom_element_data
) {
    let buffer_view = new Uint8Array(wasm_memory.buffer);

    buffer_view.set(custom_element_data, pointer);
}

export function __arwa_define_custom_element(
    name,
    extendedName,
    constructor,
    connectedCallback,
    disconnectedCallback,
    adoptedCallback,
    attributeChangedCallback,
    observedAttributes,
) {
    let extendedType = extendedName ? document.createElement(extendedName).constructor : HTMLElement;

    customElements.define(
        name,
        class extends extendedType {
            #arwa_custom_element_data;

            static get observedAttributes() {
                return observedAttributes;
            }

            constructor() {
                super();

                this.#arwa_custom_element_data = constructor(this);
            }

            __deserialize_custom_element_data(wasm_linear_memory_buffer, pointer) {
                return __arwa_js_deserialize(
                    wasm_linear_memory_buffer,
                    pointer,
                    this.#arwa_custom_element_data
                );
            }

            connectedCallback() {
                connectedCallback(this);
            }

            disconnectedCallback() {
                disconnectedCallback(this);
            }

            adoptedCallback() {
                adoptedCallback(this);
            }

            attributeChangedCallback(name, oldValue, newValue) {
                attributeChangedCallback(this, name, oldValue, newValue)
            }
        },
        extendedName ? { extends: extendedName } : undefined
    );
}

export function __arwa_create_readable_stream(underlyingSource, queuingStrategy) {
    return new ReadableStream(underlyingSource, queuingStrategy);
}

export function __arwa_create_writable_stream(underlyingSink, queuingStrategy) {
    return new WritableStream(underlyingSink, queuingStrategy);
}

export function __arwa_create_transform_stream(underlyingSink, writableStrategy, readableStrategy) {
    return new TransformStream(underlyingSink, writableStrategy, readableStrategy);
}
