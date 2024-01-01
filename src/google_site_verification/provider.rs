use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderGooglesiteverificationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<PrimField<String>>,
}

struct ProviderGooglesiteverification_ {
    data: RefCell<ProviderGooglesiteverificationData>,
}

pub struct ProviderGooglesiteverification(Rc<ProviderGooglesiteverification_>);

impl ProviderGooglesiteverification {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "googlesiteverification", alias)
        } else {
            "googlesiteverification".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `credentials`.\nEither the path to or the contents of a [service account key file](https://cloud.google.com/iam/docs/creating-managing-service-account-keys) in JSON format. If not provided, the [application default credentials](https://cloud.google.com/sdk/gcloud/reference/auth/application-default) will be used."]
    pub fn set_credentials(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().credentials = Some(v.into());
        self
    }
}

impl Provider for ProviderGooglesiteverification_ {
    fn extract_type_tf_id(&self) -> String {
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
    pub fn build(self, stack: &mut Stack) -> ProviderGooglesiteverification {
        let out =
            ProviderGooglesiteverification(
                Rc::new(ProviderGooglesiteverification_ { data: RefCell::new(ProviderGooglesiteverificationData {
                    alias: None,
                    credentials: core::default::Default::default(),
                }) }),
            );
        stack.add_provider(out.0.clone());
        out
    }
}
