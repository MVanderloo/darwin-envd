use regex::Regex;
use std::env;

use crate::shell::{Shell, execute_command};

pub fn expand_variable(var_name: &str, fallback: Option<&str>, debug_level: u8) -> String {
    if let Ok(value) = env::var(var_name) {
        if !value.is_empty() {
            if debug_level >= 3 {
                eprintln!("Environment variable {} = {}", var_name, value);
            }
            return value;
        }
    }

    if let Some(fallback_value) = fallback {
        if debug_level >= 3 {
            eprintln!("Using fallback for {}: {}", var_name, fallback_value);
        }
        fallback_value.to_string()
    } else {
        if debug_level >= 3 {
            eprintln!("Environment variable {} is unset or empty", var_name);
        }
        String::new()
    }
}

pub fn process_value(value: &str, shell: &Shell, debug_level: u8) -> String {
    let mut result = value.to_string();

    let var_fallback_re = Regex::new(r"\$\{([A-Za-z0-9_]+):-([^}]*)\}").unwrap();
    while let Some(caps) = var_fallback_re.captures(&result) {
        let full_match = caps.get(0).unwrap().as_str();
        let var_name = caps.get(1).unwrap().as_str();
        let fallback = caps.get(2).unwrap().as_str();

        let expanded = expand_variable(var_name, Some(fallback), debug_level);
        result = result.replace(full_match, &expanded);
    }

    let var_re = Regex::new(r"\$\{([A-Za-z0-9_]+)\}").unwrap();
    while let Some(caps) = var_re.captures(&result) {
        let full_match = caps.get(0).unwrap().as_str();
        let var_name = caps.get(1).unwrap().as_str();

        let expanded = expand_variable(var_name, None, debug_level);
        result = result.replace(full_match, &expanded);
    }

    let simple_var_re = Regex::new(r"\$([A-Za-z0-9_]+)").unwrap();
    while let Some(caps) = simple_var_re.captures(&result) {
        let full_match = caps.get(0).unwrap().as_str();
        let var_name = caps.get(1).unwrap().as_str();

        let expanded = expand_variable(var_name, None, debug_level);
        result = result.replace(full_match, &expanded);
    }

    let dollar_paren_re = Regex::new(r"\$\((.*?)\)").unwrap();
    while let Some(caps) = dollar_paren_re.captures(&result) {
        let full_match = caps.get(0).unwrap().as_str();
        let cmd = caps.get(1).unwrap().as_str();
        let cmd_output = execute_command(cmd, shell, debug_level);
        result = result.replace(full_match, &cmd_output);
    }

    let backtick_re = Regex::new(r"`(.*?)`").unwrap();
    while let Some(caps) = backtick_re.captures(&result) {
        let full_match = caps.get(0).unwrap().as_str();
        let cmd = caps.get(1).unwrap().as_str();
        let cmd_output = execute_command(cmd, shell, debug_level);
        result = result.replace(full_match, &cmd_output);
    }

    result
}
