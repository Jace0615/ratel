#![allow(non_snake_case)]
use crate::{
    config,
    utils::phantom::{PhantomAuth, Platform},
};
use dioxus::prelude::*;
use dto::*;

pub enum UserEvent {
    Signup(String, String, String, String),
    Login,
    Logout,
}

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    pub firebase: Signal<google_wallet::FirebaseWallet>,
    pub cli: Signal<UserClient>,
    pub email: Signal<String>,
    pub nickname: Signal<String>,
    pub profile_url: Signal<String>,
    pub principal: Signal<String>,
}

impl UserService {
    pub fn init() {
        let conf = config::get();

        #[cfg(feature = "web")]
        let mut firebase = google_wallet::FirebaseWallet::new(
            conf.firebase.api_key.clone(),
            conf.firebase.auth_domain.clone(),
            conf.firebase.project_id.clone(),
            conf.firebase.storage_bucket.clone(),
            conf.firebase.messaging_sender_id.clone(),
            conf.firebase.app_id.clone(),
            conf.firebase.measurement_id.clone(),
        );

        #[cfg(not(feature = "web"))]
        let firebase = google_wallet::FirebaseWallet::default();

        #[cfg(feature = "web")]
        let _ = firebase.try_setup_from_storage();
        let cli = User::get_client(&conf.main_api_endpoint);

        use_context_provider(|| Self {
            firebase: Signal::new(firebase),
            cli: Signal::new(cli),
            email: Signal::new("".to_string()),
            nickname: Signal::new("".to_string()),
            profile_url: Signal::new("".to_string()),
            principal: Signal::new("".to_string()),
        });

        let mut user = use_context::<UserService>();
        let firebase = (user.firebase)();
        if firebase.get_login() {
            tracing::debug!("UserService::init: wallet={:?}", firebase);
            spawn(async move {
                user.get_user_info_from_server().await;
            });
        }
    }

    pub fn logout(&mut self) {
        self.firebase.write().logout();

        self.email.set("".to_string());
        self.nickname.set("".to_string());
        self.profile_url.set("".to_string());
        self.principal.set("".to_string());
    }

    pub async fn get_user_info_from_server(&mut self) {
        let cli = (self.cli)();
        rest_api::set_signer(Box::new(*self));

        let user: User = match cli.user_info().await {
            Ok(v) => v,
            Err(e) => match e {
                ServiceError::NotFound => {
                    return;
                }
                e => {
                    tracing::error!("UserService::get_user_info_from_server: error={:?}", e);
                    return;
                }
            },
        };

        self.nickname.set(user.nickname);
        self.profile_url.set(user.profile_url);
    }

    pub fn get_user_info(&self) -> Option<(String, String)> {
        let nickname = (self.nickname)();
        let profile_url = (self.profile_url)();

        if profile_url.is_empty() || nickname.is_empty() {
            return None;
        }

        Some((nickname, profile_url))
    }

    async fn request_to_firebase(
        &mut self,
    ) -> Result<(google_wallet::WalletEvent, String, String, String, String)> {
        let mut firebase = self.firebase.write();
        let (evt, principal, email, name, profile_url) =
            match firebase.request_wallet_with_google().await {
                Ok(evt) => {
                    tracing::debug!("UserService::login: cred={:?}", evt);
                    let principal = firebase.get_principal();
                    if principal.is_empty() {
                        tracing::error!("UserService::login: principal is empty");
                        return Err(ServiceError::Unauthorized);
                    }

                    let (email, name, profile_url) = match firebase.get_user_info() {
                        Some(v) => v,
                        None => {
                            tracing::error!("UserService::login: None");
                            return Err(ServiceError::Unauthorized);
                        }
                    };

                    (evt, principal, email, name, profile_url)
                }
                Err(e) => {
                    tracing::error!("UserService::login: error={:?}", e);
                    return Err(ServiceError::Unauthorized);
                }
            };

        Ok((evt, principal, email, name, profile_url))
    }

    pub async fn google_login(&mut self) -> UserEvent {
        tracing::debug!("UserService::login");
        let (evt, principal, email, name, profile_url) = self.request_to_firebase().await.unwrap();
        match evt {
            google_wallet::WalletEvent::Signup => {
                tracing::debug!(
                    "UserService::Signup: email={} name={} profile_url={}",
                    email,
                    name,
                    profile_url
                );

                return UserEvent::Signup(principal, email, name, profile_url);
            }
            google_wallet::WalletEvent::Login => {
                tracing::debug!(
                    "UserService::Signup: email={} name={} profile_url={}",
                    email,
                    name,
                    profile_url
                );
                rest_api::set_signer(Box::new(*self));
                let cli = (self.cli)();

                let user: User = match cli.check_email(email.clone()).await {
                    // Login
                    Ok(v) => v,
                    Err(e) => {
                        // Signup
                        rest_api::remove_signer();

                        match e {
                            ServiceError::NotFound => {
                                return UserEvent::Signup(principal, email, name, profile_url);
                            }
                            e => {
                                tracing::error!("UserService::login: error={:?}", e);
                                return UserEvent::Logout;
                            }
                        }
                    }
                };

                self.email.set(email);
                self.nickname.set(user.nickname);
                self.profile_url.set(user.profile_url);
                self.principal.set(principal);

                return UserEvent::Login;
            }
            google_wallet::WalletEvent::Logout => {
                tracing::debug!("UserService::login: SignOut");
            }
        }

        UserEvent::Logout
    }

    pub async fn login_or_signup(
        &self,
        principal: &str,
        email: &str,
        nickname: &str,
        profile_url: &str,
    ) -> Result<()> {
        rest_api::set_signer(Box::new(*self));

        tracing::debug!(
            "UserService::signup: principal={} email={} nickname={} profile_url={}",
            principal,
            email,
            nickname,
            profile_url
        );

        let cli = (self.cli)();

        let res: User = match cli
            .signup(
                nickname.to_string(),
                email.to_string(),
                profile_url.to_string(),
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("UserService::signup: error={:?}", e);
                rest_api::remove_signer();
                return Err(e);
            }
        };

        tracing::debug!("UserService::signup: user={:?}", res);
        Ok(())
    }

    pub async fn phantom_login(&mut self) -> UserEvent {
        tracing::debug!("UserService::phantom_wallet login");

        let cli = (self.cli)();
        let mut phantom = PhantomAuth::new();

        match phantom.detect_platform() {
            Platform::Desktop => {
                tracing::debug!("UserService::phantom_wallet: desktop");
                match phantom.connect_desktop().await {
                    Ok(account) => {
                        let public_key_str = phantom.get_public_key(account);

                        match cli.by_principal(public_key_str.clone()).await {
                            Ok(v) => {
                                tracing::debug!("UserService::phantom_wallet: login");
                                self.email.set(v.email);
                                self.nickname.set(v.nickname);
                                self.profile_url.set(v.profile_url);
                                self.principal.set(v.principal);
                                return UserEvent::Login;
                            }
                            Err(_) => {
                                tracing::debug!("UserService::phantom_wallet: signup");
                                return UserEvent::Signup(
                                    public_key_str,
                                    "".to_string(),
                                    "".to_string(),
                                    "".to_string(),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("UserService::phantom_wallet: error={:?}", e);
                    }
                };
            }
            Platform::Mobile => {
                tracing::debug!("UserService::phantom_wallet: mobile");
            }
        };
        UserEvent::Logout
    }
}

impl rest_api::Signer for UserService {
    fn signer(&self) -> String {
        (self.firebase)().get_principal()
    }

    fn sign(
        &self,
        msg: &str,
    ) -> std::result::Result<rest_api::Signature, Box<dyn std::error::Error>> {
        tracing::debug!("UserService::sign: msg={}", msg);
        let firebase = (self.firebase)();

        if !firebase.get_login() {
            tracing::debug!("UserService::sign: not login {firebase:?}");
            return Err(Box::<ServiceException>::new(
                ServiceError::Unauthorized.into(),
            ));
        }

        let sig = firebase.sign(msg);
        if sig.is_none() {
            return Err(Box::<ServiceException>::new(
                ServiceError::Unauthorized.into(),
            ));
        }
        let sig = rest_api::Signature {
            signature: sig.unwrap().as_ref().to_vec(),
            public_key: firebase.public_key().unwrap_or_default(),
            algorithm: rest_api::signature::SignatureAlgorithm::EdDSA,
        };

        Ok(sig)
    }
}
