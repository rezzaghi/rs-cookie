mod enums;
use enums::*;

pub fn set(name: &str, value: &str, domain: enums::Domain, path: Path, expires: Expires, secure: Secure, http_only: HttpOnly) -> String {
    let http_only_to_str = match http_only {
        HttpOnly::HttpOnly => "HttpOnly",
        HttpOnly::NonHttpOnly => ""
    };
    
    let secure_to_str = match secure {
        Secure::Secure => "Secure",
        Secure::NonSecure => "",
    };

    let domain_to_str = match domain {
        Domain::SetDomain(domain) => domain,
        Domain::Default => "",
    };
    let path_to_str = match path {
        Path::SetPath(path) => path,
        Path::Default => "",
    };
    let expires_to_str = match expires {
        Expires::SetExpireDate(date) => date,
        Expires::Default => ""
    };

    return format!(
        "Set-Cookie: {name}={value}; domain={domain_to_str}; path={path_to_str}; expires={expires_to_str}; {http_only_to_str}; {secure_to_str}"
    );
}
