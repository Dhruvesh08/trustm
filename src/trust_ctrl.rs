use std::{fs, process::Command};

use anyhow::{bail, Result};
use tracing::{error as trace_error, info, trace};

use crate::{TrustZoneCtrlError, TrustZoneCtrlErrorCodes};

#[derive(Debug, Default)]
pub struct TrustZoneCtrl {
    pub path: String,
}

impl TrustZoneCtrl {
    pub fn new() -> Self {
        TrustZoneCtrl {
            path: String::from("/MECHA_TEST/optiga_trust_m"),
        }
    }

    fn read_value_from_file(&self, path: &str) -> Result<String, TrustZoneCtrlError> {
        fs::read_to_string(path).map_err(|e| {
            TrustZoneCtrlError::new(TrustZoneCtrlErrorCodes::FileReadError, e.to_string())
        })
    }

    //write a function to write a file to the trustzone ic and return ok or error

    //read_trustzone_cert we need to read the cert from the trustzone ic and return it that will be type of
    pub fn read_trustzone_cert(&self, output_file: &str, region: &str) -> Result<String> {
        trace!(task = "read_trustzone_cert", "init");

        let command_output = Command::new("/MECHA_TEST/optiga_trust_m/trustm_cert")
            .arg("-r")
            .arg(region)
            .arg("-o")
            .arg(output_file)
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("Error executing trustm_cert: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output = String::from_utf8(output.stdout).unwrap();
                info!(task = "read_trustzone_cert", "output: {}", output);

                //read the file and return it
                let output = match fs::read_to_string(output_file) {
                    Ok(x) => {
                        info!(task = "read_trustzone_cert", "output: {}", x);
                        x
                    }
                    Err(e) => {
                        trace_error!(task = "read_trustzone_cert", "unable to read cert: {}", e);
                        bail!(TrustZoneCtrlError::new(
                            TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                            format!("unable to read cert: {}", e),
                        ))
                    }
                };

                Ok(output)
            }
            Err(e) => {
                trace_error!(task = "read_trustzone_cert", "unable to read cert: {}", e);
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read cert: {}", e),
                ))
            }
        }
        

    }
    //write_trustzone_cert we need to write the cert to the trustzone ic and return ok or error using match and anyhow error
    pub fn write_trustzone_cert(&self, cert: &str) -> Result<()> {
        trace!(task = "write_trustzone_cert", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        match fs::write("/dev/trustzone_cert", cert) {
            Ok(_) => {
                info!(task = "write_trustzone_cert", "cert: {}", cert);
                Ok(())
            }
            Err(e) => {
                trace_error!(task = "write_trustzone_cert", "unable to write cert: {}", e);
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToWriteTrustZoneCert,
                    format!("unable to write cert: {}", e),
                ))
            }
        }
    }

    //remove_trustzone_cert form the trustzone ic and return ok or error using match and anyhow error
    pub fn remove_trustzone_cert(&self) -> Result<()> {
        trace!(task = "remove_trustzone_cert", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        match fs::remove_file("/dev/trustzone_cert") {
            Ok(_) => {
                info!(task = "remove_trustzone_cert", "cert removed");
                Ok(())
            }
            Err(e) => {
                trace_error!(
                    task = "remove_trustzone_cert",
                    "unable to remove cert: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToRemoveTrustZoneCert,
                    format!("unable to remove cert: {}", e),
                ))
            }
        }
    }

    //generate_trustzone_key we need to generate a key uisng trust ic and return it that will be type of string or error using match and anyhow error
    pub fn generate_trustzone_key(&self) -> Result<String> {
        trace!(task = "generate_trustzone_key", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let key = match self.read_value_from_file("/dev/trustzone_key") {
            Ok(x) => {
                info!(task = "generate_trustzone_key", "key: {}", x);
                x
            }
            Err(e) => {
                trace_error!(task = "generate_trustzone_key", "unable to read key: {}", e);
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read key: {}", e),
                ))
            }
        };
        Ok(key)
    }

    //sign_trustzone_data uisng trust ic and return it that will be type of string or error using match and anyhow error
    pub fn sign_trustzone_data(&self, _data: &str) -> Result<String> {
        trace!(task = "sign_trustzone_data", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let signed_data = match self.read_value_from_file("/dev/trustzone_sign") {
            Ok(x) => {
                info!(task = "sign_trustzone_data", "signed_data: {}", x);
                x
            }
            Err(e) => {
                trace_error!(
                    task = "sign_trustzone_data",
                    "unable to read signed_data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read signed_data: {}", e),
                ))
            }
        };
        Ok(signed_data)
    }

    //verify_trustzone_data that accept path of file using trust ic and return it that will be type of string or error using match and anyhow error
    pub fn verify_trustzone_data(&self, _data: &str) -> Result<String> {
        trace!(task = "verify_trustzone_data", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let verified_data = match self.read_value_from_file("/dev/trustzone_verify") {
            Ok(x) => {
                info!(task = "verify_trustzone_data", "verified_data: {}", x);
                x
            }
            Err(e) => {
                trace_error!(
                    task = "verify_trustzone_data",
                    "unable to read verified_data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read verified_data: {}", e),
                ))
            }
        };
        Ok(verified_data)
    }
    //encrypt_trustzone_data that accept path of file using trust ic and return it that will be type of string or error using match and anyhow error
    pub fn encrypt_trustzone_data(&self, _data: &str) -> Result<String> {
        trace!(task = "encrypt_trustzone_data", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let encrypted_data = match self.read_value_from_file("/dev/trustzone_encrypt") {
            Ok(x) => {
                info!(task = "encrypt_trustzone_data", "encrypted_data: {}", x);
                x
            }
            Err(e) => {
                trace_error!(
                    task = "encrypt_trustzone_data",
                    "unable to read encrypted_data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read encrypted_data: {}", e),
                ))
            }
        };
        Ok(encrypted_data)
    }

    //decrypt_trustzone_data that accept path of file using trust ic and return it that will be type of string or error using match and anyhow error
    pub fn decrypt_trustzone_data(&self, _data: &str) -> Result<String> {
        trace!(task = "decrypt_trustzone_data", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let decrypted_data = match self.read_value_from_file("/dev/trustzone_decrypt") {
            Ok(x) => {
                info!(task = "decrypt_trustzone_data", "decrypted_data: {}", x);
                x
            }
            Err(e) => {
                trace_error!(
                    task = "decrypt_trustzone_data",
                    "unable to read decrypted_data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read decrypted_data: {}", e),
                ))
            }
        };
        Ok(decrypted_data)
    }

    //encrypt_trustzone_data that accept path of file using trust ic and return it that will be type of string or error using match and anyhow error
    pub fn encrypt_trustzone_data_with_key(&self, _data: &str, _key: &str) -> Result<String> {
        trace!(task = "encrypt_trustzone_data_with_key", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let encrypted_data = match self.read_value_from_file("/dev/trustzone_encrypt") {
            Ok(x) => {
                info!(
                    task = "encrypt_trustzone_data_with_key",
                    "encrypted_data: {}", x
                );
                x
            }
            Err(e) => {
                trace_error!(
                    task = "encrypt_trustzone_data_with_key",
                    "unable to read encrypted_data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read encrypted_data: {}", e),
                ))
            }
        };
        Ok(encrypted_data)
    }

    //generate_trustzone_hmac we need to generate a hmac uisng trust ic and return it that will be type of string or error using match and anyhow error
    pub fn generate_trustzone_hmac(&self) -> Result<String> {
        trace!(task = "generate_trustzone_hmac", "init");
        let genrated_hmac = match self.read_value_from_file("/dev/trustzone_encrypt") {
            Ok(x) => {
                info!(
                    task = "encrypt_trustzone_data_with_key",
                    "genrated_hmac: {}", x
                );
                x
            }
            Err(e) => {
                trace_error!(
                    task = "encrypt_trustzone_data_with_key",
                    "unable to read genrated_hmac: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read genrated_hmac: {}", e),
                ))
            }
        };
        Ok(genrated_hmac)
    }
}
