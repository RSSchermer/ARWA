use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use delegate::delegate;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Client as WebSysClient, ServiceWorkerRegistration};

use crate::fetch::{cache_context_seal, CacheContext};
use crate::message::{
    message_event_target_seal, message_sender_seal, MessageEventTarget, MessageSender,
};
use crate::unchecked_cast_array::unchecked_cast_array;
use crate::url::Url;
use crate::worker::impl_worker_global_scope_traits;
use crate::{dom_exception_wrapper, impl_common_wrapper_traits, type_error_wrapper, InvalidCast};

#[derive(Clone)]
pub struct ServiceWorkerGlobalScope {
    inner: web_sys::ServiceWorkerGlobalScope,
}

impl ServiceWorkerGlobalScope {
    pub fn clients(&self) -> Clients {
        self.inner.clients().into()
    }

    pub fn registration(&self) -> ServiceWorkerRegistration {
        self.inner.registration().into()
    }
}

impl message_event_target_seal::Seal for ServiceWorkerGlobalScope {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl MessageEventTarget for ServiceWorkerGlobalScope {}

impl cache_context_seal::Seal for ServiceWorkerGlobalScope {}

impl CacheContext for ServiceWorkerGlobalScope {}

impl From<web_sys::ServiceWorkerGlobalScope> for ServiceWorkerGlobalScope {
    fn from(inner: web_sys::ServiceWorkerGlobalScope) -> Self {
        ServiceWorkerGlobalScope { inner }
    }
}

impl AsRef<web_sys::ServiceWorkerGlobalScope> for ServiceWorkerGlobalScope {
    fn as_ref(&self) -> &web_sys::ServiceWorkerGlobalScope {
        &self.inner
    }
}

impl_worker_global_scope_traits!(ServiceWorkerGlobalScope, ServiceWorkerGlobalScope);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MatchClientType {
    Window,
    DedicatedWorker,
    SharedWorker,
    All,
}

impl MatchClientType {
    fn to_web_sys(&self) -> web_sys::ClientType {
        match self {
            MatchClientType::Window => web_sys::ClientType::Window,
            MatchClientType::DedicatedWorker => web_sys::ClientType::Worker,
            MatchClientType::SharedWorker => web_sys::ClientType::Sharedworker,
            MatchClientType::All => web_sys::ClientType::All,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ClientQuery {
    pub client_type: MatchClientType,
    pub include_uncontrolled: bool,
}

#[derive(Clone)]
pub struct Clients {
    inner: web_sys::Clients,
}

impl Clients {
    pub fn query_id(&self, client_id: &str) -> ClientsQueryId {
        ClientsQueryId {
            init: Some(QueryIdInit {
                clients: self.inner.clone(),
                client_id: client_id.to_string(),
            }),
            inner: None,
        }
    }

    pub fn query_all(&self, query: ClientQuery) -> ClientsQueryAll {
        let mut query_options = web_sys::ClientQueryOptions::new();

        query_options.include_uncontrolled(query.include_uncontrolled);
        query_options.type_(query.client_type.to_web_sys());

        ClientsQueryAll {
            init: Some(QueryAllInit {
                clients: self.inner.clone(),
                query_options,
            }),
            inner: None,
        }
    }

    pub fn claim(&self) -> ClaimClients {
        ClaimClients {
            clients: Some(self.inner.clone()),
            inner: None,
        }
    }

    pub fn open_window(&self, url: &Url) -> OpenWindowClient {
        OpenWindowClient {
            init: Some(OpenWindowClientInit {
                clients: self.inner.clone(),
                url: url.to_string(),
            }),
            inner: None,
        }
    }
}

impl From<web_sys::Clients> for Clients {
    fn from(inner: web_sys::Clients) -> Self {
        Clients { inner }
    }
}

impl AsRef<web_sys::Clients> for Clients {
    fn as_ref(&self) -> &web_sys::Clients {
        &self.inner
    }
}

impl_common_wrapper_traits!(Clients);

struct QueryIdInit {
    clients: web_sys::Clients,
    client_id: String,
}

pub struct ClientsQueryId {
    init: Option<QueryIdInit>,
    inner: Option<JsFuture>,
}

impl Future for ClientsQueryId {
    type Output = Option<Client>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let QueryIdInit { clients, client_id } = init;

            self.inner = Some(clients.get(&client_id).into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner.poll(cx).map(|res| {
            res.ok().and_then(|val| {
                if val.is_undefined() {
                    None
                } else {
                    Some(Client::from(val.unchecked_into::<web_sys::Client>()))
                }
            })
        })
    }
}

struct QueryAllInit {
    clients: web_sys::Clients,
    query_options: web_sys::ClientQueryOptions,
}

pub struct ClientsQueryAll {
    init: Option<QueryAllInit>,
    inner: Option<JsFuture>,
}

impl Future for ClientsQueryAll {
    type Output = MatchingClients;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let QueryAllInit {
                clients,
                query_options,
            } = init;

            self.inner = Some(clients.match_all_with_options(&query_options).into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner.poll(cx).map(|res| {
            let inner = res
                .map(|v| v.unchecked_into::<js_sys::Array>())
                .unwrap_or_else(|_| js_sys::Array::new());

            MatchingClients::new(inner)
        })
    }
}

unchecked_cast_array!(Client, WebSysClient, MatchingClients);

dom_exception_wrapper!(OpenWindowClientError);

struct OpenWindowClientInit {
    clients: web_sys::Clients,
    url: String,
}

pub struct OpenWindowClient {
    init: Option<OpenWindowClientInit>,
    inner: Option<JsFuture>,
}

impl Future for OpenWindowClient {
    type Output = Result<Option<WindowClient>, OpenWindowClientError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let OpenWindowClientInit { clients, url } = init;

            self.inner = Some(clients.open_window(&url).into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|v| {
                if v.is_null() {
                    None
                } else {
                    Some(WindowClient::from(
                        v.unchecked_into::<web_sys::WindowClient>(),
                    ))
                }
            })
            .map_err(|err| OpenWindowClientError::new(err.unchecked_into()))
    }
}

pub struct ClaimClients {
    clients: Option<web_sys::Clients>,
    inner: Option<JsFuture>,
}

impl Future for ClaimClients {
    type Output = Result<(), ClaimClientsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(clients) = self.clients.take() {
            self.inner = Some(clients.claim().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| ClaimClientsError::new(err.unchecked_into()))
    }
}

dom_exception_wrapper!(ClaimClientsError);

mod service_worker_client_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_client(&self) -> &web_sys::Client;
    }
}

pub trait ServiceWorkerClient: service_worker_client_seal::Seal {
    fn id(&self) -> String {
        self.as_web_sys_client().id()
    }

    fn client_type(&self) -> ClientType {
        ClientType::from_web_sys(self.as_web_sys_client().type_())
    }

    fn frame_type(&self) -> ClientFrameType {
        ClientFrameType::from_web_sys(self.as_web_sys_client().frame_type())
    }

    fn url(&self) -> Url {
        Url::parse(self.as_web_sys_client().url().as_ref()).unwrap_throw()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientType {
    Window,
    DedicatedWorker,
    SharedWorker,
}

impl ClientType {
    fn from_web_sys(client_type: web_sys::ClientType) -> Self {
        match client_type {
            web_sys::ClientType::Window => ClientType::Window,
            web_sys::ClientType::Worker => ClientType::DedicatedWorker,
            web_sys::ClientType::Sharedworker => ClientType::SharedWorker,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientFrameType {
    TopLevel,
    Auxiliary,
    Nested,
    None,
}

impl ClientFrameType {
    fn from_web_sys(frame_type: web_sys::FrameType) -> Self {
        match frame_type {
            web_sys::FrameType::TopLevel => ClientFrameType::TopLevel,
            web_sys::FrameType::Auxiliary => ClientFrameType::Auxiliary,
            web_sys::FrameType::Nested => ClientFrameType::Nested,
            web_sys::FrameType::None => ClientFrameType::None,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Client {
    inner: web_sys::Client,
}

impl service_worker_client_seal::Seal for Client {
    fn as_web_sys_client(&self) -> &web_sys::Client {
        &self.inner
    }
}

impl ServiceWorkerClient for Client {}

impl message_sender_seal::Seal for Client {}

impl MessageSender for Client {}

impl From<web_sys::Client> for Client {
    fn from(inner: web_sys::Client) -> Self {
        Client { inner }
    }
}

impl From<Client> for web_sys::Client {
    fn from(client: Client) -> Self {
        client.inner
    }
}

impl AsRef<web_sys::Client> for Client {
    fn as_ref(&self) -> &web_sys::Client {
        &self.inner
    }
}

impl_common_wrapper_traits!(Client);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowClientVisibilityState {
    Visible,
    Hidden,
}

impl WindowClientVisibilityState {
    fn from_web_sys(state: web_sys::VisibilityState) -> Self {
        match state {
            web_sys::VisibilityState::Visible => WindowClientVisibilityState::Visible,
            web_sys::VisibilityState::Hidden => WindowClientVisibilityState::Hidden,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct WindowClient {
    inner: web_sys::WindowClient,
}

impl WindowClient {
    delegate! {
        to self.inner {
            pub fn focused(&self) -> bool;
        }
    }

    pub fn visibility_state(&self) -> WindowClientVisibilityState {
        WindowClientVisibilityState::from_web_sys(self.inner.visibility_state())
    }

    pub fn focus(&self) -> FocusWindowClient {
        FocusWindowClient {
            window_client: Some(self.inner.clone()),
            inner: None,
        }
    }

    pub fn navigate(&self, url: &Url) -> NavigateWindowClient {
        NavigateWindowClient {
            init: Some(NavigateWindowClientInit {
                window_client: self.inner.clone(),
                url: url.to_string(),
            }),
            inner: None,
        }
    }
}

impl service_worker_client_seal::Seal for WindowClient {
    fn as_web_sys_client(&self) -> &web_sys::Client {
        self.inner.as_ref()
    }
}

impl ServiceWorkerClient for WindowClient {}

impl message_sender_seal::Seal for WindowClient {}

impl MessageSender for WindowClient {}

impl From<web_sys::WindowClient> for WindowClient {
    fn from(inner: web_sys::WindowClient) -> Self {
        WindowClient { inner }
    }
}

impl AsRef<web_sys::WindowClient> for WindowClient {
    fn as_ref(&self) -> &web_sys::WindowClient {
        &self.inner
    }
}

impl TryFrom<Client> for WindowClient {
    type Error = InvalidCast<Client, WindowClient>;

    fn try_from(value: Client) -> Result<Self, Self::Error> {
        let value: web_sys::Client = value.into();

        value
            .dyn_into::<web_sys::WindowClient>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast::new(e.into()))
    }
}

impl_common_wrapper_traits!(WindowClient);

pub struct FocusWindowClient {
    window_client: Option<web_sys::WindowClient>,
    inner: Option<JsFuture>,
}

impl Future for FocusWindowClient {
    type Output = Result<WindowClient, FocusWindowClientError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(window_client) = self.window_client.take() {
            // No indication in the spec that focus is fallible; it does return a fallible promise.
            self.inner = Some(window_client.focus().unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|c| WindowClient::from(c.unchecked_into::<web_sys::WindowClient>()))
            .map_err(|err| FocusWindowClientError::new(err.unchecked_into()))
    }
}

type_error_wrapper!(FocusWindowClientError);

struct NavigateWindowClientInit {
    window_client: web_sys::WindowClient,
    url: String,
}

pub struct NavigateWindowClient {
    init: Option<NavigateWindowClientInit>,
    inner: Option<JsFuture>,
}

impl Future for NavigateWindowClient {
    type Output = Result<WindowClient, NavigateWindowClientError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let NavigateWindowClientInit { window_client, url } = init;

            // No indication in the spec that navigate is fallible; it does return a fallible
            // promise.
            self.inner = Some(window_client.navigate(&url).unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|c| WindowClient::from(c.unchecked_into::<web_sys::WindowClient>()))
            .map_err(|err| NavigateWindowClientError::new(err.unchecked_into()))
    }
}

type_error_wrapper!(NavigateWindowClientError);
