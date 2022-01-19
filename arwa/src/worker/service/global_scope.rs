use crate::message::{
    message_event_target_seal, message_sender_seal, MessageEventTarget, MessageSender,
};
use crate::url::ContextualUrl;
use crate::InvalidCast;
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use url::Url;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::ServiceWorkerRegistration;
use crate::fetch::{CacheContext, cache_context_seal};

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

impl_worker_global_scope_traits!(ServiceWorkerGlobalScope, web_sys::ServiceWorkerGlobalScope);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MatchClientType {
    Window,
    DedicatedWorker,
    SharedWorker,
    All,
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
        let mut query_options = ClientQueryOptions::new();

        query_options.include_uncontrolled(query.include_uncontrolled);

        let web_sys_client_type = match query.client_type {
            MatchClientType::Window => web_sys::ClientType::Window,
            MatchClientType::DedicatedWorker => web_sys::ClientType::Worker,
            MatchClientType::SharedWorker => web_sys::ClientType::Sharedworker,
            MatchClientType::All => web_sys::ClientType::All,
        };

        query_options.type_(&web_sys_client_type);

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
            inner: None
        }
    }

    pub fn open_window(&self, url: ContextualUrl) -> OpenWindowClient {
        OpenWindowClient {
            init: Some(OpenWindowClientInit { clients: self.inner.clone(), url: url.to_string() }),
            inner: None
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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let QueryIdInit { clients, client_id } = init;

            self.inner = Some(clients.get(&client_id).into());
        }

        self.inner.as_mut().unwrap().poll(cx).map(|res| {
            res.ok().and_then(|val| {
                if val.is_undefined() {
                    None
                } else {
                    Some(Client::from(c.unchecked_into()))
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
    init: Option<MatchAllClientsInit>,
    inner: Option<JsFuture>,
}

impl Future for ClientsQueryAll {
    type Output = MatchingClients;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let QueryAllInit {
                clients,
                query_options,
            } = init;

            self.inner = Some(clients.match_all(&query_options).into());
        }

        self.inner.as_mut().unwrap().poll(cx).map(|res| {
            let inner = res
                .map(|v| v.unchecked_into())
                .unwrap_or_else(|_| js_sys::Array::new());

            MatchingClients { inner }
        })
    }
}

unchecked_cast_array!(Client, web_sys::Client, MatchingClients);

struct OpenWindowClientInit {
    clients: web_sys::Clients,
    url: String,
}

pub struct OpenWindowClient {
    init: Option<MatchAllClientsInit>,
    inner: Option<JsFuture>,
}

impl Future for OpenWindowClient {
    type Output = Result<Option<WindowClient>, OpenWindowClientError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let OpenWindowClientInit {
                clients,
                url,
            } = init;

            self.inner = Some(clients.open_window(&url).into());
        }

        self.inner.as_mut().unwrap().poll(cx).map_ok(|v| {
            if v.is_null() {
                None
            } else {
                Some(WindowClient::from(v.unchecked_into()))
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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(clients) = self.clients.take() {
            self.inner = Some(clients.claim().into());
        }

        self.inner.as_mut().unwrap().poll(cx).map_ok(|_| ())
            .map_err(|err| ClaimClientsError::new(err.unchecked_into()))
    }
}

#[derive(Clone)]
pub struct ClaimClientsError {
    inner: web_sys::DomException
}

impl ClaimClientsError {
    fn new(inner: web_sys::DomException) -> Self {
        ClaimClientsError {
            inner
        }
    }
}

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
        match self.as_web_sys_client().type_() {
            web_sys::ClientType::Window => ClientType::Window,
            web_sys::ClientType::Worker => ClientType::DedicatedWorker,
            web_sys::ClientType::Sharedworker => ClientType::SharedWorker,
            _ => unreachable!(),
        }
    }

    fn frame_type(&self) -> ClientFrameType {
        match self.as_web_sys_client().frame_type() {
            web_sys::FrameType::TopLevel => ClientFrameType::TopLevel,
            web_sys::FrameType::Auxiliary => ClientFrameType::Auxiliary,
            web_sys::FrameType::Nested => ClientFrameType::Nested,
            web_sys::FrameType::None => ClientFrameType::None,
        }
    }

    fn url(&self) -> Url {
        Url::parse(self.as_web_sys_client().url()).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientType {
    Window,
    DedicatedWorker,
    SharedWorker,
}

pub enum ClientFrameType {
    TopLevel,
    Auxiliary,
    Nested,
    None,
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
        match self.inner.visibility_state {
            web_sys::VisibilityState::Visible => WindowClientVisibilityState::Visible,
            web_sys::VisibilityState::Hidden => WindowClientVisibilityState::Hidden,
        }
    }

    pub fn focus(&self) -> FocusWindowClient {
        FocusWindowClient {
            window_client: Some(self.inner.clone()),
            inner: None,
        }
    }

    pub fn navigate(&self, url: ContextualUrl) -> NavigateWindowClient {
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
    type Error = InvalidCast<Client>;

    fn try_from(value: Client) -> Result<Self, Self::Error> {
        let value: web_sys::Client = value.into();

        value
            .dyn_into::<web_sys::WindowClient>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl_common_wrapper_traits!(WindowClient);

pub struct FocusWindowClient {
    window_client: Option<web_sys::WindowClient>,
    inner: Option<JsFuture>,
}

impl Future for FocusWindowClient {
    type Output = Result<WindowClient, FocusWindowClientError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(window_client) = self.window_client.take() {
            self.inner = Some(window_client.focus().into());
        }

        self.inner
            .as_mut()
            .unwrap()
            .poll(cx)
            .map_ok(|c| c.into())
            .map_err(|err| FocusWindowClientError::new(err))
    }
}

#[derive(Clone)]
pub struct FocusWindowClientError {
    inner: js_sys::TypeError,
}

impl FocusWindowClientError {
    fn new(inner: js_sys::TypeError) -> Self {
        FocusWindowClientError { inner }
    }
}

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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let NavigateWindowClientInit { window_client, url } = init;

            self.inner = Some(window_client.navigate(&url).into());
        }

        self.inner
            .as_mut()
            .unwrap()
            .poll(cx)
            .map_ok(|c| c.into())
            .map_err(|err| NavigateWindowClientError::new(err))
    }
}

#[derive(Clone)]
pub struct NavigateWindowClientError {
    inner: js_sys::TypeError,
}

impl NavigateWindowClientError {
    fn new(inner: js_sys::TypeError) -> Self {
        NavigateWindowClientError { inner }
    }
}
