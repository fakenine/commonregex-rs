pub mod ip {
    pub const V4: &'static str = r"(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)";
}

pub mod phone {
    pub const FR: &'static str = r"(?:(?:\+|00)33|0)\s*[1-9](?:[\s.-]*\d{2}){4}";
}
