pub enum CheckResult {
    Valid {
        tool: String,
        ver: String,
    },
    Outdated {
        tool: String,
        ver: String,
        ver_required: String,
    },
    Missing {
        tool: String,
        ver_required: String,
    },
}