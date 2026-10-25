#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Serialize)]
struct ProviderGooglesiteverificationData_ {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ProviderGooglesiteverification_ {
    data: std::cell::RefCell<ProviderGooglesiteverificationData_>,
}

pub struct ProviderGooglesiteverification(std::rc::Rc<ProviderGooglesiteverification_>);

impl ProviderGooglesiteverification {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "googlesiteverification", alias)
        } else {
            "googlesiteverification".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc =
        "Set the field `credentials`.\nEither the path to or the contents of a [service account key file](https://cloud.google.com/iam/docs/creating-managing-service-account-keys) in JSON format. If not provided, the [application default credentials](https://cloud.google.com/sdk/gcloud/reference/auth/application-default) will be used."]
    pub fn set_credentials(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().credentials = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderGooglesiteverification_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "googlesiteverification".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "hectorj/googlesiteverification",
            "version": "0.4.5",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderGooglesiteverification {}

impl BuildProviderGooglesiteverification {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderGooglesiteverification {
        let out =
            ProviderGooglesiteverification(
                std::rc::Rc::new(
                    ProviderGooglesiteverification_ {
                        data: std::cell::RefCell::new(ProviderGooglesiteverificationData_ {
                            alias: std::option::Option::None,
                            credentials: core::default::Default::default(),
                        }),
                    },
                ),
            );
        stack.add_provider(out.0.clone());
        out
    }
}
