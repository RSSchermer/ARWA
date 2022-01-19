pub(crate) mod message_sender_seal {
    pub trait Seal {}
}

pub trait MessageSender: message_sender_seal::Seal {
    // TODO: add `post_message` method. Figure out how to define the message type. This can be
    // simply `JsValue`, however, this can trigger errors when parts of that `JsValue` do not
    // support "structured cloning". I think we might want to approach this in a similar way to
    // how `serde` approaches serialization, except instead of `serde`'s data model, the data model
    // would consist of all structurally cloneable types. On the message receiver end, do use
    // `JsValue`, then provide something similar to `serde`'s Deserialization derive macro to
    // attempt to translate it into a precise type.

    // TODO: figure out what to do about transferables. It seems that to transfer a Transferable,
    // you include it both in the `data`, and then list it in the transfer list to have it
    // transferred rather than copied. You can then access the transferred data through
    // MessageEvent::data on the receiving end like you would copied data... Except when the
    // transferable is a MessagePort, in which case you don't necessarily have to include it in
    // `data`, but it will somehow appear in `MessageEvent::ports`, at some unspecced index?
    //
    // There is also the issue of "neutering", which is where if you try to use an object after it
    // has been transferred, you will trigger errors. This is to avoid race-conditions, but the
    // "neutering" side-effect seems really unfortunate.
    //
    // What I'd like to do ideally is leverage Rust's ownership semantics with some sort of
    // `TransferBox` wrapper. Creating such a box would be a fallible operation, resulting in an
    // error if the object reference passed to the constructor is not unique. You cannot do anything
    // with a successfully created `TransferBox`, except take the boxed item back out, destroying
    // the box. Including a TransferBox in a message would then result in the object being
    // transferred, whereas all unboxed objects in the message would be copied, thus also
    // eliminating the need for a separate `transferList` to marks the objects that are to be
    // transferred. This however requires a way to ask the browser to confirm that a given object
    // reference is the *only* live reference to that object. I don't think this is currently
    // possible, even with WeakRef and FinalizationRegistry, but perhaps the WASM garbage collection
    // proposal will allow us to do something like this eventually?
}
