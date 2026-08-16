use crate::flags::{HiArgs, SearchMode};

pub(crate) fn write(_args: &HiArgs) -> anyhow::Result<()> {
    anyhow::bail!("{}", crate::i18n::t("err-indexing-disabled"))
}

pub(crate) fn read(_args: &HiArgs, _mode: SearchMode) -> anyhow::Result<bool> {
    anyhow::bail!("{}", crate::i18n::t("err-indexing-disabled"))
}
