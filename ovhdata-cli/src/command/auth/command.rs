use std::io::stdout;

use ovhdata_common::api;
use ovhdata_common::ovhapi::{AuthApi, OVHapiV6Client};

use crate::config::{Config, Context, Ovhapiv6Credentials, Toggle};
use crate::utils::ui::printer::{Output, Printer, HELP_LOGIN_HOW_TO, HELP_LOGIN_SUCCESS};
use crate::utils::{Error, Result};

pub struct Auth {
    config: Config,
}

impl Auth {
    pub fn new() -> Self {
        let config = Config::get();
        Self { config }
    }

    pub async fn logout(&self) -> Result<()> {
        let mut context = Context::get();
        context.logout();
        context.save()?;

        Printer::println_success(&mut stdout(), "You have successfully logged out!");
        Ok(())
    }

    pub async fn login(
        &self,
        application_key: Option<String>,
        consumer_key: Option<String>,
        application_secret: Option<String>,
        output: Output,
    ) -> Result<()> {
        let interactive =
            application_secret.is_none() || application_key.is_none() || consumer_key.is_none();

        let creds = if interactive {
            // If the config exists, test it
            let ovhapicreds = {
                let context = Context::get();
                context.get_ovhapi_credentials()
            };

            if let Some(creds) = ovhapicreds {
                // test connection
                Printer::println_success(&mut stdout(), "Current connection infos...");

                let ovhapiv6_client = OVHapiV6Client::new(
                    Config::get().ovhapiv6.endpoint_url.clone(),
                    creds.application_key.clone().unwrap(),
                    creds.application_secret.clone().unwrap(),
                    creds.consumer_key.clone().unwrap(),
                );

                let creds_result = ovhapiv6_client.current_credential().await;

                if creds_result.is_ok() {
                    Printer::print_object(&creds_result.unwrap(), &output)?;
                } else {
                    let error = creds_result.err().unwrap();
                    if let api::Error::Response(status_code, message) = error {
                        match status_code {
                            reqwest::StatusCode::FORBIDDEN | reqwest::StatusCode::UNAUTHORIZED => {
                                Printer::eprintln_fail(&format!(
                                    "You are not authenticated, status_code={}",
                                    status_code
                                ));
                                Printer::eprintln_fail(&message);
                            }

                            // Propagate other errors
                            _ => {
                                return Err(Error::DataApi(api::Error::Response(
                                    status_code,
                                    message,
                                )))
                            }
                        }
                    }
                }

                let confirm = Printer::confirm("Do you want to reset the current credentials?");

                if confirm.is_err() {
                    return Err(Error::custom("Maybe another day ;-)"));
                }
            }

            Printer::print_help(HELP_LOGIN_HOW_TO, Toggle::NoToggle);

            // Login with browser
            open::that(self.config.ovhapiv6.create_token_url.as_str())
                .map_err(|_| Error::UserInput)?;

            // Ask Application key
            let application_key = match application_key {
                Some(application_key) => application_key,
                None => Printer::ask_input("Application Key", None)?,
            };
            if application_key.is_empty() {
                return Err(Error::UserInput);
            }

            // Ask Application Secret
            let application_secret = match application_secret {
                Some(application_secret) => application_secret,
                None => Printer::ask_password("Application Secret")?,
            };
            if application_secret.is_empty() {
                return Err(Error::UserInput);
            }

            // Ask Consumer key
            let consumer_key = match consumer_key {
                Some(consumer_key) => consumer_key,
                None => Printer::ask_input("Consumer Key", None)?,
            };
            if consumer_key.is_empty() {
                return Err(Error::UserInput);
            }

            Ovhapiv6Credentials {
                application_key: Option::from(application_key),
                application_secret: Option::from(application_secret),
                consumer_key: Option::from(consumer_key),
            }
        } else {
            Ovhapiv6Credentials {
                application_key,
                application_secret,
                consumer_key,
            }
        };

        // test connection
        let ovhapiv6_client = OVHapiV6Client::new(
            Config::get().ovhapiv6.endpoint_url.clone(),
            creds.application_key.clone().unwrap(),
            creds.application_secret.clone().unwrap(),
            creds.consumer_key.clone().unwrap(),
        );

        let cred_details = ovhapiv6_client.current_credential().await?;
        Printer::print_object(&cred_details, &output)?;

        // Store api credentials
        {
            let mut context = Context::get();
            context.set_ovhapi_credentials(creds);
            context.save()?;
        }

        println!();
        Printer::println_success(&mut stdout(), "You are now logged in.");

        if interactive {
            Printer::print_help(HELP_LOGIN_SUCCESS, Toggle::NoToggle);
        }

        Ok(())
    }
}
