export function define_custom_element(
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

            get __arwa_custom_element_data() {
                return this.#arwa_custom_element_data;
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
