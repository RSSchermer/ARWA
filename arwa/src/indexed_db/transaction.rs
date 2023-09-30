use std::marker;

use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::dom_exception_wrapper;
use crate::indexed_db::KeyPath;

pub struct KeyConfig {
    pub path: KeyPath,
    pub auto_increment: bool,
}

impl KeyConfig {
    fn to_web_sys(&self) -> web_sys::IdbObjectStoreParameters {
        let mut params = web_sys::IdbObjectStoreParameters::new();

        params.key_path(Some(&JsValue::from_str(self.path.as_ref())));
        params.auto_increment(self.auto_increment);

        params
    }
}

dom_exception_wrapper!(GetObjectStoreError);
dom_exception_wrapper!(CreateObjectStoreError);
dom_exception_wrapper!(DeleteObjectStoreError);

pub mod transaction_capability {
    mod write_seal {
        pub trait Seal {}
    }

    pub trait Write: write_seal::Seal {}

    mod upgrade_seal {
        pub trait Seal {
            #[doc(hidden)]
            fn database(&self) -> &web_sys::IdbDatabase;
        }
    }

    pub trait Upgrade: upgrade_seal::Seal {}

    pub struct R {}

    pub struct RW {}

    impl write_seal::Seal for RW {}
    impl Write for RW {}

    pub struct U {
        pub(super) database: web_sys::IdbDatabase,
    }

    impl write_seal::Seal for U {}
    impl Write for U {}

    impl upgrade_seal::Seal for U {
        fn database(&self) -> &web_sys::IdbDatabase {
            &self.database
        }
    }
    impl Upgrade for U {}
}

use std::future::Future;

use futures::TryFutureExt;
use js_sys::Array;
use transaction_capability::*;

use crate::indexed_db::request_future::RequestFuture;

pub type ReadOnlyTransaction = Transaction<R>;
pub type ReadWriteTransaction = Transaction<RW>;
pub type UpgradeTransaction = Transaction<U>;

pub struct Transaction<Capabilities> {
    inner: web_sys::IdbTransaction,
    capabilities: Capabilities,
}

impl Transaction<R> {
    pub(super) fn read_only(transaction: web_sys::IdbTransaction) -> Self {
        Transaction {
            inner: transaction,
            capabilities: transaction_capability::R {},
        }
    }
}

impl Transaction<RW> {
    pub(super) fn read_write(transaction: web_sys::IdbTransaction) -> Self {
        Transaction {
            inner: transaction,
            capabilities: transaction_capability::RW {},
        }
    }
}

impl Transaction<U> {
    pub(super) fn upgrade(
        transaction: web_sys::IdbTransaction,
        database: web_sys::IdbDatabase,
    ) -> Self {
        Transaction {
            inner: transaction,
            capabilities: U { database },
        }
    }
}

impl<Capabilities> Transaction<Capabilities> {
    pub fn commit(self) {
        self.inner.commit().unwrap_throw();
    }

    pub fn abort(self) {
        self.inner.abort().unwrap_throw();
    }

    pub fn object_store(
        &self,
        name: &str,
    ) -> Result<ObjectStore<Capabilities>, GetObjectStoreError> {
        self.inner
            .object_store(name)
            .map(|store| ObjectStore::new(store))
            .map_err(|err| GetObjectStoreError::new(err.unchecked_into()))
    }

    pub fn create_object_store(
        &self,
        name: &str,
        key_config: Option<KeyConfig>,
    ) -> Result<ObjectStore<Capabilities>, CreateObjectStoreError>
    where
        Capabilities: Upgrade,
    {
        let database = self.capabilities.database();

        let res = if let Some(key_config) = key_config {
            database.create_object_store_with_optional_parameters(name, &key_config.to_web_sys())
        } else {
            database.create_object_store(name)
        };

        res.map(|store| ObjectStore::new(store))
            .map_err(|err| CreateObjectStoreError::new(err.unchecked_into()))
    }

    pub fn delete_object_store(&mut self, name: &str) -> Result<(), DeleteObjectStoreError>
    where
        Capabilities: Upgrade,
    {
        self.capabilities
            .database()
            .delete_object_store(name)
            .map_err(|err| DeleteObjectStoreError::new(err.unchecked_into()))
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct CreateIndexOptions {
    pub unique: bool,
    pub multi_entry: bool,
}

// TODO: add methods that an enum for these error types that disambiguate the errors that can occur
// for each of the individual operations per the spec

dom_exception_wrapper!(CountError);
dom_exception_wrapper!(CountWithQueryError);
dom_exception_wrapper!(GetError);
dom_exception_wrapper!(GetKeyError);
dom_exception_wrapper!(GetAllError);
dom_exception_wrapper!(GetAllKeysError);
dom_exception_wrapper!(AddError);
dom_exception_wrapper!(AddWithQueryError);
dom_exception_wrapper!(PutError);
dom_exception_wrapper!(PutWithQueryError);
dom_exception_wrapper!(DeleteError);
dom_exception_wrapper!(ClearError);
dom_exception_wrapper!(IndexError);
dom_exception_wrapper!(CreateIndexError);
dom_exception_wrapper!(DeleteIndexError);
dom_exception_wrapper!(OpenCursorError);
dom_exception_wrapper!(OpenCursorWithQueryError);
dom_exception_wrapper!(OpenKeyCursorError);
dom_exception_wrapper!(OpenKeyCursorWithQueryError);

dom_exception_wrapper!(RequestCountError);
dom_exception_wrapper!(RequestCountWithQueryError);
dom_exception_wrapper!(RequestGetError);
dom_exception_wrapper!(RequestGetKeyError);
dom_exception_wrapper!(RequestGetAllError);
dom_exception_wrapper!(RequestGetAllKeysError);
dom_exception_wrapper!(RequestAddError);
dom_exception_wrapper!(RequestAddWithQueryError);
dom_exception_wrapper!(RequestPutError);
dom_exception_wrapper!(RequestPutWithQueryError);
dom_exception_wrapper!(RequestDeleteError);
dom_exception_wrapper!(RequestClearError);
dom_exception_wrapper!(RequestOpenCursorError);
dom_exception_wrapper!(RequestOpenCursorWithQueryError);
dom_exception_wrapper!(RequestOpenKeyCursorError);
dom_exception_wrapper!(RequestOpenKeyCursorWithQueryError);

pub struct ObjectStore<Capabilities> {
    inner: web_sys::IdbObjectStore,
    _capabilities: marker::PhantomData<Capabilities>,
}

impl<Capabilities> ObjectStore<Capabilities> {
    fn new(inner: web_sys::IdbObjectStore) -> Self {
        ObjectStore {
            inner,
            _capabilities: Default::default(),
        }
    }

    pub fn name(&self) -> String {
        self.inner.name()
    }

    pub fn set_name(&self, name: &str)
    where
        Capabilities: Upgrade,
    {
        self.inner.set_name(name);
    }

    pub fn key_path(&self) -> Option<KeyPath> {
        self.inner.key_path().ok().map(|raw| {
            let raw = raw.as_string().unwrap_or_default();

            KeyPath::parse(&raw.to_string()).unwrap_throw()
        })
    }

    pub fn auto_increment(&self) -> bool {
        self.inner.auto_increment()
    }

    pub fn index_names(&self) -> StoreIndexNames {
        StoreIndexNames {
            inner: self.inner.index_names(),
            current: 0,
        }
    }

    pub fn count(
        &self,
    ) -> Result<impl Future<Output = Result<u32, RequestCountError>>, CountError> {
        self.inner
            .count()
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.as_f64().unwrap_throw() as u32)
                    .map_err(|err| RequestCountError::new(err))
            })
            .map_err(|err| CountError::new(err.unchecked_into()))
    }

    pub fn count_with_query<K>(
        &self,
        query: &K,
    ) -> Result<impl Future<Output = Result<u32, RequestCountWithQueryError>>, CountWithQueryError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .count_with_key(query.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.as_f64().unwrap_throw() as u32)
                    .map_err(|err| RequestCountWithQueryError::new(err))
            })
            .map_err(|err| CountWithQueryError::new(err.unchecked_into()))
    }

    pub fn get<K>(
        &self,
        key: &K,
    ) -> Result<impl Future<Output = Result<Option<JsValue>, RequestGetError>>, GetError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .get(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| if v.is_undefined() { None } else { Some(v) })
                    .map_err(|err| RequestGetError::new(err))
            })
            .map_err(|err| GetError::new(err.unchecked_into()))
    }

    pub fn get_key<K>(
        &self,
        key: &K,
    ) -> Result<impl Future<Output = Result<Option<JsValue>, RequestGetKeyError>>, GetKeyError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .get_key(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| if v.is_undefined() { None } else { Some(v) })
                    .map_err(|err| RequestGetKeyError::new(err))
            })
            .map_err(|err| GetKeyError::new(err.unchecked_into()))
    }

    pub fn get_all(
        &self,
    ) -> Result<impl Future<Output = Result<Array, RequestGetAllError>>, GetAllError> {
        self.inner
            .get_all()
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.unchecked_into())
                    .map_err(|err| RequestGetAllError::new(err))
            })
            .map_err(|err| GetAllError::new(err.unchecked_into()))
    }

    pub fn get_all_keys(
        &self,
    ) -> Result<impl Future<Output = Result<Array, RequestGetAllKeysError>>, GetAllKeysError> {
        self.inner
            .get_all_keys()
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.unchecked_into())
                    .map_err(|err| RequestGetAllKeysError::new(err))
            })
            .map_err(|err| GetAllKeysError::new(err.unchecked_into()))
    }

    pub fn get_all_with_key<K>(
        &self,
        key: &K,
    ) -> Result<impl Future<Output = Result<Array, RequestGetAllError>>, GetAllError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .get_all_with_key(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.unchecked_into())
                    .map_err(|err| RequestGetAllError::new(err))
            })
            .map_err(|err| GetAllError::new(err.unchecked_into()))
    }

    pub fn get_all_keys_with_key<K>(
        &self,
        key: &K,
    ) -> Result<impl Future<Output = Result<Array, RequestGetAllKeysError>>, GetAllKeysError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .get_all_keys_with_key(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.unchecked_into())
                    .map_err(|err| RequestGetAllKeysError::new(err))
            })
            .map_err(|err| GetAllKeysError::new(err.unchecked_into()))
    }

    pub fn open_cursor(
        &self,
        direction: CursorDirection,
    ) -> Result<
        impl Future<Output = Result<Option<ValueCursor<Capabilities>>, RequestOpenCursorError>>,
        OpenCursorError,
    > {
        self.inner
            .open_cursor_with_range_and_direction(&JsValue::null(), direction.to_web_sys())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(ValueCursor::new(v.unchecked_into()))
                        }
                    })
                    .map_err(|err| RequestOpenCursorError::new(err))
            })
            .map_err(|err| OpenCursorError::new(err.unchecked_into()))
    }

    pub fn open_cursor_with_query<K>(
        &self,
        query: &K,
        direction: CursorDirection,
    ) -> Result<
        impl Future<Output = Result<Option<ValueCursor<Capabilities>>, RequestOpenCursorWithQueryError>>,
        OpenCursorWithQueryError,
    >
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .open_cursor_with_range_and_direction(query.as_ref(), direction.to_web_sys())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(ValueCursor::new(v.unchecked_into()))
                        }
                    })
                    .map_err(|err| RequestOpenCursorWithQueryError::new(err))
            })
            .map_err(|err| OpenCursorWithQueryError::new(err.unchecked_into()))
    }

    pub fn open_key_cursor(
        &self,
        direction: CursorDirection,
    ) -> Result<
        impl Future<Output = Result<Option<KeyCursor<Capabilities>>, RequestOpenKeyCursorError>>,
        OpenKeyCursorError,
    > {
        self.inner
            .open_key_cursor_with_range_and_direction(&JsValue::null(), direction.to_web_sys())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(KeyCursor::new(v.unchecked_into()))
                        }
                    })
                    .map_err(|err| RequestOpenKeyCursorError::new(err))
            })
            .map_err(|err| OpenKeyCursorError::new(err.unchecked_into()))
    }

    pub fn open_key_cursor_with_query<K>(
        &self,
        query: &K,
        direction: CursorDirection,
    ) -> Result<
        impl Future<
            Output = Result<Option<KeyCursor<Capabilities>>, RequestOpenKeyCursorWithQueryError>,
        >,
        OpenKeyCursorWithQueryError,
    >
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .open_key_cursor_with_range_and_direction(query.as_ref(), direction.to_web_sys())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(KeyCursor::new(v.unchecked_into()))
                        }
                    })
                    .map_err(|err| RequestOpenKeyCursorWithQueryError::new(err))
            })
            .map_err(|err| OpenKeyCursorWithQueryError::new(err.unchecked_into()))
    }

    pub fn add<V>(
        &self,
        value: &V,
    ) -> Result<impl Future<Output = Result<(), RequestAddError>>, AddError>
    where
        Capabilities: Write,
        V: AsRef<JsValue>,
    {
        self.inner
            .add(value.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|_| ())
                    .map_err(|err| RequestAddError::new(err))
            })
            .map_err(|err| AddError::new(err.unchecked_into()))
    }

    pub fn add_with_key<K, V>(
        &self,
        key: &K,
        value: &V,
    ) -> Result<impl Future<Output = Result<(), RequestAddWithQueryError>>, AddWithQueryError>
    where
        Capabilities: Write,
        K: AsRef<JsValue>,
        V: AsRef<JsValue>,
    {
        self.inner
            .add_with_key(value.as_ref(), key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|_| ())
                    .map_err(|err| RequestAddWithQueryError::new(err))
            })
            .map_err(|err| AddWithQueryError::new(err.unchecked_into()))
    }

    pub fn put<V>(
        &self,
        value: &V,
    ) -> Result<impl Future<Output = Result<(), RequestPutError>>, PutError>
    where
        Capabilities: Write,
        V: AsRef<JsValue>,
    {
        self.inner
            .put(value.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|_| ())
                    .map_err(|err| RequestPutError::new(err))
            })
            .map_err(|err| PutError::new(err.unchecked_into()))
    }

    pub fn put_with_key<K, V>(
        &self,
        key: &K,
        value: &V,
    ) -> Result<impl Future<Output = Result<(), RequestPutWithQueryError>>, PutWithQueryError>
    where
        Capabilities: Write,
        K: AsRef<JsValue>,
        V: AsRef<JsValue>,
    {
        self.inner
            .put_with_key(value.as_ref(), key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|_| ())
                    .map_err(|err| RequestPutWithQueryError::new(err))
            })
            .map_err(|err| PutWithQueryError::new(err.unchecked_into()))
    }

    pub fn delete<K>(
        &self,
        key: &K,
    ) -> Result<impl Future<Output = Result<(), RequestDeleteError>>, DeleteError>
    where
        Capabilities: Write,
        K: AsRef<JsValue>,
    {
        self.inner
            .delete(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|_| ())
                    .map_err(|err| RequestDeleteError::new(err))
            })
            .map_err(|err| DeleteError::new(err.unchecked_into()))
    }

    pub fn clear(&self) -> Result<impl Future<Output = Result<(), RequestClearError>>, ClearError>
    where
        Capabilities: Write,
    {
        self.inner
            .clear()
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|_| ())
                    .map_err(|err| RequestClearError::new(err))
            })
            .map_err(|err| ClearError::new(err.unchecked_into()))
    }

    pub fn index(&self, name: &str) -> Result<Index<Capabilities>, IndexError> {
        self.inner
            .index(name)
            .map(|i| Index::new(i))
            .map_err(|err| IndexError::new(err.unchecked_into()))
    }

    pub fn create_index(
        &self,
        name: &str,
        key_path: &KeyPath,
        options: &CreateIndexOptions,
    ) -> Result<Index<Capabilities>, CreateIndexError>
    where
        Capabilities: Upgrade,
    {
        let mut params = web_sys::IdbIndexParameters::new();

        params.unique(options.unique);
        params.multi_entry(options.multi_entry);

        self.inner
            .create_index_with_str_and_optional_parameters(name, key_path.as_ref(), &params)
            .map(|i| Index::new(i))
            .map_err(|err| CreateIndexError::new(err.unchecked_into()))
    }

    pub fn delete_index(&self, name: &str) -> Result<(), DeleteIndexError>
    where
        Capabilities: Upgrade,
    {
        self.inner
            .delete_index(name)
            .map(|_| ())
            .map_err(|err| DeleteIndexError::new(err.unchecked_into()))
    }
}

pub struct StoreIndexNames {
    inner: web_sys::DomStringList,
    current: u32,
}

impl Iterator for StoreIndexNames {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.inner.get(self.current);

        if current.is_some() {
            self.current += 1;
        }

        current
    }
}

pub struct Index<Capabilities> {
    inner: web_sys::IdbIndex,
    _capabilities: marker::PhantomData<Capabilities>,
}

impl<Capabilities> Index<Capabilities> {
    fn new(inner: web_sys::IdbIndex) -> Self {
        Index {
            inner,
            _capabilities: Default::default(),
        }
    }

    pub fn count_with_query<K>(
        &self,
        query: &K,
    ) -> Result<impl Future<Output = Result<u32, RequestCountWithQueryError>>, CountWithQueryError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .count_with_key(query.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.as_f64().unwrap_throw() as u32)
                    .map_err(|err| RequestCountWithQueryError::new(err))
            })
            .map_err(|err| CountWithQueryError::new(err.unchecked_into()))
    }

    pub fn get<K>(
        &self,
        key: &K,
    ) -> Result<impl Future<Output = Result<Option<JsValue>, RequestGetError>>, GetError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .get(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| if v.is_undefined() { None } else { Some(v) })
                    .map_err(|err| RequestGetError::new(err))
            })
            .map_err(|err| GetError::new(err.unchecked_into()))
    }

    pub fn get_key<K>(
        &self,
        key: &K,
    ) -> Result<impl Future<Output = Result<Option<JsValue>, RequestGetKeyError>>, GetKeyError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .get_key(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| if v.is_undefined() { None } else { Some(v) })
                    .map_err(|err| RequestGetKeyError::new(err))
            })
            .map_err(|err| GetKeyError::new(err.unchecked_into()))
    }

    pub fn get_all(
        &self,
    ) -> Result<impl Future<Output = Result<Array, RequestGetAllError>>, GetAllError> {
        self.inner
            .get_all()
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.unchecked_into())
                    .map_err(|err| RequestGetAllError::new(err))
            })
            .map_err(|err| GetAllError::new(err.unchecked_into()))
    }

    pub fn get_all_keys(
        &self,
    ) -> Result<impl Future<Output = Result<Array, RequestGetAllKeysError>>, GetAllKeysError> {
        self.inner
            .get_all_keys()
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.unchecked_into())
                    .map_err(|err| RequestGetAllKeysError::new(err))
            })
            .map_err(|err| GetAllKeysError::new(err.unchecked_into()))
    }

    pub fn get_all_with_query<K>(
        &self,
        query: &K,
    ) -> Result<impl Future<Output = Result<Array, RequestGetAllError>>, GetAllError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .get_all_with_key(query.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.unchecked_into())
                    .map_err(|err| RequestGetAllError::new(err))
            })
            .map_err(|err| GetAllError::new(err.unchecked_into()))
    }

    pub fn get_all_keys_with_query<K>(
        &self,
        query: &K,
    ) -> Result<impl Future<Output = Result<Array, RequestGetAllKeysError>>, GetAllKeysError>
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .get_all_keys_with_key(query.as_ref())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| v.unchecked_into())
                    .map_err(|err| RequestGetAllKeysError::new(err))
            })
            .map_err(|err| GetAllKeysError::new(err.unchecked_into()))
    }

    pub fn open_cursor(
        &self,
        direction: CursorDirection,
    ) -> Result<
        impl Future<Output = Result<Option<ValueCursor<Capabilities>>, RequestOpenCursorError>>,
        OpenCursorError,
    > {
        self.inner
            .open_cursor_with_range_and_direction(&JsValue::null(), direction.to_web_sys())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(ValueCursor::new(v.unchecked_into()))
                        }
                    })
                    .map_err(|err| RequestOpenCursorError::new(err))
            })
            .map_err(|err| OpenCursorError::new(err.unchecked_into()))
    }

    pub fn open_cursor_with_query<K>(
        &self,
        query: &K,
        direction: CursorDirection,
    ) -> Result<
        impl Future<Output = Result<Option<ValueCursor<Capabilities>>, RequestOpenCursorWithQueryError>>,
        OpenCursorWithQueryError,
    >
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .open_cursor_with_range_and_direction(query.as_ref(), direction.to_web_sys())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(ValueCursor::new(v.unchecked_into()))
                        }
                    })
                    .map_err(|err| RequestOpenCursorWithQueryError::new(err))
            })
            .map_err(|err| OpenCursorWithQueryError::new(err.unchecked_into()))
    }

    pub fn open_key_cursor(
        &self,
        direction: CursorDirection,
    ) -> Result<
        impl Future<Output = Result<Option<KeyCursor<Capabilities>>, RequestOpenKeyCursorError>>,
        OpenKeyCursorError,
    > {
        self.inner
            .open_key_cursor_with_range_and_direction(&JsValue::null(), direction.to_web_sys())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(KeyCursor::new(v.unchecked_into()))
                        }
                    })
                    .map_err(|err| RequestOpenKeyCursorError::new(err))
            })
            .map_err(|err| OpenKeyCursorError::new(err.unchecked_into()))
    }

    pub fn open_key_cursor_with_query<K>(
        &self,
        query: &K,
        direction: CursorDirection,
    ) -> Result<
        impl Future<
            Output = Result<Option<KeyCursor<Capabilities>>, RequestOpenKeyCursorWithQueryError>,
        >,
        OpenKeyCursorWithQueryError,
    >
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .open_key_cursor_with_range_and_direction(query.as_ref(), direction.to_web_sys())
            .map(|ok| {
                RequestFuture::from_request(ok)
                    .map_ok(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(KeyCursor::new(v.unchecked_into()))
                        }
                    })
                    .map_err(|err| RequestOpenKeyCursorWithQueryError::new(err))
            })
            .map_err(|err| OpenKeyCursorWithQueryError::new(err.unchecked_into()))
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CursorDirection {
    Next,
    NextUnique,
    Prev,
    PrevUnique,
}

impl Default for CursorDirection {
    fn default() -> Self {
        CursorDirection::Next
    }
}

impl CursorDirection {
    fn to_web_sys(&self) -> web_sys::IdbCursorDirection {
        match self {
            CursorDirection::Next => web_sys::IdbCursorDirection::Next,
            CursorDirection::NextUnique => web_sys::IdbCursorDirection::Nextunique,
            CursorDirection::Prev => web_sys::IdbCursorDirection::Prev,
            CursorDirection::PrevUnique => web_sys::IdbCursorDirection::Prevunique,
        }
    }
}

dom_exception_wrapper!(CursorKeyError);
dom_exception_wrapper!(CursorAdvanceError);
dom_exception_wrapper!(CursorContinueKeyError);
dom_exception_wrapper!(CursorContinuePrimaryKeyError);
dom_exception_wrapper!(CursorUpdateError);
dom_exception_wrapper!(CursorDeleteError);

dom_exception_wrapper!(RequestCursorAdvanceError);
dom_exception_wrapper!(RequestCursorContinueKeyError);
dom_exception_wrapper!(RequestCursorContinuePrimaryKeyError);
dom_exception_wrapper!(RequestCursorUpdateError);
dom_exception_wrapper!(RequestCursorDeleteError);

mod cursor_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys(&self) -> &web_sys::IdbCursor;
    }
}

pub trait Cursor: cursor_seal::Seal {
    fn direction(&self) -> CursorDirection {
        match self.as_web_sys().direction() {
            web_sys::IdbCursorDirection::Next => CursorDirection::Next,
            web_sys::IdbCursorDirection::Nextunique => CursorDirection::NextUnique,
            web_sys::IdbCursorDirection::Prev => CursorDirection::Prev,
            web_sys::IdbCursorDirection::Prevunique => CursorDirection::PrevUnique,
            _ => unreachable!(),
        }
    }

    fn key(&self) -> Option<JsValue> {
        // I can't find any indication in the spec that this can actually fail
        let v = self.as_web_sys().key().unwrap_throw();

        if v.is_undefined() {
            None
        } else {
            Some(v)
        }
    }

    fn primary_key(&self) -> Option<JsValue> {
        // I can't find any indication in the spec that this can actually fail
        let v = self.as_web_sys().primary_key().unwrap_throw();

        if v.is_undefined() {
            None
        } else {
            Some(v)
        }
    }
}

pub struct KeyCursor<Capabilities> {
    inner: web_sys::IdbCursor,
    _capabilities: marker::PhantomData<Capabilities>,
}

impl<Capabilities> KeyCursor<Capabilities> {
    fn new(inner: web_sys::IdbCursor) -> Self {
        KeyCursor {
            inner,
            _capabilities: Default::default(),
        }
    }

    pub fn advance(
        self,
        count: u32,
    ) -> Result<
        impl Future<Output = Result<Option<KeyCursor<Capabilities>>, RequestCursorAdvanceError>>,
        CursorAdvanceError,
    > {
        assert!(count > 0, "count must not be zero");

        self.inner
            .advance(count)
            .map(|ok| {
                RequestFuture::from_request(self.inner.request())
                    .map_ok(|v| if v.is_null() { None } else { Some(self) })
                    .map_err(|err| RequestCursorAdvanceError::new(err))
            })
            .map_err(|err| CursorAdvanceError::new(err.unchecked_into()))
    }

    pub fn continue_key<K>(
        self,
        key: &K,
    ) -> Result<
        impl Future<Output = Result<Option<KeyCursor<Capabilities>>, RequestCursorContinueKeyError>>,
        CursorContinueKeyError,
    >
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .continue_with_key(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(self.inner.request())
                    .map_ok(|v| if v.is_null() { None } else { Some(self) })
                    .map_err(|err| RequestCursorContinueKeyError::new(err))
            })
            .map_err(|err| CursorContinueKeyError::new(err.unchecked_into()))
    }

    pub fn continue_primary_key<K, P>(
        self,
        key: &K,
        primary_key: &P,
    ) -> Result<
        impl Future<Output = Result<Option<KeyCursor<Capabilities>>, RequestCursorContinueKeyError>>,
        CursorContinueKeyError,
    >
    where
        K: AsRef<JsValue>,
        P: AsRef<JsValue>,
    {
        self.inner
            .continue_primary_key(key.as_ref(), primary_key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(self.inner.request())
                    .map_ok(|v| if v.is_null() { None } else { Some(self) })
                    .map_err(|err| RequestCursorContinueKeyError::new(err))
            })
            .map_err(|err| CursorContinueKeyError::new(err.unchecked_into()))
    }
}

impl<Capabilities> cursor_seal::Seal for KeyCursor<Capabilities> {
    fn as_web_sys(&self) -> &web_sys::IdbCursor {
        self.inner.as_ref()
    }
}
impl<Capabilities> Cursor for KeyCursor<Capabilities> {}

pub struct ValueCursor<Capabilities> {
    inner: web_sys::IdbCursorWithValue,
    _capabilities: marker::PhantomData<Capabilities>,
}

impl<Capabilities> ValueCursor<Capabilities> {
    fn new(inner: web_sys::IdbCursorWithValue) -> Self {
        ValueCursor {
            inner,
            _capabilities: Default::default(),
        }
    }

    pub fn value(&self) -> Option<JsValue> {
        // I can't find any indication in the spec that this can actually fail
        let v = self.inner.value().unwrap_throw();

        if v.is_undefined() {
            None
        } else {
            Some(v)
        }
    }

    pub fn advance(
        self,
        count: u32,
    ) -> Result<
        impl Future<Output = Result<Option<ValueCursor<Capabilities>>, RequestCursorAdvanceError>>,
        CursorAdvanceError,
    > {
        assert!(count > 0, "count must not be zero");

        self.inner
            .advance(count)
            .map(|ok| {
                RequestFuture::from_request(self.inner.request())
                    .map_ok(|v| if v.is_null() { None } else { Some(self) })
                    .map_err(|err| RequestCursorAdvanceError::new(err))
            })
            .map_err(|err| CursorAdvanceError::new(err.unchecked_into()))
    }

    pub fn continue_key<K>(
        self,
        key: &K,
    ) -> Result<
        impl Future<Output = Result<Option<ValueCursor<Capabilities>>, RequestCursorContinueKeyError>>,
        CursorContinueKeyError,
    >
    where
        K: AsRef<JsValue>,
    {
        self.inner
            .continue_with_key(key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(self.inner.request())
                    .map_ok(|v| if v.is_null() { None } else { Some(self) })
                    .map_err(|err| RequestCursorContinueKeyError::new(err))
            })
            .map_err(|err| CursorContinueKeyError::new(err.unchecked_into()))
    }

    pub fn continue_primary_key<K, P>(
        self,
        key: &K,
        primary_key: &P,
    ) -> Result<
        impl Future<Output = Result<Option<ValueCursor<Capabilities>>, RequestCursorContinueKeyError>>,
        CursorContinueKeyError,
    >
    where
        K: AsRef<JsValue>,
        P: AsRef<JsValue>,
    {
        self.inner
            .continue_primary_key(key.as_ref(), primary_key.as_ref())
            .map(|ok| {
                RequestFuture::from_request(self.inner.request())
                    .map_ok(|v| if v.is_null() { None } else { Some(self) })
                    .map_err(|err| RequestCursorContinueKeyError::new(err))
            })
            .map_err(|err| CursorContinueKeyError::new(err.unchecked_into()))
    }

    pub fn update<V>(
        self,
        value: &V,
    ) -> Result<
        impl Future<Output = Result<ValueCursor<Capabilities>, RequestCursorUpdateError>>,
        CursorUpdateError,
    >
    where
        Capabilities: Write,
        V: AsRef<JsValue>,
    {
        self.inner
            .update(value.as_ref())
            .map(|ok| {
                RequestFuture::from_request(self.inner.request())
                    .map_ok(|_| self)
                    .map_err(|err| RequestCursorUpdateError::new(err))
            })
            .map_err(|err| CursorUpdateError::new(err.unchecked_into()))
    }

    pub fn delete(
        self,
    ) -> Result<
        impl Future<Output = Result<ValueCursor<Capabilities>, RequestCursorDeleteError>>,
        CursorDeleteError,
    >
    where
        Capabilities: Write,
    {
        self.inner
            .delete()
            .map(|ok| {
                RequestFuture::from_request(self.inner.request())
                    .map_ok(|_| self)
                    .map_err(|err| RequestCursorDeleteError::new(err))
            })
            .map_err(|err| CursorDeleteError::new(err.unchecked_into()))
    }
}

impl<Capabilities> cursor_seal::Seal for ValueCursor<Capabilities> {
    fn as_web_sys(&self) -> &web_sys::IdbCursor {
        self.inner.as_ref()
    }
}
impl<Capabilities> Cursor for ValueCursor<Capabilities> {}
