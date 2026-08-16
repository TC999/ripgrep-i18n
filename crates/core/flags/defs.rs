/*!
Defines all of the flags available in ripgrep.

Each flag corresponds to a unit struct with a corresponding implementation
of `Flag`. Note that each implementation of `Flag` might actually have many
possible manifestations of the same "flag." That is, each implementation of
`Flag` can have the following flags available to an end user of ripgrep:

* The long flag name.
* An optional short flag name.
* An optional negated long flag name.
* An arbitrarily long list of aliases.

The idea is that even though there are multiple flags that a user can type,
one implementation of `Flag` corresponds to a single _logical_ flag inside of
ripgrep. For example, `-E`, `--encoding` and `--no-encoding` all manipulate the
same encoding state in ripgrep.
*/

use std::{path::PathBuf, sync::LazyLock};

use {anyhow::Context as AnyhowContext, bstr::ByteVec};

use crate::flags::{
    Category, Flag, FlagValue,
    lowargs::{
        BinaryMode, BoundaryMode, BufferMode, CaseMode, ColorChoice,
        ContextMode, EncodingMode, EngineChoice, GenerateMode, IndexMode,
        LoggingMode, LowArgs, MmapMode, Mode, PatternSource, SearchMode,
        SortMode, SortModeKind, SpecialMode, TypeChange,
    },
};

#[cfg(test)]
use crate::flags::parse::parse_low_raw;

use super::CompletionType;

/// A list of all flags in ripgrep via implementations of `Flag`.
///
/// The order of these flags matter. It determines the order of the flags in
/// the generated documentation (`-h`, `--help` and the man page) within each
/// category. (This is why the deprecated flags are last.)
pub(super) const FLAGS: &[&dyn Flag] = &[
    // -e/--regexp and -f/--file should come before anything else in the
    // same category.
    &Regexp,
    &File,
    &AfterContext,
    &BeforeContext,
    &Binary,
    &BlockBuffered,
    &ByteOffset,
    &CaseSensitive,
    &Color,
    &Colors,
    &Column,
    &Context,
    &ContextSeparator,
    &Count,
    &CountMatches,
    &Crlf,
    &Debug,
    &DfaSizeLimit,
    &Encoding,
    &Engine,
    &FieldContextSeparator,
    &FieldMatchSeparator,
    &Files,
    &FilesWithMatches,
    &FilesWithoutMatch,
    &FixedStrings,
    &Follow,
    &Generate,
    &Glob,
    &GlobCaseInsensitive,
    &Heading,
    &Help,
    &Hidden,
    &HostnameBin,
    &HyperlinkFormat,
    &IGlob,
    &IgnoreCase,
    &IgnoreFile,
    &IgnoreFileCaseInsensitive,
    &IncludeZero,
    &Index,
    &IndexCrud,
    &IndexForce,
    &IndexPath,
    &InvertMatch,
    &JSON,
    &LineBuffered,
    &LineNumber,
    &LineNumberNo,
    &LineRegexp,
    &MaxColumns,
    &MaxColumnsPreview,
    &MaxCount,
    &MaxDepth,
    &MaxFilesize,
    &Mmap,
    &Multiline,
    &MultilineDotall,
    &NoConfig,
    &NoIgnore,
    &NoIgnoreDot,
    &NoIgnoreExclude,
    &NoIgnoreFiles,
    &NoIgnoreGlobal,
    &NoIgnoreMessages,
    &NoIgnoreParent,
    &NoIgnoreVcs,
    &NoMessages,
    &NoRequireGit,
    &NoUnicode,
    &Null,
    &NullData,
    &OneFileSystem,
    &OnlyMatching,
    &PathSeparator,
    &Passthru,
    &PCRE2,
    &PCRE2Version,
    &Pre,
    &PreGlob,
    &Pretty,
    &Quiet,
    &RegexSizeLimit,
    &Replace,
    &SearchZip,
    &SmartCase,
    &Sort,
    &Sortr,
    &Stats,
    &StopOnNonmatch,
    &Text,
    &Threads,
    &Trace,
    &Trim,
    &Type,
    &TypeNot,
    &TypeAdd,
    &TypeClear,
    &TypeList,
    &Unrestricted,
    &Version,
    &Vimgrep,
    &WithFilename,
    &WithFilenameNo,
    &WordRegexp,
    // DEPRECATED (make them show up last in their respective categories)
    &AutoHybridRegex,
    &NoPcre2Unicode,
    &SortFiles,
];

impl LowArgs {
    /// Returns a flag that does not support indexing, if it's enabled.
    ///
    /// If there aren't any flags enabled that don't support indexing, then
    /// `None` is returned.
    ///
    /// The idea of this routine is to start out very paranoid about what is
    /// allowed to be used when indexing is enabled. Ideally, most or all of
    /// these would eventually become supported.
    pub(super) fn indexing_unsupported_flag(
        &self,
    ) -> Option<&'static dyn Flag> {
        if matches!(self.mode, Mode::Search(SearchMode::FilesWithoutMatch)) {
            return Some(&FilesWithoutMatch);
        }
        if matches!(self.binary, BinaryMode::AsText) {
            return Some(&Binary);
        }
        if !matches!(self.encoding, EncodingMode::Auto) {
            return Some(&Encoding);
        }
        if matches!(self.engine, EngineChoice::PCRE2) {
            return Some(&Engine);
        }
        if self.follow {
            return Some(&Follow);
        }
        if !self.globs.is_empty() {
            return Some(&Glob);
        }
        if self.hidden {
            return Some(&Hidden);
        }
        if !self.iglobs.is_empty() {
            return Some(&Glob);
        }
        if !self.ignore_file.is_empty() {
            return Some(&IgnoreFile);
        }
        if self.no_ignore_dot {
            return Some(&NoIgnoreDot);
        }
        if self.no_ignore_exclude {
            return Some(&NoIgnoreExclude);
        }
        if self.no_ignore_files {
            return Some(&NoIgnoreFiles);
        }
        if self.no_ignore_global {
            return Some(&NoIgnoreGlobal);
        }
        if self.no_ignore_parent {
            return Some(&NoIgnoreParent);
        }
        if self.no_ignore_vcs {
            return Some(&NoIgnoreVcs);
        }
        if self.no_require_git {
            return Some(&NoRequireGit);
        }
        if self.one_file_system {
            return Some(&OneFileSystem);
        }
        if self.pre.is_some() {
            return Some(&Pre);
        }
        if self.search_zip {
            return Some(&SearchZip);
        }
        None
    }
}

/// -A/--after-context
#[derive(Debug)]
struct AfterContext;

impl Flag for AfterContext {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'A')
    }
    fn name_long(&self) -> &'static str {
        "after-context"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-after-context-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-after-context-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.context.set_after(convert::usize(&v.unwrap_value())?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_after_context() {
    let mkctx = |lines| {
        let mut mode = ContextMode::default();
        mode.set_after(lines);
        mode
    };

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(ContextMode::default(), args.context);

    let args = parse_low_raw(["--after-context", "5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["--after-context=5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-A", "5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-A5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-A5", "-A10"]).unwrap();
    assert_eq!(mkctx(10), args.context);

    let args = parse_low_raw(["-A5", "-A0"]).unwrap();
    assert_eq!(mkctx(0), args.context);

    let args = parse_low_raw(["-A5", "--passthru"]).unwrap();
    assert_eq!(ContextMode::Passthru, args.context);

    let args = parse_low_raw(["--passthru", "-A5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let n = usize::MAX.to_string();
    let args = parse_low_raw(["--after-context", n.as_str()]).unwrap();
    assert_eq!(mkctx(usize::MAX), args.context);

    #[cfg(target_pointer_width = "64")]
    {
        let n = (u128::from(u64::MAX) + 1).to_string();
        let result = parse_low_raw(["--after-context", n.as_str()]);
        assert!(result.is_err(), "{result:?}");
    }
}

/// --auto-hybrid-regex
#[derive(Debug)]
struct AutoHybridRegex;

impl Flag for AutoHybridRegex {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "auto-hybrid-regex"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-auto-hybrid-regex")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-auto-hybrid-regex-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-auto-hybrid-regex-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let mode = if v.unwrap_switch() {
            EngineChoice::Auto
        } else {
            EngineChoice::Default
        };
        args.engine = mode;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_auto_hybrid_regex() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(EngineChoice::Default, args.engine);

    let args = parse_low_raw(["--auto-hybrid-regex"]).unwrap();
    assert_eq!(EngineChoice::Auto, args.engine);

    let args =
        parse_low_raw(["--auto-hybrid-regex", "--no-auto-hybrid-regex"])
            .unwrap();
    assert_eq!(EngineChoice::Default, args.engine);

    let args =
        parse_low_raw(["--no-auto-hybrid-regex", "--auto-hybrid-regex"])
            .unwrap();
    assert_eq!(EngineChoice::Auto, args.engine);

    let args = parse_low_raw(["--auto-hybrid-regex", "-P"]).unwrap();
    assert_eq!(EngineChoice::PCRE2, args.engine);

    let args = parse_low_raw(["-P", "--auto-hybrid-regex"]).unwrap();
    assert_eq!(EngineChoice::Auto, args.engine);

    let args =
        parse_low_raw(["--engine=auto", "--auto-hybrid-regex"]).unwrap();
    assert_eq!(EngineChoice::Auto, args.engine);

    let args =
        parse_low_raw(["--engine=default", "--auto-hybrid-regex"]).unwrap();
    assert_eq!(EngineChoice::Auto, args.engine);

    let args =
        parse_low_raw(["--auto-hybrid-regex", "--engine=default"]).unwrap();
    assert_eq!(EngineChoice::Default, args.engine);
}

/// -B/--before-context
#[derive(Debug)]
struct BeforeContext;

impl Flag for BeforeContext {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'B')
    }
    fn name_long(&self) -> &'static str {
        "before-context"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-before-context-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-before-context-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.context.set_before(convert::usize(&v.unwrap_value())?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_before_context() {
    let mkctx = |lines| {
        let mut mode = ContextMode::default();
        mode.set_before(lines);
        mode
    };

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(ContextMode::default(), args.context);

    let args = parse_low_raw(["--before-context", "5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["--before-context=5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-B", "5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-B5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-B5", "-B10"]).unwrap();
    assert_eq!(mkctx(10), args.context);

    let args = parse_low_raw(["-B5", "-B0"]).unwrap();
    assert_eq!(mkctx(0), args.context);

    let args = parse_low_raw(["-B5", "--passthru"]).unwrap();
    assert_eq!(ContextMode::Passthru, args.context);

    let args = parse_low_raw(["--passthru", "-B5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let n = usize::MAX.to_string();
    let args = parse_low_raw(["--before-context", n.as_str()]).unwrap();
    assert_eq!(mkctx(usize::MAX), args.context);

    #[cfg(target_pointer_width = "64")]
    {
        let n = (u128::from(u64::MAX) + 1).to_string();
        let result = parse_low_raw(["--before-context", n.as_str()]);
        assert!(result.is_err(), "{result:?}");
    }
}

/// --binary
#[derive(Debug)]
struct Binary;

impl Flag for Binary {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "binary"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-binary")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-binary-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-binary-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.binary = if v.unwrap_switch() {
            BinaryMode::SearchAndSuppress
        } else {
            BinaryMode::Auto
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_binary() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(BinaryMode::Auto, args.binary);

    let args = parse_low_raw(["--binary"]).unwrap();
    assert_eq!(BinaryMode::SearchAndSuppress, args.binary);

    let args = parse_low_raw(["--binary", "--no-binary"]).unwrap();
    assert_eq!(BinaryMode::Auto, args.binary);

    let args = parse_low_raw(["--no-binary", "--binary"]).unwrap();
    assert_eq!(BinaryMode::SearchAndSuppress, args.binary);

    let args = parse_low_raw(["--binary", "-a"]).unwrap();
    assert_eq!(BinaryMode::AsText, args.binary);

    let args = parse_low_raw(["-a", "--binary"]).unwrap();
    assert_eq!(BinaryMode::SearchAndSuppress, args.binary);

    let args = parse_low_raw(["-a", "--no-binary"]).unwrap();
    assert_eq!(BinaryMode::Auto, args.binary);
}

/// --block-buffered
#[derive(Debug)]
struct BlockBuffered;

impl Flag for BlockBuffered {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "block-buffered"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-block-buffered")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-block-buffered-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-block-buffered-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.buffer = if v.unwrap_switch() {
            BufferMode::Block
        } else {
            BufferMode::Auto
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_block_buffered() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(BufferMode::Auto, args.buffer);

    let args = parse_low_raw(["--block-buffered"]).unwrap();
    assert_eq!(BufferMode::Block, args.buffer);

    let args =
        parse_low_raw(["--block-buffered", "--no-block-buffered"]).unwrap();
    assert_eq!(BufferMode::Auto, args.buffer);

    let args = parse_low_raw(["--block-buffered", "--line-buffered"]).unwrap();
    assert_eq!(BufferMode::Line, args.buffer);
}

/// --byte-offset
#[derive(Debug)]
struct ByteOffset;

impl Flag for ByteOffset {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'b')
    }
    fn name_long(&self) -> &'static str {
        "byte-offset"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-byte-offset")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-byte-offset-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-byte-offset-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.byte_offset = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_byte_offset() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.byte_offset);

    let args = parse_low_raw(["--byte-offset"]).unwrap();
    assert_eq!(true, args.byte_offset);

    let args = parse_low_raw(["-b"]).unwrap();
    assert_eq!(true, args.byte_offset);

    let args = parse_low_raw(["--byte-offset", "--no-byte-offset"]).unwrap();
    assert_eq!(false, args.byte_offset);

    let args = parse_low_raw(["--no-byte-offset", "-b"]).unwrap();
    assert_eq!(true, args.byte_offset);
}

/// -s/--case-sensitive
#[derive(Debug)]
struct CaseSensitive;

impl Flag for CaseSensitive {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b's')
    }
    fn name_long(&self) -> &'static str {
        "case-sensitive"
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-case-sensitive-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-case-sensitive-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "flag has no negation");
        args.case = CaseMode::Sensitive;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_case_sensitive() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(CaseMode::Sensitive, args.case);

    let args = parse_low_raw(["--case-sensitive"]).unwrap();
    assert_eq!(CaseMode::Sensitive, args.case);

    let args = parse_low_raw(["-s"]).unwrap();
    assert_eq!(CaseMode::Sensitive, args.case);
}

/// --color
#[derive(Debug)]
struct Color;

impl Flag for Color {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "color"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("WHEN")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-color-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-color-long"
    }
    fn doc_choices(&self) -> &'static [&'static str] {
        &["never", "auto", "always", "ansi"]
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.color = match convert::str(&v.unwrap_value())? {
            "never" => ColorChoice::Never,
            "auto" => ColorChoice::Auto,
            "always" => ColorChoice::Always,
            "ansi" => ColorChoice::Ansi,
            unk => anyhow::bail!(
                "{}",
                crate::i18n::t_args(
                    "err-choice-unrecognized",
                    &[("choice", unk)]
                )
            ),
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_color() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(ColorChoice::Auto, args.color);

    let args = parse_low_raw(["--color", "never"]).unwrap();
    assert_eq!(ColorChoice::Never, args.color);

    let args = parse_low_raw(["--color", "auto"]).unwrap();
    assert_eq!(ColorChoice::Auto, args.color);

    let args = parse_low_raw(["--color", "always"]).unwrap();
    assert_eq!(ColorChoice::Always, args.color);

    let args = parse_low_raw(["--color", "ansi"]).unwrap();
    assert_eq!(ColorChoice::Ansi, args.color);

    let args = parse_low_raw(["--color=never"]).unwrap();
    assert_eq!(ColorChoice::Never, args.color);

    let args =
        parse_low_raw(["--color", "always", "--color", "never"]).unwrap();
    assert_eq!(ColorChoice::Never, args.color);

    let args =
        parse_low_raw(["--color", "never", "--color", "always"]).unwrap();
    assert_eq!(ColorChoice::Always, args.color);

    let result = parse_low_raw(["--color", "foofoo"]);
    assert!(result.is_err(), "{result:?}");

    let result = parse_low_raw(["--color", "Always"]);
    assert!(result.is_err(), "{result:?}");
}

/// --colors
#[derive(Debug)]
struct Colors;

impl Flag for Colors {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "colors"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("COLOR_SPEC")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-colors-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-colors-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let v = v.unwrap_value();
        let v = convert::str(&v)?;
        args.colors.push(v.parse()?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_colors() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert!(args.colors.is_empty());

    let args = parse_low_raw(["--colors", "match:fg:magenta"]).unwrap();
    assert_eq!(args.colors, vec!["match:fg:magenta".parse().unwrap()]);

    let args = parse_low_raw([
        "--colors",
        "match:fg:magenta",
        "--colors",
        "line:bg:yellow",
    ])
    .unwrap();
    assert_eq!(
        args.colors,
        vec![
            "match:fg:magenta".parse().unwrap(),
            "line:bg:yellow".parse().unwrap()
        ]
    );

    let args = parse_low_raw(["--colors", "highlight:bg:240"]).unwrap();
    assert_eq!(args.colors, vec!["highlight:bg:240".parse().unwrap()]);

    let args = parse_low_raw([
        "--colors",
        "match:fg:magenta",
        "--colors",
        "highlight:bg:blue",
    ])
    .unwrap();
    assert_eq!(
        args.colors,
        vec![
            "match:fg:magenta".parse().unwrap(),
            "highlight:bg:blue".parse().unwrap()
        ]
    );
}

/// --column
#[derive(Debug)]
struct Column;

impl Flag for Column {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "column"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-column")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-column-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-column-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.column = Some(v.unwrap_switch());
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_column() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.column);

    let args = parse_low_raw(["--column"]).unwrap();
    assert_eq!(Some(true), args.column);

    let args = parse_low_raw(["--column", "--no-column"]).unwrap();
    assert_eq!(Some(false), args.column);

    let args = parse_low_raw(["--no-column", "--column"]).unwrap();
    assert_eq!(Some(true), args.column);
}

/// -C/--context
#[derive(Debug)]
struct Context;

impl Flag for Context {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'C')
    }
    fn name_long(&self) -> &'static str {
        "context"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-context-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-context-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.context.set_both(convert::usize(&v.unwrap_value())?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_context() {
    let mkctx = |lines| {
        let mut mode = ContextMode::default();
        mode.set_both(lines);
        mode
    };

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(ContextMode::default(), args.context);

    let args = parse_low_raw(["--context", "5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["--context=5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-C", "5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-C5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let args = parse_low_raw(["-C5", "-C10"]).unwrap();
    assert_eq!(mkctx(10), args.context);

    let args = parse_low_raw(["-C5", "-C0"]).unwrap();
    assert_eq!(mkctx(0), args.context);

    let args = parse_low_raw(["-C5", "--passthru"]).unwrap();
    assert_eq!(ContextMode::Passthru, args.context);

    let args = parse_low_raw(["--passthru", "-C5"]).unwrap();
    assert_eq!(mkctx(5), args.context);

    let n = usize::MAX.to_string();
    let args = parse_low_raw(["--context", n.as_str()]).unwrap();
    assert_eq!(mkctx(usize::MAX), args.context);

    #[cfg(target_pointer_width = "64")]
    {
        let n = (u128::from(u64::MAX) + 1).to_string();
        let result = parse_low_raw(["--context", n.as_str()]);
        assert!(result.is_err(), "{result:?}");
    }

    // Test the interaction between -A/-B and -C. Basically, -A/-B always
    // partially overrides -C, regardless of where they appear relative to
    // each other. This behavior is also how GNU grep works, and it also makes
    // logical sense to me: -A/-B are the more specific flags.
    let args = parse_low_raw(["-A1", "-C5"]).unwrap();
    let mut mode = ContextMode::default();
    mode.set_after(1);
    mode.set_both(5);
    assert_eq!(mode, args.context);
    assert_eq!((5, 1), args.context.get_limited());

    let args = parse_low_raw(["-B1", "-C5"]).unwrap();
    let mut mode = ContextMode::default();
    mode.set_before(1);
    mode.set_both(5);
    assert_eq!(mode, args.context);
    assert_eq!((1, 5), args.context.get_limited());

    let args = parse_low_raw(["-A1", "-B2", "-C5"]).unwrap();
    let mut mode = ContextMode::default();
    mode.set_before(2);
    mode.set_after(1);
    mode.set_both(5);
    assert_eq!(mode, args.context);
    assert_eq!((2, 1), args.context.get_limited());

    // These next three are like the ones above, but with -C before -A/-B. This
    // tests that -A and -B only partially override -C. That is, -C1 -A2 is
    // equivalent to -B1 -A2.
    let args = parse_low_raw(["-C5", "-A1"]).unwrap();
    let mut mode = ContextMode::default();
    mode.set_after(1);
    mode.set_both(5);
    assert_eq!(mode, args.context);
    assert_eq!((5, 1), args.context.get_limited());

    let args = parse_low_raw(["-C5", "-B1"]).unwrap();
    let mut mode = ContextMode::default();
    mode.set_before(1);
    mode.set_both(5);
    assert_eq!(mode, args.context);
    assert_eq!((1, 5), args.context.get_limited());

    let args = parse_low_raw(["-C5", "-A1", "-B2"]).unwrap();
    let mut mode = ContextMode::default();
    mode.set_before(2);
    mode.set_after(1);
    mode.set_both(5);
    assert_eq!(mode, args.context);
    assert_eq!((2, 1), args.context.get_limited());
}

/// --context-separator
#[derive(Debug)]
struct ContextSeparator;

impl Flag for ContextSeparator {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "context-separator"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-context-separator")
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("SEPARATOR")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-context-separator-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-context-separator-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        use crate::flags::lowargs::ContextSeparator as Separator;

        args.context_separator = match v {
            FlagValue::Switch(true) => {
                unreachable!("flag can only be disabled")
            }
            FlagValue::Switch(false) => Separator::disabled(),
            FlagValue::Value(v) => Separator::new(&v)?,
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_context_separator() {
    use bstr::BString;

    use crate::flags::lowargs::ContextSeparator as Separator;

    let getbytes = |ctxsep: Separator| ctxsep.into_bytes().map(BString::from);

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Some(BString::from("--")), getbytes(args.context_separator));

    let args = parse_low_raw(["--context-separator", "XYZ"]).unwrap();
    assert_eq!(Some(BString::from("XYZ")), getbytes(args.context_separator));

    let args = parse_low_raw(["--no-context-separator"]).unwrap();
    assert_eq!(None, getbytes(args.context_separator));

    let args = parse_low_raw([
        "--context-separator",
        "XYZ",
        "--no-context-separator",
    ])
    .unwrap();
    assert_eq!(None, getbytes(args.context_separator));

    let args = parse_low_raw([
        "--no-context-separator",
        "--context-separator",
        "XYZ",
    ])
    .unwrap();
    assert_eq!(Some(BString::from("XYZ")), getbytes(args.context_separator));

    // This checks that invalid UTF-8 can be used. This case isn't too tricky
    // to handle, because it passes the invalid UTF-8 as an escape sequence
    // that is itself valid UTF-8. It doesn't become invalid UTF-8 until after
    // the argument is parsed and then unescaped.
    let args = parse_low_raw(["--context-separator", r"\xFF"]).unwrap();
    assert_eq!(Some(BString::from(b"\xFF")), getbytes(args.context_separator));

    // In this case, we specifically try to pass an invalid UTF-8 argument to
    // the flag. In theory we might be able to support this, but because we do
    // unescaping and because unescaping wants valid UTF-8, we do a UTF-8 check
    // on the value. Since we pass invalid UTF-8, it fails. This demonstrates
    // that the only way to use an invalid UTF-8 separator is by specifying an
    // escape sequence that is itself valid UTF-8.
    #[cfg(unix)]
    {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

        let result = parse_low_raw([
            OsStr::from_bytes(b"--context-separator"),
            OsStr::from_bytes(&[0xFF]),
        ]);
        assert!(result.is_err(), "{result:?}");
    }
}

/// -c/--count
#[derive(Debug)]
struct Count;

impl Flag for Count {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'c')
    }
    fn name_long(&self) -> &'static str {
        "count"
    }
    fn doc_category(&self) -> Category {
        Category::OutputModes
    }
    fn doc_short(&self) -> &'static str {
        "flag-count-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-count-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--count can only be enabled");
        args.mode.update(Mode::Search(SearchMode::Count));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_count() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--count"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::Count), args.mode);

    let args = parse_low_raw(["-c"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::Count), args.mode);

    let args = parse_low_raw(["--count-matches", "--count"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::Count), args.mode);

    let args = parse_low_raw(["--count-matches", "-c"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::Count), args.mode);
}

/// --count-matches
#[derive(Debug)]
struct CountMatches;

impl Flag for CountMatches {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "count-matches"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        None
    }
    fn doc_category(&self) -> Category {
        Category::OutputModes
    }
    fn doc_short(&self) -> &'static str {
        "flag-count-matches-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-count-matches-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--count-matches can only be enabled");
        args.mode.update(Mode::Search(SearchMode::CountMatches));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_count_matches() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--count-matches"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::CountMatches), args.mode);

    let args = parse_low_raw(["--count", "--count-matches"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::CountMatches), args.mode);

    let args = parse_low_raw(["-c", "--count-matches"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::CountMatches), args.mode);
}

/// --crlf
#[derive(Debug)]
struct Crlf;

impl Flag for Crlf {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "crlf"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-crlf")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-crlf-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-crlf-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.crlf = v.unwrap_switch();
        if args.crlf {
            args.null_data = false;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_crlf() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.crlf);

    let args = parse_low_raw(["--crlf"]).unwrap();
    assert_eq!(true, args.crlf);
    assert_eq!(false, args.null_data);

    let args = parse_low_raw(["--crlf", "--null-data"]).unwrap();
    assert_eq!(false, args.crlf);
    assert_eq!(true, args.null_data);

    let args = parse_low_raw(["--null-data", "--crlf"]).unwrap();
    assert_eq!(true, args.crlf);
    assert_eq!(false, args.null_data);

    let args = parse_low_raw(["--null-data", "--no-crlf"]).unwrap();
    assert_eq!(false, args.crlf);
    assert_eq!(true, args.null_data);

    let args = parse_low_raw(["--null-data", "--crlf", "--no-crlf"]).unwrap();
    assert_eq!(false, args.crlf);
    assert_eq!(false, args.null_data);
}

/// --debug
#[derive(Debug)]
struct Debug;

impl Flag for Debug {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "debug"
    }
    fn doc_category(&self) -> Category {
        Category::Logging
    }
    fn doc_short(&self) -> &'static str {
        "flag-debug-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-debug-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--debug can only be enabled");
        args.logging = Some(LoggingMode::Debug);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_debug() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.logging);

    let args = parse_low_raw(["--debug"]).unwrap();
    assert_eq!(Some(LoggingMode::Debug), args.logging);

    let args = parse_low_raw(["--trace", "--debug"]).unwrap();
    assert_eq!(Some(LoggingMode::Debug), args.logging);
}

/// --dfa-size-limit
#[derive(Debug)]
struct DfaSizeLimit;

impl Flag for DfaSizeLimit {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "dfa-size-limit"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM+SUFFIX?")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-dfa-size-limit-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-dfa-size-limit-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let v = v.unwrap_value();
        args.dfa_size_limit = Some(convert::human_readable_usize(&v)?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_dfa_size_limit() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.dfa_size_limit);

    #[cfg(target_pointer_width = "64")]
    {
        let args = parse_low_raw(["--dfa-size-limit", "9G"]).unwrap();
        assert_eq!(Some(9 * (1 << 30)), args.dfa_size_limit);

        let args = parse_low_raw(["--dfa-size-limit=9G"]).unwrap();
        assert_eq!(Some(9 * (1 << 30)), args.dfa_size_limit);

        let args =
            parse_low_raw(["--dfa-size-limit=9G", "--dfa-size-limit=0"])
                .unwrap();
        assert_eq!(Some(0), args.dfa_size_limit);
    }

    let args = parse_low_raw(["--dfa-size-limit=0K"]).unwrap();
    assert_eq!(Some(0), args.dfa_size_limit);

    let args = parse_low_raw(["--dfa-size-limit=0M"]).unwrap();
    assert_eq!(Some(0), args.dfa_size_limit);

    let args = parse_low_raw(["--dfa-size-limit=0G"]).unwrap();
    assert_eq!(Some(0), args.dfa_size_limit);

    let result = parse_low_raw(["--dfa-size-limit", "9999999999999999999999"]);
    assert!(result.is_err(), "{result:?}");

    let result = parse_low_raw(["--dfa-size-limit", "9999999999999999G"]);
    assert!(result.is_err(), "{result:?}");
}

/// -E/--encoding
#[derive(Debug)]
struct Encoding;

impl Flag for Encoding {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'E')
    }
    fn name_long(&self) -> &'static str {
        "encoding"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-encoding")
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("ENCODING")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-encoding-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-encoding-long"
    }
    fn completion_type(&self) -> CompletionType {
        CompletionType::Encoding
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let value = match v {
            FlagValue::Value(v) => v,
            FlagValue::Switch(true) => {
                unreachable!("--encoding must accept a value")
            }
            FlagValue::Switch(false) => {
                args.encoding = EncodingMode::Auto;
                return Ok(());
            }
        };
        let label = convert::str(&value)?;
        args.encoding = match label {
            "auto" => EncodingMode::Auto,
            "none" => EncodingMode::Disabled,
            _ => EncodingMode::Some(grep::searcher::Encoding::new(label)?),
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_encoding() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(EncodingMode::Auto, args.encoding);

    let args = parse_low_raw(["--encoding", "auto"]).unwrap();
    assert_eq!(EncodingMode::Auto, args.encoding);

    let args = parse_low_raw(["--encoding", "none"]).unwrap();
    assert_eq!(EncodingMode::Disabled, args.encoding);

    let args = parse_low_raw(["--encoding=none"]).unwrap();
    assert_eq!(EncodingMode::Disabled, args.encoding);

    let args = parse_low_raw(["-E", "none"]).unwrap();
    assert_eq!(EncodingMode::Disabled, args.encoding);

    let args = parse_low_raw(["-Enone"]).unwrap();
    assert_eq!(EncodingMode::Disabled, args.encoding);

    let args = parse_low_raw(["-E", "none", "--no-encoding"]).unwrap();
    assert_eq!(EncodingMode::Auto, args.encoding);

    let args = parse_low_raw(["--no-encoding", "-E", "none"]).unwrap();
    assert_eq!(EncodingMode::Disabled, args.encoding);

    let args = parse_low_raw(["-E", "utf-16"]).unwrap();
    let enc = grep::searcher::Encoding::new("utf-16").unwrap();
    assert_eq!(EncodingMode::Some(enc), args.encoding);

    let args = parse_low_raw(["-E", "utf-16", "--no-encoding"]).unwrap();
    assert_eq!(EncodingMode::Auto, args.encoding);

    let result = parse_low_raw(["-E", "foo"]);
    assert!(result.is_err(), "{result:?}");
}

/// --engine
#[derive(Debug)]
struct Engine;

impl Flag for Engine {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "engine"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("ENGINE")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-engine-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-engine-long"
    }
    fn doc_choices(&self) -> &'static [&'static str] {
        &["default", "pcre2", "auto"]
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let v = v.unwrap_value();
        let string = convert::str(&v)?;
        args.engine = match string {
            "default" => EngineChoice::Default,
            "pcre2" => EngineChoice::PCRE2,
            "auto" => EngineChoice::Auto,
            _ => anyhow::bail!(
                "{}",
                crate::i18n::t_args(
                    "err-regex-engine-unrecognized",
                    &[("engine", string)]
                )
            ),
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_engine() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(EngineChoice::Default, args.engine);

    let args = parse_low_raw(["--engine", "pcre2"]).unwrap();
    assert_eq!(EngineChoice::PCRE2, args.engine);

    let args = parse_low_raw(["--engine=pcre2"]).unwrap();
    assert_eq!(EngineChoice::PCRE2, args.engine);

    let args =
        parse_low_raw(["--auto-hybrid-regex", "--engine=pcre2"]).unwrap();
    assert_eq!(EngineChoice::PCRE2, args.engine);

    let args =
        parse_low_raw(["--engine=pcre2", "--auto-hybrid-regex"]).unwrap();
    assert_eq!(EngineChoice::Auto, args.engine);

    let args =
        parse_low_raw(["--auto-hybrid-regex", "--engine=auto"]).unwrap();
    assert_eq!(EngineChoice::Auto, args.engine);

    let args =
        parse_low_raw(["--auto-hybrid-regex", "--engine=default"]).unwrap();
    assert_eq!(EngineChoice::Default, args.engine);

    let args =
        parse_low_raw(["--engine=pcre2", "--no-auto-hybrid-regex"]).unwrap();
    assert_eq!(EngineChoice::Default, args.engine);
}

/// --field-context-separator
#[derive(Debug)]
struct FieldContextSeparator;

impl Flag for FieldContextSeparator {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "field-context-separator"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("SEPARATOR")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-field-context-separator-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-field-context-separator-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        use crate::flags::lowargs::FieldContextSeparator as Separator;

        args.field_context_separator = Separator::new(&v.unwrap_value())?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_field_context_separator() {
    use bstr::BString;

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(BString::from("-"), args.field_context_separator.into_bytes());

    let args = parse_low_raw(["--field-context-separator", "XYZ"]).unwrap();
    assert_eq!(
        BString::from("XYZ"),
        args.field_context_separator.into_bytes()
    );

    let args = parse_low_raw(["--field-context-separator=XYZ"]).unwrap();
    assert_eq!(
        BString::from("XYZ"),
        args.field_context_separator.into_bytes()
    );

    let args = parse_low_raw([
        "--field-context-separator",
        "XYZ",
        "--field-context-separator",
        "ABC",
    ])
    .unwrap();
    assert_eq!(
        BString::from("ABC"),
        args.field_context_separator.into_bytes()
    );

    let args = parse_low_raw(["--field-context-separator", r"\t"]).unwrap();
    assert_eq!(BString::from("\t"), args.field_context_separator.into_bytes());

    let args = parse_low_raw(["--field-context-separator", r"\x00"]).unwrap();
    assert_eq!(
        BString::from("\x00"),
        args.field_context_separator.into_bytes()
    );

    // This checks that invalid UTF-8 can be used. This case isn't too tricky
    // to handle, because it passes the invalid UTF-8 as an escape sequence
    // that is itself valid UTF-8. It doesn't become invalid UTF-8 until after
    // the argument is parsed and then unescaped.
    let args = parse_low_raw(["--field-context-separator", r"\xFF"]).unwrap();
    assert_eq!(
        BString::from(b"\xFF"),
        args.field_context_separator.into_bytes()
    );

    // In this case, we specifically try to pass an invalid UTF-8 argument to
    // the flag. In theory we might be able to support this, but because we do
    // unescaping and because unescaping wants valid UTF-8, we do a UTF-8 check
    // on the value. Since we pass invalid UTF-8, it fails. This demonstrates
    // that the only way to use an invalid UTF-8 separator is by specifying an
    // escape sequence that is itself valid UTF-8.
    #[cfg(unix)]
    {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

        let result = parse_low_raw([
            OsStr::from_bytes(b"--field-context-separator"),
            OsStr::from_bytes(&[0xFF]),
        ]);
        assert!(result.is_err(), "{result:?}");
    }
}

/// --field-match-separator
#[derive(Debug)]
struct FieldMatchSeparator;

impl Flag for FieldMatchSeparator {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "field-match-separator"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("SEPARATOR")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-field-match-separator-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-field-match-separator-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        use crate::flags::lowargs::FieldMatchSeparator as Separator;

        args.field_match_separator = Separator::new(&v.unwrap_value())?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_field_match_separator() {
    use bstr::BString;

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(BString::from(":"), args.field_match_separator.into_bytes());

    let args = parse_low_raw(["--field-match-separator", "XYZ"]).unwrap();
    assert_eq!(BString::from("XYZ"), args.field_match_separator.into_bytes());

    let args = parse_low_raw(["--field-match-separator=XYZ"]).unwrap();
    assert_eq!(BString::from("XYZ"), args.field_match_separator.into_bytes());

    let args = parse_low_raw([
        "--field-match-separator",
        "XYZ",
        "--field-match-separator",
        "ABC",
    ])
    .unwrap();
    assert_eq!(BString::from("ABC"), args.field_match_separator.into_bytes());

    let args = parse_low_raw(["--field-match-separator", r"\t"]).unwrap();
    assert_eq!(BString::from("\t"), args.field_match_separator.into_bytes());

    let args = parse_low_raw(["--field-match-separator", r"\x00"]).unwrap();
    assert_eq!(BString::from("\x00"), args.field_match_separator.into_bytes());

    // This checks that invalid UTF-8 can be used. This case isn't too tricky
    // to handle, because it passes the invalid UTF-8 as an escape sequence
    // that is itself valid UTF-8. It doesn't become invalid UTF-8 until after
    // the argument is parsed and then unescaped.
    let args = parse_low_raw(["--field-match-separator", r"\xFF"]).unwrap();
    assert_eq!(
        BString::from(b"\xFF"),
        args.field_match_separator.into_bytes()
    );

    // In this case, we specifically try to pass an invalid UTF-8 argument to
    // the flag. In theory we might be able to support this, but because we do
    // unescaping and because unescaping wants valid UTF-8, we do a UTF-8 check
    // on the value. Since we pass invalid UTF-8, it fails. This demonstrates
    // that the only way to use an invalid UTF-8 separator is by specifying an
    // escape sequence that is itself valid UTF-8.
    #[cfg(unix)]
    {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

        let result = parse_low_raw([
            OsStr::from_bytes(b"--field-match-separator"),
            OsStr::from_bytes(&[0xFF]),
        ]);
        assert!(result.is_err(), "{result:?}");
    }
}

/// -f/--file
#[derive(Debug)]
struct File;

impl Flag for File {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'f')
    }
    fn name_long(&self) -> &'static str {
        "file"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("PATTERNFILE")
    }
    fn doc_category(&self) -> Category {
        Category::Input
    }
    fn doc_short(&self) -> &'static str {
        "flag-file-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-file-long"
    }
    fn completion_type(&self) -> CompletionType {
        CompletionType::Filename
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let path = PathBuf::from(v.unwrap_value());
        args.patterns.push(PatternSource::File(path));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_file() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<PatternSource>::new(), args.patterns);

    let args = parse_low_raw(["--file", "foo"]).unwrap();
    assert_eq!(vec![PatternSource::File(PathBuf::from("foo"))], args.patterns);

    let args = parse_low_raw(["--file=foo"]).unwrap();
    assert_eq!(vec![PatternSource::File(PathBuf::from("foo"))], args.patterns);

    let args = parse_low_raw(["-f", "foo"]).unwrap();
    assert_eq!(vec![PatternSource::File(PathBuf::from("foo"))], args.patterns);

    let args = parse_low_raw(["-ffoo"]).unwrap();
    assert_eq!(vec![PatternSource::File(PathBuf::from("foo"))], args.patterns);

    let args = parse_low_raw(["--file", "-foo"]).unwrap();
    assert_eq!(
        vec![PatternSource::File(PathBuf::from("-foo"))],
        args.patterns
    );

    let args = parse_low_raw(["--file=-foo"]).unwrap();
    assert_eq!(
        vec![PatternSource::File(PathBuf::from("-foo"))],
        args.patterns
    );

    let args = parse_low_raw(["-f", "-foo"]).unwrap();
    assert_eq!(
        vec![PatternSource::File(PathBuf::from("-foo"))],
        args.patterns
    );

    let args = parse_low_raw(["-f-foo"]).unwrap();
    assert_eq!(
        vec![PatternSource::File(PathBuf::from("-foo"))],
        args.patterns
    );

    let args = parse_low_raw(["--file=foo", "--file", "bar"]).unwrap();
    assert_eq!(
        vec![
            PatternSource::File(PathBuf::from("foo")),
            PatternSource::File(PathBuf::from("bar"))
        ],
        args.patterns
    );

    // We permit path arguments to be invalid UTF-8. So test that. Some of
    // these cases are tricky and depend on lexopt doing the right thing.
    //
    // We probably should add tests for this handling on Windows too, but paths
    // that are invalid UTF-16 appear incredibly rare in the Windows world.
    #[cfg(unix)]
    {
        use std::{
            ffi::{OsStr, OsString},
            os::unix::ffi::{OsStrExt, OsStringExt},
        };

        let bytes = &[b'A', 0xFF, b'Z'][..];
        let path = PathBuf::from(OsString::from_vec(bytes.to_vec()));

        let args = parse_low_raw([
            OsStr::from_bytes(b"--file"),
            OsStr::from_bytes(bytes),
        ])
        .unwrap();
        assert_eq!(vec![PatternSource::File(path.clone())], args.patterns);

        let args = parse_low_raw([
            OsStr::from_bytes(b"-f"),
            OsStr::from_bytes(bytes),
        ])
        .unwrap();
        assert_eq!(vec![PatternSource::File(path.clone())], args.patterns);

        let mut bytes = b"--file=A".to_vec();
        bytes.push(0xFF);
        bytes.push(b'Z');
        let args = parse_low_raw([OsStr::from_bytes(&bytes)]).unwrap();
        assert_eq!(vec![PatternSource::File(path.clone())], args.patterns);

        let mut bytes = b"-fA".to_vec();
        bytes.push(0xFF);
        bytes.push(b'Z');
        let args = parse_low_raw([OsStr::from_bytes(&bytes)]).unwrap();
        assert_eq!(vec![PatternSource::File(path.clone())], args.patterns);
    }
}

/// --files
#[derive(Debug)]
struct Files;

impl Flag for Files {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "files"
    }
    fn doc_category(&self) -> Category {
        Category::OtherBehaviors
    }
    fn doc_short(&self) -> &'static str {
        "flag-files-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-files-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch());
        args.mode.update(Mode::Files);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_files() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--files"]).unwrap();
    assert_eq!(Mode::Files, args.mode);
}

/// -l/--files-with-matches
#[derive(Debug)]
struct FilesWithMatches;

impl Flag for FilesWithMatches {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'l')
    }
    fn name_long(&self) -> &'static str {
        "files-with-matches"
    }
    fn doc_category(&self) -> Category {
        Category::OutputModes
    }
    fn doc_short(&self) -> &'static str {
        "flag-files-with-matches-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-files-with-matches-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--files-with-matches can only be enabled");
        args.mode.update(Mode::Search(SearchMode::FilesWithMatches));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_files_with_matches() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--files-with-matches"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::FilesWithMatches), args.mode);

    let args = parse_low_raw(["-l"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::FilesWithMatches), args.mode);
}

/// -l/--files-without-match
#[derive(Debug)]
struct FilesWithoutMatch;

impl Flag for FilesWithoutMatch {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "files-without-match"
    }
    fn doc_category(&self) -> Category {
        Category::OutputModes
    }
    fn doc_short(&self) -> &'static str {
        "flag-files-without-match-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-files-without-match-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(
            v.unwrap_switch(),
            "--files-without-match can only be enabled"
        );
        args.mode.update(Mode::Search(SearchMode::FilesWithoutMatch));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_files_without_match() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--files-without-match"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::FilesWithoutMatch), args.mode);

    let args =
        parse_low_raw(["--files-with-matches", "--files-without-match"])
            .unwrap();
    assert_eq!(Mode::Search(SearchMode::FilesWithoutMatch), args.mode);

    let args =
        parse_low_raw(["--files-without-match", "--files-with-matches"])
            .unwrap();
    assert_eq!(Mode::Search(SearchMode::FilesWithMatches), args.mode);
}

/// -F/--fixed-strings
#[derive(Debug)]
struct FixedStrings;

impl Flag for FixedStrings {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'F')
    }
    fn name_long(&self) -> &'static str {
        "fixed-strings"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-fixed-strings")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-fixed-strings-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-fixed-strings-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.fixed_strings = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_fixed_strings() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.fixed_strings);

    let args = parse_low_raw(["--fixed-strings"]).unwrap();
    assert_eq!(true, args.fixed_strings);

    let args = parse_low_raw(["-F"]).unwrap();
    assert_eq!(true, args.fixed_strings);

    let args = parse_low_raw(["-F", "--no-fixed-strings"]).unwrap();
    assert_eq!(false, args.fixed_strings);

    let args = parse_low_raw(["--no-fixed-strings", "-F"]).unwrap();
    assert_eq!(true, args.fixed_strings);
}

/// -L/--follow
#[derive(Debug)]
struct Follow;

impl Flag for Follow {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'L')
    }
    fn name_long(&self) -> &'static str {
        "follow"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-follow")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-follow-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-follow-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.follow = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_follow() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.follow);

    let args = parse_low_raw(["--follow"]).unwrap();
    assert_eq!(true, args.follow);

    let args = parse_low_raw(["-L"]).unwrap();
    assert_eq!(true, args.follow);

    let args = parse_low_raw(["-L", "--no-follow"]).unwrap();
    assert_eq!(false, args.follow);

    let args = parse_low_raw(["--no-follow", "-L"]).unwrap();
    assert_eq!(true, args.follow);
}

/// --generate
#[derive(Debug)]
struct Generate;

impl Flag for Generate {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "generate"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("KIND")
    }
    fn doc_category(&self) -> Category {
        Category::OtherBehaviors
    }
    fn doc_short(&self) -> &'static str {
        "flag-generate-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-generate-long"
    }
    fn doc_choices(&self) -> &'static [&'static str] {
        &[
            "man",
            "complete-bash",
            "complete-zsh",
            "complete-fish",
            "complete-powershell",
        ]
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let genmode = match convert::str(&v.unwrap_value())? {
            "man" => GenerateMode::Man,
            "complete-bash" => GenerateMode::CompleteBash,
            "complete-zsh" => GenerateMode::CompleteZsh,
            "complete-fish" => GenerateMode::CompleteFish,
            "complete-powershell" => GenerateMode::CompletePowerShell,
            unk => anyhow::bail!(
                "{}",
                crate::i18n::t_args(
                    "err-choice-unrecognized",
                    &[("choice", unk)]
                )
            ),
        };
        args.mode.update(Mode::Generate(genmode));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_generate() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--generate", "man"]).unwrap();
    assert_eq!(Mode::Generate(GenerateMode::Man), args.mode);

    let args = parse_low_raw(["--generate", "complete-bash"]).unwrap();
    assert_eq!(Mode::Generate(GenerateMode::CompleteBash), args.mode);

    let args = parse_low_raw(["--generate", "complete-zsh"]).unwrap();
    assert_eq!(Mode::Generate(GenerateMode::CompleteZsh), args.mode);

    let args = parse_low_raw(["--generate", "complete-fish"]).unwrap();
    assert_eq!(Mode::Generate(GenerateMode::CompleteFish), args.mode);

    let args = parse_low_raw(["--generate", "complete-powershell"]).unwrap();
    assert_eq!(Mode::Generate(GenerateMode::CompletePowerShell), args.mode);

    let args =
        parse_low_raw(["--generate", "complete-bash", "--generate=man"])
            .unwrap();
    assert_eq!(Mode::Generate(GenerateMode::Man), args.mode);

    let args = parse_low_raw(["--generate", "man", "-l"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::FilesWithMatches), args.mode);

    // An interesting quirk of how the modes override each other that lets
    // you get back to the "default" mode of searching.
    let args =
        parse_low_raw(["--generate", "man", "--json", "--no-json"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);
}

/// -g/--glob
#[derive(Debug)]
struct Glob;

impl Flag for Glob {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'g')
    }
    fn name_long(&self) -> &'static str {
        "glob"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("GLOB")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-glob-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-glob-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let glob = convert::string(v.unwrap_value())?;
        args.globs.push(glob);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_glob() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<String>::new(), args.globs);

    let args = parse_low_raw(["--glob", "foo"]).unwrap();
    assert_eq!(vec!["foo".to_string()], args.globs);

    let args = parse_low_raw(["--glob=foo"]).unwrap();
    assert_eq!(vec!["foo".to_string()], args.globs);

    let args = parse_low_raw(["-g", "foo"]).unwrap();
    assert_eq!(vec!["foo".to_string()], args.globs);

    let args = parse_low_raw(["-gfoo"]).unwrap();
    assert_eq!(vec!["foo".to_string()], args.globs);

    let args = parse_low_raw(["--glob", "-foo"]).unwrap();
    assert_eq!(vec!["-foo".to_string()], args.globs);

    let args = parse_low_raw(["--glob=-foo"]).unwrap();
    assert_eq!(vec!["-foo".to_string()], args.globs);

    let args = parse_low_raw(["-g", "-foo"]).unwrap();
    assert_eq!(vec!["-foo".to_string()], args.globs);

    let args = parse_low_raw(["-g-foo"]).unwrap();
    assert_eq!(vec!["-foo".to_string()], args.globs);
}

/// --glob-case-insensitive
#[derive(Debug)]
struct GlobCaseInsensitive;

impl Flag for GlobCaseInsensitive {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "glob-case-insensitive"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-glob-case-insensitive")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-glob-case-insensitive-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-glob-case-insensitive-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.glob_case_insensitive = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_glob_case_insensitive() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.glob_case_insensitive);

    let args = parse_low_raw(["--glob-case-insensitive"]).unwrap();
    assert_eq!(true, args.glob_case_insensitive);

    let args = parse_low_raw([
        "--glob-case-insensitive",
        "--no-glob-case-insensitive",
    ])
    .unwrap();
    assert_eq!(false, args.glob_case_insensitive);

    let args = parse_low_raw([
        "--no-glob-case-insensitive",
        "--glob-case-insensitive",
    ])
    .unwrap();
    assert_eq!(true, args.glob_case_insensitive);
}

/// --heading
#[derive(Debug)]
struct Heading;

impl Flag for Heading {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "heading"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-heading")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-heading-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-heading-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.heading = Some(v.unwrap_switch());
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_heading() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.heading);

    let args = parse_low_raw(["--heading"]).unwrap();
    assert_eq!(Some(true), args.heading);

    let args = parse_low_raw(["--no-heading"]).unwrap();
    assert_eq!(Some(false), args.heading);

    let args = parse_low_raw(["--heading", "--no-heading"]).unwrap();
    assert_eq!(Some(false), args.heading);

    let args = parse_low_raw(["--no-heading", "--heading"]).unwrap();
    assert_eq!(Some(true), args.heading);
}

/// -h/--help
#[derive(Debug)]
struct Help;

impl Flag for Help {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "help"
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'h')
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-help-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-help-long"
    }

    fn update(&self, v: FlagValue, _: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--help has no negation");
        // Since this flag has different semantics for -h and --help and the
        // Flag trait doesn't support encoding this sort of thing, we handle it
        // as a special case in the parser.
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_help() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.special);

    let args = parse_low_raw(["-h"]).unwrap();
    assert_eq!(Some(SpecialMode::HelpShort), args.special);

    let args = parse_low_raw(["--help"]).unwrap();
    assert_eq!(Some(SpecialMode::HelpLong), args.special);

    let args = parse_low_raw(["-h", "--help"]).unwrap();
    assert_eq!(Some(SpecialMode::HelpLong), args.special);

    let args = parse_low_raw(["--help", "-h"]).unwrap();
    assert_eq!(Some(SpecialMode::HelpShort), args.special);
}

/// -./--hidden
#[derive(Debug)]
struct Hidden;

impl Flag for Hidden {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'.')
    }
    fn name_long(&self) -> &'static str {
        "hidden"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-hidden")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-hidden-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-hidden-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.hidden = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_hidden() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.hidden);

    let args = parse_low_raw(["--hidden"]).unwrap();
    assert_eq!(true, args.hidden);

    let args = parse_low_raw(["-."]).unwrap();
    assert_eq!(true, args.hidden);

    let args = parse_low_raw(["-.", "--no-hidden"]).unwrap();
    assert_eq!(false, args.hidden);

    let args = parse_low_raw(["--no-hidden", "-."]).unwrap();
    assert_eq!(true, args.hidden);
}

/// --hostname-bin
#[derive(Debug)]
struct HostnameBin;

impl Flag for HostnameBin {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "hostname-bin"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("COMMAND")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-hostname-bin-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-hostname-bin-long"
    }
    fn completion_type(&self) -> CompletionType {
        CompletionType::Executable
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let path = PathBuf::from(v.unwrap_value());
        args.hostname_bin =
            if path.as_os_str().is_empty() { None } else { Some(path) };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_hostname_bin() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.hostname_bin);

    let args = parse_low_raw(["--hostname-bin", "foo"]).unwrap();
    assert_eq!(Some(PathBuf::from("foo")), args.hostname_bin);

    let args = parse_low_raw(["--hostname-bin=foo"]).unwrap();
    assert_eq!(Some(PathBuf::from("foo")), args.hostname_bin);
}

/// --hyperlink-format
#[derive(Debug)]
struct HyperlinkFormat;

impl Flag for HyperlinkFormat {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "hyperlink-format"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("FORMAT")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-hyperlink-format-short"
    }
    fn doc_long(&self) -> &'static str {
        static DOC: LazyLock<String> = LazyLock::new(|| {
            let mut doc = crate::i18n::t("flag-hyperlink-format-long-prefix");

            let mut aliases = grep::printer::hyperlink_aliases();
            aliases.sort_by_key(|alias| {
                alias.display_priority().unwrap_or(i16::MAX)
            });
            for (i, alias) in aliases.iter().enumerate() {
                doc.push_str(r"\fB");
                doc.push_str(alias.name());
                doc.push_str(r"\fP");
                doc.push_str(if i < aliases.len() - 1 { ", " } else { "." });
            }
            doc.push_str(&crate::i18n::t("flag-hyperlink-format-long-suffix"));
            doc
        });
        &DOC
    }

    fn doc_choices(&self) -> &'static [&'static str] {
        static CHOICES: LazyLock<Vec<String>> = LazyLock::new(|| {
            let mut aliases = grep::printer::hyperlink_aliases();
            aliases.sort_by_key(|alias| {
                alias.display_priority().unwrap_or(i16::MAX)
            });
            aliases.iter().map(|alias| alias.name().to_string()).collect()
        });
        static BORROWED: LazyLock<Vec<&'static str>> =
            LazyLock::new(|| CHOICES.iter().map(|name| &**name).collect());
        &*BORROWED
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let v = v.unwrap_value();
        let string = convert::str(&v)?;
        let format = string
            .parse()
            .context(crate::i18n::t("err-invalid-hyperlink-format"))?;
        args.hyperlink_format = format;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_hyperlink_format() {
    let parseformat = |format: &str| {
        format.parse::<grep::printer::HyperlinkFormat>().unwrap()
    };

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(parseformat("none"), args.hyperlink_format);

    let args = parse_low_raw(["--hyperlink-format", "default"]).unwrap();
    #[cfg(windows)]
    assert_eq!(parseformat("file://{path}"), args.hyperlink_format);
    #[cfg(not(windows))]
    assert_eq!(parseformat("file://{host}{path}"), args.hyperlink_format);

    let args = parse_low_raw(["--hyperlink-format", "file"]).unwrap();
    assert_eq!(parseformat("file://{host}{path}"), args.hyperlink_format);

    let args = parse_low_raw([
        "--hyperlink-format",
        "file",
        "--hyperlink-format=grep+",
    ])
    .unwrap();
    assert_eq!(parseformat("grep+://{path}:{line}"), args.hyperlink_format);

    let args =
        parse_low_raw(["--hyperlink-format", "file://{host}{path}#{line}"])
            .unwrap();
    assert_eq!(
        parseformat("file://{host}{path}#{line}"),
        args.hyperlink_format
    );

    let result = parse_low_raw(["--hyperlink-format", "file://heythere"]);
    assert!(result.is_err(), "{result:?}");
}

/// --iglob
#[derive(Debug)]
struct IGlob;

impl Flag for IGlob {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "iglob"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("GLOB")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-i-glob-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-i-glob-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let glob = convert::string(v.unwrap_value())?;
        args.iglobs.push(glob);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_iglob() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<String>::new(), args.iglobs);

    let args = parse_low_raw(["--iglob", "foo"]).unwrap();
    assert_eq!(vec!["foo".to_string()], args.iglobs);

    let args = parse_low_raw(["--iglob=foo"]).unwrap();
    assert_eq!(vec!["foo".to_string()], args.iglobs);

    let args = parse_low_raw(["--iglob", "-foo"]).unwrap();
    assert_eq!(vec!["-foo".to_string()], args.iglobs);

    let args = parse_low_raw(["--iglob=-foo"]).unwrap();
    assert_eq!(vec!["-foo".to_string()], args.iglobs);
}

/// -i/--ignore-case
#[derive(Debug)]
struct IgnoreCase;

impl Flag for IgnoreCase {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'i')
    }
    fn name_long(&self) -> &'static str {
        "ignore-case"
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-ignore-case-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-ignore-case-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "flag has no negation");
        args.case = CaseMode::Insensitive;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_ignore_case() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(CaseMode::Sensitive, args.case);

    let args = parse_low_raw(["--ignore-case"]).unwrap();
    assert_eq!(CaseMode::Insensitive, args.case);

    let args = parse_low_raw(["-i"]).unwrap();
    assert_eq!(CaseMode::Insensitive, args.case);

    let args = parse_low_raw(["-i", "-s"]).unwrap();
    assert_eq!(CaseMode::Sensitive, args.case);

    let args = parse_low_raw(["-s", "-i"]).unwrap();
    assert_eq!(CaseMode::Insensitive, args.case);
}

/// --ignore-file
#[derive(Debug)]
struct IgnoreFile;

impl Flag for IgnoreFile {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "ignore-file"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("PATH")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-ignore-file-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-ignore-file-long"
    }
    fn completion_type(&self) -> CompletionType {
        CompletionType::Filename
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let path = PathBuf::from(v.unwrap_value());
        args.ignore_file.push(path);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_ignore_file() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<PathBuf>::new(), args.ignore_file);

    let args = parse_low_raw(["--ignore-file", "foo"]).unwrap();
    assert_eq!(vec![PathBuf::from("foo")], args.ignore_file);

    let args = parse_low_raw(["--ignore-file", "foo", "--ignore-file", "bar"])
        .unwrap();
    assert_eq!(
        vec![PathBuf::from("foo"), PathBuf::from("bar")],
        args.ignore_file
    );
}

/// --ignore-file-case-insensitive
#[derive(Debug)]
struct IgnoreFileCaseInsensitive;

impl Flag for IgnoreFileCaseInsensitive {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "ignore-file-case-insensitive"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-ignore-file-case-insensitive")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-ignore-file-case-insensitive-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-ignore-file-case-insensitive-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.ignore_file_case_insensitive = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_ignore_file_case_insensitive() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.ignore_file_case_insensitive);

    let args = parse_low_raw(["--ignore-file-case-insensitive"]).unwrap();
    assert_eq!(true, args.ignore_file_case_insensitive);

    let args = parse_low_raw([
        "--ignore-file-case-insensitive",
        "--no-ignore-file-case-insensitive",
    ])
    .unwrap();
    assert_eq!(false, args.ignore_file_case_insensitive);

    let args = parse_low_raw([
        "--no-ignore-file-case-insensitive",
        "--ignore-file-case-insensitive",
    ])
    .unwrap();
    assert_eq!(true, args.ignore_file_case_insensitive);
}

/// --include-zero
#[derive(Debug)]
struct IncludeZero;

impl Flag for IncludeZero {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "include-zero"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-include-zero")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-include-zero-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-include-zero-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.include_zero = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_include_zero() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.include_zero);

    let args = parse_low_raw(["--include-zero"]).unwrap();
    assert_eq!(true, args.include_zero);

    let args = parse_low_raw(["--include-zero", "--no-include-zero"]).unwrap();
    assert_eq!(false, args.include_zero);
}

/// -X/--index
#[derive(Debug)]
struct Index;

impl Flag for Index {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'X')
    }
    fn name_long(&self) -> &'static str {
        "index"
    }
    fn doc_category(&self) -> Category {
        Category::Indexing
    }
    fn doc_short(&self) -> &'static str {
        "flag-index-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-index-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        check_indexing_allowed()?;
        assert!(v.unwrap_switch(), "--index has no negation");
        args.index = args.index.saturating_add(1);
        anyhow::ensure!(
            args.index <= 2,
            "{}",
            crate::i18n::t("err-index-at-most-twice")
        );
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_index() {
    if !cfg!(feature = "unstable-index") {
        assert!(parse_low_raw(["-X"]).is_err());
        return;
    }

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(0, args.index);
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["-X"]).unwrap();
    assert_eq!(1, args.index);
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--index"]).unwrap();
    assert_eq!(1, args.index);

    let args = parse_low_raw(["-XX"]).unwrap();
    assert_eq!(2, args.index);

    let args = parse_low_raw(["-X", "--index"]).unwrap();
    assert_eq!(2, args.index);

    let result = parse_low_raw(["-XXX"]);
    assert!(result.is_err(), "{result:?}");
}

/// --x-crud
#[derive(Debug)]
struct IndexCrud;

impl Flag for IndexCrud {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "x-crud"
    }
    fn doc_category(&self) -> Category {
        Category::Indexing
    }
    fn doc_short(&self) -> &'static str {
        "flag-index-crud-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-index-crud-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        check_indexing_allowed()?;
        assert!(v.unwrap_switch(), "--x-crud has no negation");
        args.mode.update(Mode::Index(IndexMode::Crud));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_index_crud() {
    if !cfg!(feature = "unstable-index") {
        assert!(parse_low_raw(["--x-crud"]).is_err());
        return;
    }

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--x-crud"]).unwrap();
    assert_eq!(Mode::Index(IndexMode::Crud), args.mode);

    let args = parse_low_raw(["--files", "--x-crud", "foo"]).unwrap();
    assert_eq!(Mode::Index(IndexMode::Crud), args.mode);
    assert_eq!(vec![std::ffi::OsString::from("foo")], args.positional);

    let args = parse_low_raw(["--x-crud", "--files"]).unwrap();
    assert_eq!(Mode::Files, args.mode);
}

/// --x-force
#[derive(Debug)]
struct IndexForce;

impl Flag for IndexForce {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "x-force"
    }
    fn doc_category(&self) -> Category {
        Category::Indexing
    }
    fn doc_short(&self) -> &'static str {
        "flag-index-force-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-index-force-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        check_indexing_allowed()?;
        assert!(v.unwrap_switch(), "--x-force has no negation");
        args.index_force = true;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_index_force() {
    if !cfg!(feature = "unstable-index") {
        assert!(parse_low_raw(["--x-force"]).is_err());
        return;
    }

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.index_force);

    let args = parse_low_raw(["--x-force"]).unwrap();
    assert_eq!(true, args.index_force);
}

/// --x-path
#[derive(Debug)]
struct IndexPath;

impl Flag for IndexPath {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "x-path"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("PATH")
    }
    fn doc_category(&self) -> Category {
        Category::Indexing
    }
    fn doc_short(&self) -> &'static str {
        "flag-index-path-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-index-path-long"
    }
    fn completion_type(&self) -> CompletionType {
        CompletionType::Filename
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        check_indexing_allowed()?;
        args.index_path = Some(PathBuf::from(v.unwrap_value()));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_index_path() {
    if !cfg!(feature = "unstable-index") {
        assert!(parse_low_raw(["--x-path"]).is_err());
        return;
    }

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.index_path);

    let args = parse_low_raw(["--x-path", "foo"]).unwrap();
    assert_eq!(Some(PathBuf::from("foo")), args.index_path);

    let args = parse_low_raw(["--x-path=foo", "--x-path", "bar"]).unwrap();
    assert_eq!(Some(PathBuf::from("bar")), args.index_path);
}

/// -v/--invert-match
#[derive(Debug)]
struct InvertMatch;

impl Flag for InvertMatch {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'v')
    }
    fn name_long(&self) -> &'static str {
        "invert-match"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-invert-match")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-invert-match-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-invert-match-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.invert_match = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_invert_match() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.invert_match);

    let args = parse_low_raw(["--invert-match"]).unwrap();
    assert_eq!(true, args.invert_match);

    let args = parse_low_raw(["-v"]).unwrap();
    assert_eq!(true, args.invert_match);

    let args = parse_low_raw(["-v", "--no-invert-match"]).unwrap();
    assert_eq!(false, args.invert_match);
}

/// --json
#[derive(Debug)]
struct JSON;

impl Flag for JSON {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "json"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-json")
    }
    fn doc_category(&self) -> Category {
        Category::OutputModes
    }
    fn doc_short(&self) -> &'static str {
        "flag-json-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-json-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        if v.unwrap_switch() {
            args.mode.update(Mode::Search(SearchMode::JSON));
        } else if matches!(args.mode, Mode::Search(SearchMode::JSON)) {
            // --no-json only reverts to the default mode if the mode is
            // JSON, otherwise it's a no-op.
            args.mode.update(Mode::Search(SearchMode::Standard));
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_json() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--json"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::JSON), args.mode);

    let args = parse_low_raw(["--json", "--no-json"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--json", "--files", "--no-json"]).unwrap();
    assert_eq!(Mode::Files, args.mode);

    let args = parse_low_raw(["--json", "-l", "--no-json"]).unwrap();
    assert_eq!(Mode::Search(SearchMode::FilesWithMatches), args.mode);
}

/// --line-buffered
#[derive(Debug)]
struct LineBuffered;

impl Flag for LineBuffered {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "line-buffered"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-line-buffered")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-line-buffered-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-line-buffered-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.buffer = if v.unwrap_switch() {
            BufferMode::Line
        } else {
            BufferMode::Auto
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_line_buffered() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(BufferMode::Auto, args.buffer);

    let args = parse_low_raw(["--line-buffered"]).unwrap();
    assert_eq!(BufferMode::Line, args.buffer);

    let args =
        parse_low_raw(["--line-buffered", "--no-line-buffered"]).unwrap();
    assert_eq!(BufferMode::Auto, args.buffer);

    let args = parse_low_raw(["--line-buffered", "--block-buffered"]).unwrap();
    assert_eq!(BufferMode::Block, args.buffer);
}

/// -n/--line-number
#[derive(Debug)]
struct LineNumber;

impl Flag for LineNumber {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'n')
    }
    fn name_long(&self) -> &'static str {
        "line-number"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-line-number-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-line-number-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--line-number has no automatic negation");
        args.line_number = Some(true);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_line_number() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.line_number);

    let args = parse_low_raw(["--line-number"]).unwrap();
    assert_eq!(Some(true), args.line_number);

    let args = parse_low_raw(["-n"]).unwrap();
    assert_eq!(Some(true), args.line_number);

    let args = parse_low_raw(["-n", "--no-line-number"]).unwrap();
    assert_eq!(Some(false), args.line_number);
}

/// -N/--no-line-number
#[derive(Debug)]
struct LineNumberNo;

impl Flag for LineNumberNo {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'N')
    }
    fn name_long(&self) -> &'static str {
        "no-line-number"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-line-number-no-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-line-number-no-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(
            v.unwrap_switch(),
            "--no-line-number has no automatic negation"
        );
        args.line_number = Some(false);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_line_number() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.line_number);

    let args = parse_low_raw(["--no-line-number"]).unwrap();
    assert_eq!(Some(false), args.line_number);

    let args = parse_low_raw(["-N"]).unwrap();
    assert_eq!(Some(false), args.line_number);

    let args = parse_low_raw(["-N", "--line-number"]).unwrap();
    assert_eq!(Some(true), args.line_number);
}

/// -x/--line-regexp
#[derive(Debug)]
struct LineRegexp;

impl Flag for LineRegexp {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'x')
    }
    fn name_long(&self) -> &'static str {
        "line-regexp"
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-line-regexp-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-line-regexp-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--line-regexp has no negation");
        args.boundary = Some(BoundaryMode::Line);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_line_regexp() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.boundary);

    let args = parse_low_raw(["--line-regexp"]).unwrap();
    assert_eq!(Some(BoundaryMode::Line), args.boundary);

    let args = parse_low_raw(["-x"]).unwrap();
    assert_eq!(Some(BoundaryMode::Line), args.boundary);
}

/// -M/--max-columns
#[derive(Debug)]
struct MaxColumns;

impl Flag for MaxColumns {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'M')
    }
    fn name_long(&self) -> &'static str {
        "max-columns"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-max-columns-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-max-columns-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let max = convert::u64(&v.unwrap_value())?;
        args.max_columns = if max == 0 { None } else { Some(max) };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_max_columns() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.max_columns);

    let args = parse_low_raw(["--max-columns", "5"]).unwrap();
    assert_eq!(Some(5), args.max_columns);

    let args = parse_low_raw(["-M", "5"]).unwrap();
    assert_eq!(Some(5), args.max_columns);

    let args = parse_low_raw(["-M5"]).unwrap();
    assert_eq!(Some(5), args.max_columns);

    let args = parse_low_raw(["--max-columns", "5", "-M0"]).unwrap();
    assert_eq!(None, args.max_columns);
}

/// --max-columns-preview
#[derive(Debug)]
struct MaxColumnsPreview;

impl Flag for MaxColumnsPreview {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "max-columns-preview"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-max-columns-preview")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-max-columns-preview-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-max-columns-preview-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.max_columns_preview = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_max_columns_preview() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.max_columns_preview);

    let args = parse_low_raw(["--max-columns-preview"]).unwrap();
    assert_eq!(true, args.max_columns_preview);

    let args =
        parse_low_raw(["--max-columns-preview", "--no-max-columns-preview"])
            .unwrap();
    assert_eq!(false, args.max_columns_preview);
}

/// -m/--max-count
#[derive(Debug)]
struct MaxCount;

impl Flag for MaxCount {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'm')
    }
    fn name_long(&self) -> &'static str {
        "max-count"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-max-count-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-max-count-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.max_count = Some(convert::u64(&v.unwrap_value())?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_max_count() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.max_count);

    let args = parse_low_raw(["--max-count", "5"]).unwrap();
    assert_eq!(Some(5), args.max_count);

    let args = parse_low_raw(["-m", "5"]).unwrap();
    assert_eq!(Some(5), args.max_count);

    let args = parse_low_raw(["-m", "5", "--max-count=10"]).unwrap();
    assert_eq!(Some(10), args.max_count);
    let args = parse_low_raw(["-m0"]).unwrap();
    assert_eq!(Some(0), args.max_count);
}

/// --max-depth
#[derive(Debug)]
struct MaxDepth;

impl Flag for MaxDepth {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'd')
    }
    fn name_long(&self) -> &'static str {
        "max-depth"
    }
    fn aliases(&self) -> &'static [&'static str] {
        &["maxdepth"]
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-max-depth-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-max-depth-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.max_depth = Some(convert::usize(&v.unwrap_value())?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_max_depth() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.max_depth);

    let args = parse_low_raw(["--max-depth", "5"]).unwrap();
    assert_eq!(Some(5), args.max_depth);

    let args = parse_low_raw(["-d", "5"]).unwrap();
    assert_eq!(Some(5), args.max_depth);

    let args = parse_low_raw(["--max-depth", "5", "--max-depth=10"]).unwrap();
    assert_eq!(Some(10), args.max_depth);

    let args = parse_low_raw(["--max-depth", "0"]).unwrap();
    assert_eq!(Some(0), args.max_depth);

    let args = parse_low_raw(["--maxdepth", "5"]).unwrap();
    assert_eq!(Some(5), args.max_depth);
}

/// --max-filesize
#[derive(Debug)]
struct MaxFilesize;

impl Flag for MaxFilesize {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "max-filesize"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM+SUFFIX?")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-max-filesize-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-max-filesize-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let v = v.unwrap_value();
        args.max_filesize = Some(convert::human_readable_u64(&v)?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_max_filesize() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.max_filesize);

    let args = parse_low_raw(["--max-filesize", "1024"]).unwrap();
    assert_eq!(Some(1024), args.max_filesize);

    let args = parse_low_raw(["--max-filesize", "1K"]).unwrap();
    assert_eq!(Some(1024), args.max_filesize);

    let args =
        parse_low_raw(["--max-filesize", "1K", "--max-filesize=1M"]).unwrap();
    assert_eq!(Some(1024 * 1024), args.max_filesize);
}

/// --mmap
#[derive(Debug)]
struct Mmap;

impl Flag for Mmap {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "mmap"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-mmap")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-mmap-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-mmap-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.mmap = if v.unwrap_switch() {
            MmapMode::AlwaysTryMmap
        } else {
            MmapMode::Never
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_mmap() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(MmapMode::Auto, args.mmap);

    let args = parse_low_raw(["--mmap"]).unwrap();
    assert_eq!(MmapMode::AlwaysTryMmap, args.mmap);

    let args = parse_low_raw(["--no-mmap"]).unwrap();
    assert_eq!(MmapMode::Never, args.mmap);

    let args = parse_low_raw(["--mmap", "--no-mmap"]).unwrap();
    assert_eq!(MmapMode::Never, args.mmap);

    let args = parse_low_raw(["--no-mmap", "--mmap"]).unwrap();
    assert_eq!(MmapMode::AlwaysTryMmap, args.mmap);
}

/// -U/--multiline
#[derive(Debug)]
struct Multiline;

impl Flag for Multiline {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'U')
    }
    fn name_long(&self) -> &'static str {
        "multiline"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-multiline")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-multiline-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-multiline-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.multiline = v.unwrap_switch();
        if args.multiline {
            args.stop_on_nonmatch = false;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_multiline() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.multiline);

    let args = parse_low_raw(["--multiline"]).unwrap();
    assert_eq!(true, args.multiline);

    let args = parse_low_raw(["-U"]).unwrap();
    assert_eq!(true, args.multiline);

    let args = parse_low_raw(["-U", "--no-multiline"]).unwrap();
    assert_eq!(false, args.multiline);
}

/// --multiline-dotall
#[derive(Debug)]
struct MultilineDotall;

impl Flag for MultilineDotall {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "multiline-dotall"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-multiline-dotall")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-multiline-dotall-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-multiline-dotall-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.multiline_dotall = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_multiline_dotall() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.multiline_dotall);

    let args = parse_low_raw(["--multiline-dotall"]).unwrap();
    assert_eq!(true, args.multiline_dotall);

    let args = parse_low_raw(["--multiline-dotall", "--no-multiline-dotall"])
        .unwrap();
    assert_eq!(false, args.multiline_dotall);
}

/// --no-config
#[derive(Debug)]
struct NoConfig;

impl Flag for NoConfig {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-config"
    }
    fn doc_category(&self) -> Category {
        Category::OtherBehaviors
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-config-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-config-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--no-config has no negation");
        args.no_config = true;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_config() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_config);

    let args = parse_low_raw(["--no-config"]).unwrap();
    assert_eq!(true, args.no_config);
}

/// --no-ignore
#[derive(Debug)]
struct NoIgnore;

impl Flag for NoIgnore {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-ignore"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-ignore-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-ignore-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let yes = v.unwrap_switch();
        args.no_ignore_dot = yes;
        args.no_ignore_exclude = yes;
        args.no_ignore_global = yes;
        args.no_ignore_parent = yes;
        args.no_ignore_vcs = yes;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_ignore() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_dot);
    assert_eq!(false, args.no_ignore_exclude);
    assert_eq!(false, args.no_ignore_global);
    assert_eq!(false, args.no_ignore_parent);
    assert_eq!(false, args.no_ignore_vcs);

    let args = parse_low_raw(["--no-ignore"]).unwrap();
    assert_eq!(true, args.no_ignore_dot);
    assert_eq!(true, args.no_ignore_exclude);
    assert_eq!(true, args.no_ignore_global);
    assert_eq!(true, args.no_ignore_parent);
    assert_eq!(true, args.no_ignore_vcs);

    let args = parse_low_raw(["--no-ignore", "--ignore"]).unwrap();
    assert_eq!(false, args.no_ignore_dot);
    assert_eq!(false, args.no_ignore_exclude);
    assert_eq!(false, args.no_ignore_global);
    assert_eq!(false, args.no_ignore_parent);
    assert_eq!(false, args.no_ignore_vcs);
}

/// --no-ignore-dot
#[derive(Debug)]
struct NoIgnoreDot;

impl Flag for NoIgnoreDot {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-ignore-dot"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore-dot")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-ignore-dot-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-ignore-dot-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_ignore_dot = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_ignore_dot() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_dot);

    let args = parse_low_raw(["--no-ignore-dot"]).unwrap();
    assert_eq!(true, args.no_ignore_dot);

    let args = parse_low_raw(["--no-ignore-dot", "--ignore-dot"]).unwrap();
    assert_eq!(false, args.no_ignore_dot);
}

/// --no-ignore-exclude
#[derive(Debug)]
struct NoIgnoreExclude;

impl Flag for NoIgnoreExclude {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-ignore-exclude"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore-exclude")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-ignore-exclude-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-ignore-exclude-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_ignore_exclude = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_ignore_exclude() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_exclude);

    let args = parse_low_raw(["--no-ignore-exclude"]).unwrap();
    assert_eq!(true, args.no_ignore_exclude);

    let args =
        parse_low_raw(["--no-ignore-exclude", "--ignore-exclude"]).unwrap();
    assert_eq!(false, args.no_ignore_exclude);
}

/// --no-ignore-files
#[derive(Debug)]
struct NoIgnoreFiles;

impl Flag for NoIgnoreFiles {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-ignore-files"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore-files")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-ignore-files-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-ignore-files-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_ignore_files = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_ignore_files() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_files);

    let args = parse_low_raw(["--no-ignore-files"]).unwrap();
    assert_eq!(true, args.no_ignore_files);

    let args = parse_low_raw(["--no-ignore-files", "--ignore-files"]).unwrap();
    assert_eq!(false, args.no_ignore_files);
}

/// --no-ignore-global
#[derive(Debug)]
struct NoIgnoreGlobal;

impl Flag for NoIgnoreGlobal {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-ignore-global"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore-global")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-ignore-global-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-ignore-global-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_ignore_global = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_ignore_global() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_global);

    let args = parse_low_raw(["--no-ignore-global"]).unwrap();
    assert_eq!(true, args.no_ignore_global);

    let args =
        parse_low_raw(["--no-ignore-global", "--ignore-global"]).unwrap();
    assert_eq!(false, args.no_ignore_global);
}

/// --no-ignore-messages
#[derive(Debug)]
struct NoIgnoreMessages;

impl Flag for NoIgnoreMessages {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-ignore-messages"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore-messages")
    }
    fn doc_category(&self) -> Category {
        Category::Logging
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-ignore-messages-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-ignore-messages-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_ignore_messages = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_ignore_messages() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_messages);

    let args = parse_low_raw(["--no-ignore-messages"]).unwrap();
    assert_eq!(true, args.no_ignore_messages);

    let args =
        parse_low_raw(["--no-ignore-messages", "--ignore-messages"]).unwrap();
    assert_eq!(false, args.no_ignore_messages);
}

/// --no-ignore-parent
#[derive(Debug)]
struct NoIgnoreParent;

impl Flag for NoIgnoreParent {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-ignore-parent"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore-parent")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-ignore-parent-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-ignore-parent-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_ignore_parent = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_ignore_parent() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_parent);

    let args = parse_low_raw(["--no-ignore-parent"]).unwrap();
    assert_eq!(true, args.no_ignore_parent);

    let args =
        parse_low_raw(["--no-ignore-parent", "--ignore-parent"]).unwrap();
    assert_eq!(false, args.no_ignore_parent);
}

/// --no-ignore-vcs
#[derive(Debug)]
struct NoIgnoreVcs;

impl Flag for NoIgnoreVcs {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-ignore-vcs"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("ignore-vcs")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-ignore-vcs-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-ignore-vcs-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_ignore_vcs = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_ignore_vcs() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_vcs);

    let args = parse_low_raw(["--no-ignore-vcs"]).unwrap();
    assert_eq!(true, args.no_ignore_vcs);

    let args = parse_low_raw(["--no-ignore-vcs", "--ignore-vcs"]).unwrap();
    assert_eq!(false, args.no_ignore_vcs);
}

/// --no-messages
#[derive(Debug)]
struct NoMessages;

impl Flag for NoMessages {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-messages"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("messages")
    }
    fn doc_category(&self) -> Category {
        Category::Logging
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-messages-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-messages-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_messages = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_messages() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_messages);

    let args = parse_low_raw(["--no-messages"]).unwrap();
    assert_eq!(true, args.no_messages);

    let args = parse_low_raw(["--no-messages", "--messages"]).unwrap();
    assert_eq!(false, args.no_messages);
}

/// --no-pcre2-unicode
#[derive(Debug)]
struct NoPcre2Unicode;

impl Flag for NoPcre2Unicode {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-pcre2-unicode"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("pcre2-unicode")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-pcre2unicode-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-pcre2unicode-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_unicode = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_pcre2_unicode() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_unicode);

    let args = parse_low_raw(["--no-pcre2-unicode"]).unwrap();
    assert_eq!(true, args.no_unicode);

    let args =
        parse_low_raw(["--no-pcre2-unicode", "--pcre2-unicode"]).unwrap();
    assert_eq!(false, args.no_unicode);
}

/// --no-require-git
#[derive(Debug)]
struct NoRequireGit;

impl Flag for NoRequireGit {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-require-git"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("require-git")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-require-git-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-require-git-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_require_git = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_require_git() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_require_git);

    let args = parse_low_raw(["--no-require-git"]).unwrap();
    assert_eq!(true, args.no_require_git);

    let args = parse_low_raw(["--no-require-git", "--require-git"]).unwrap();
    assert_eq!(false, args.no_require_git);
}

/// --no-unicode
#[derive(Debug)]
struct NoUnicode;

impl Flag for NoUnicode {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "no-unicode"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("unicode")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-no-unicode-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-no-unicode-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.no_unicode = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_no_unicode() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_unicode);

    let args = parse_low_raw(["--no-unicode"]).unwrap();
    assert_eq!(true, args.no_unicode);

    let args = parse_low_raw(["--no-unicode", "--unicode"]).unwrap();
    assert_eq!(false, args.no_unicode);

    let args = parse_low_raw(["--no-unicode", "--pcre2-unicode"]).unwrap();
    assert_eq!(false, args.no_unicode);

    let args = parse_low_raw(["--no-pcre2-unicode", "--unicode"]).unwrap();
    assert_eq!(false, args.no_unicode);
}

/// -0/--null
#[derive(Debug)]
struct Null;

impl Flag for Null {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'0')
    }
    fn name_long(&self) -> &'static str {
        "null"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-null-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-null-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--null has no negation");
        args.null = true;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_null() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.null);

    let args = parse_low_raw(["--null"]).unwrap();
    assert_eq!(true, args.null);

    let args = parse_low_raw(["-0"]).unwrap();
    assert_eq!(true, args.null);
}

/// --null-data
#[derive(Debug)]
struct NullData;

impl Flag for NullData {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "null-data"
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-null-data-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-null-data-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--null-data has no negation");
        args.crlf = false;
        args.null_data = true;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_null_data() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.null_data);

    let args = parse_low_raw(["--null-data"]).unwrap();
    assert_eq!(true, args.null_data);

    let args = parse_low_raw(["--null-data", "--crlf"]).unwrap();
    assert_eq!(false, args.null_data);
    assert_eq!(true, args.crlf);

    let args = parse_low_raw(["--crlf", "--null-data"]).unwrap();
    assert_eq!(true, args.null_data);
    assert_eq!(false, args.crlf);

    let args = parse_low_raw(["--null-data", "--no-crlf"]).unwrap();
    assert_eq!(true, args.null_data);
    assert_eq!(false, args.crlf);
}

/// --one-file-system
#[derive(Debug)]
struct OneFileSystem;

impl Flag for OneFileSystem {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "one-file-system"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-one-file-system")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-one-file-system-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-one-file-system-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.one_file_system = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_one_file_system() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.one_file_system);

    let args = parse_low_raw(["--one-file-system"]).unwrap();
    assert_eq!(true, args.one_file_system);

    let args =
        parse_low_raw(["--one-file-system", "--no-one-file-system"]).unwrap();
    assert_eq!(false, args.one_file_system);
}

/// -o/--only-matching
#[derive(Debug)]
struct OnlyMatching;

impl Flag for OnlyMatching {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'o')
    }
    fn name_long(&self) -> &'static str {
        "only-matching"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-only-matching-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-only-matching-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--only-matching does not have a negation");
        args.only_matching = true;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_only_matching() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.only_matching);

    let args = parse_low_raw(["--only-matching"]).unwrap();
    assert_eq!(true, args.only_matching);

    let args = parse_low_raw(["-o"]).unwrap();
    assert_eq!(true, args.only_matching);
}

/// --path-separator
#[derive(Debug)]
struct PathSeparator;

impl Flag for PathSeparator {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "path-separator"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("SEPARATOR")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-path-separator-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-path-separator-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let s = convert::string(v.unwrap_value())?;
        let raw = Vec::unescape_bytes(&s);
        args.path_separator = if raw.is_empty() {
            None
        } else if raw.len() == 1 {
            Some(raw[0])
        } else {
            anyhow::bail!(
                "{}",
                crate::i18n::t_args(
                    "err-path-separator",
                    &[("len", &raw.len().to_string()), ("sep", &s),],
                ),
            )
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_path_separator() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.path_separator);

    let args = parse_low_raw(["--path-separator", "/"]).unwrap();
    assert_eq!(Some(b'/'), args.path_separator);

    let args = parse_low_raw(["--path-separator", r"\"]).unwrap();
    assert_eq!(Some(b'\\'), args.path_separator);

    let args = parse_low_raw(["--path-separator", r"\x00"]).unwrap();
    assert_eq!(Some(0), args.path_separator);

    let args = parse_low_raw(["--path-separator", r"\0"]).unwrap();
    assert_eq!(Some(0), args.path_separator);

    let args = parse_low_raw(["--path-separator", "\x00"]).unwrap();
    assert_eq!(Some(0), args.path_separator);

    let args = parse_low_raw(["--path-separator", "\0"]).unwrap();
    assert_eq!(Some(0), args.path_separator);

    let args =
        parse_low_raw(["--path-separator", r"\x00", "--path-separator=/"])
            .unwrap();
    assert_eq!(Some(b'/'), args.path_separator);

    let result = parse_low_raw(["--path-separator", "foo"]);
    assert!(result.is_err(), "{result:?}");

    let result = parse_low_raw(["--path-separator", r"\\x00"]);
    assert!(result.is_err(), "{result:?}");
}

/// --passthru
#[derive(Debug)]
struct Passthru;

impl Flag for Passthru {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "passthru"
    }
    fn aliases(&self) -> &'static [&'static str] {
        &["passthrough"]
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-passthru-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-passthru-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--passthru has no negation");
        args.context = ContextMode::Passthru;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_passthru() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(ContextMode::default(), args.context);

    let args = parse_low_raw(["--passthru"]).unwrap();
    assert_eq!(ContextMode::Passthru, args.context);

    let args = parse_low_raw(["--passthrough"]).unwrap();
    assert_eq!(ContextMode::Passthru, args.context);
}

/// -P/--pcre2
#[derive(Debug)]
struct PCRE2;

impl Flag for PCRE2 {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'P')
    }
    fn name_long(&self) -> &'static str {
        "pcre2"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-pcre2")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-pcre2-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-pcre2-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.engine = if v.unwrap_switch() {
            EngineChoice::PCRE2
        } else {
            EngineChoice::Default
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_pcre2() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(EngineChoice::Default, args.engine);

    let args = parse_low_raw(["--pcre2"]).unwrap();
    assert_eq!(EngineChoice::PCRE2, args.engine);

    let args = parse_low_raw(["-P"]).unwrap();
    assert_eq!(EngineChoice::PCRE2, args.engine);

    let args = parse_low_raw(["-P", "--no-pcre2"]).unwrap();
    assert_eq!(EngineChoice::Default, args.engine);

    let args = parse_low_raw(["--engine=auto", "-P", "--no-pcre2"]).unwrap();
    assert_eq!(EngineChoice::Default, args.engine);

    let args = parse_low_raw(["-P", "--engine=auto"]).unwrap();
    assert_eq!(EngineChoice::Auto, args.engine);
}

/// --pcre2-version
#[derive(Debug)]
struct PCRE2Version;

impl Flag for PCRE2Version {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "pcre2-version"
    }
    fn doc_category(&self) -> Category {
        Category::OtherBehaviors
    }
    fn doc_short(&self) -> &'static str {
        "flag-pcre2version-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-pcre2version-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--pcre2-version has no negation");
        args.special = Some(SpecialMode::VersionPCRE2);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_pcre2_version() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.special);

    let args = parse_low_raw(["--pcre2-version"]).unwrap();
    assert_eq!(Some(SpecialMode::VersionPCRE2), args.special);
}

/// --pre
#[derive(Debug)]
struct Pre;

impl Flag for Pre {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "pre"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-pre")
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("COMMAND")
    }
    fn doc_category(&self) -> Category {
        Category::Input
    }
    fn doc_short(&self) -> &'static str {
        "flag-pre-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-pre-long"
    }
    fn completion_type(&self) -> CompletionType {
        CompletionType::Executable
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let path = match v {
            FlagValue::Value(v) => PathBuf::from(v),
            FlagValue::Switch(yes) => {
                assert!(!yes, "there is no affirmative switch for --pre");
                args.pre = None;
                return Ok(());
            }
        };
        args.pre = if path.as_os_str().is_empty() { None } else { Some(path) };
        if args.pre.is_some() {
            args.search_zip = false;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_pre() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.pre);

    let args = parse_low_raw(["--pre", "foo/bar"]).unwrap();
    assert_eq!(Some(PathBuf::from("foo/bar")), args.pre);

    let args = parse_low_raw(["--pre", ""]).unwrap();
    assert_eq!(None, args.pre);

    let args = parse_low_raw(["--pre", "foo/bar", "--pre", ""]).unwrap();
    assert_eq!(None, args.pre);

    let args = parse_low_raw(["--pre", "foo/bar", "--pre="]).unwrap();
    assert_eq!(None, args.pre);

    let args = parse_low_raw(["--pre", "foo/bar", "--no-pre"]).unwrap();
    assert_eq!(None, args.pre);
}

/// --pre-glob
#[derive(Debug)]
struct PreGlob;

impl Flag for PreGlob {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "pre-glob"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("GLOB")
    }
    fn doc_category(&self) -> Category {
        Category::Input
    }
    fn doc_short(&self) -> &'static str {
        "flag-pre-glob-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-pre-glob-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let glob = convert::string(v.unwrap_value())?;
        args.pre_glob.push(glob);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_pre_glob() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<String>::new(), args.pre_glob);

    let args = parse_low_raw(["--pre-glob", "*.pdf"]).unwrap();
    assert_eq!(vec!["*.pdf".to_string()], args.pre_glob);

    let args =
        parse_low_raw(["--pre-glob", "*.pdf", "--pre-glob=foo"]).unwrap();
    assert_eq!(vec!["*.pdf".to_string(), "foo".to_string()], args.pre_glob);
}

/// -p/--pretty
#[derive(Debug)]
struct Pretty;

impl Flag for Pretty {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'p')
    }
    fn name_long(&self) -> &'static str {
        "pretty"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-pretty-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-pretty-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--pretty has no negation");
        args.color = ColorChoice::Always;
        args.heading = Some(true);
        args.line_number = Some(true);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_pretty() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(ColorChoice::Auto, args.color);
    assert_eq!(None, args.heading);
    assert_eq!(None, args.line_number);

    let args = parse_low_raw(["--pretty"]).unwrap();
    assert_eq!(ColorChoice::Always, args.color);
    assert_eq!(Some(true), args.heading);
    assert_eq!(Some(true), args.line_number);

    let args = parse_low_raw(["-p"]).unwrap();
    assert_eq!(ColorChoice::Always, args.color);
    assert_eq!(Some(true), args.heading);
    assert_eq!(Some(true), args.line_number);
}

/// -q/--quiet
#[derive(Debug)]
struct Quiet;

impl Flag for Quiet {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'q')
    }
    fn name_long(&self) -> &'static str {
        "quiet"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-quiet-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-quiet-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--quiet has no negation");
        args.quiet = true;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_quiet() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.quiet);

    let args = parse_low_raw(["--quiet"]).unwrap();
    assert_eq!(true, args.quiet);

    let args = parse_low_raw(["-q"]).unwrap();
    assert_eq!(true, args.quiet);

    // flags like -l and --json cannot override -q, regardless of order
    let args = parse_low_raw(["-q", "--json"]).unwrap();
    assert_eq!(true, args.quiet);

    let args = parse_low_raw(["-q", "--files-with-matches"]).unwrap();
    assert_eq!(true, args.quiet);

    let args = parse_low_raw(["-q", "--files-without-match"]).unwrap();
    assert_eq!(true, args.quiet);

    let args = parse_low_raw(["-q", "--count"]).unwrap();
    assert_eq!(true, args.quiet);

    let args = parse_low_raw(["-q", "--count-matches"]).unwrap();
    assert_eq!(true, args.quiet);
}

/// --regex-size-limit
#[derive(Debug)]
struct RegexSizeLimit;

impl Flag for RegexSizeLimit {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "regex-size-limit"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM+SUFFIX?")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-regex-size-limit-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-regex-size-limit-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let v = v.unwrap_value();
        args.regex_size_limit = Some(convert::human_readable_usize(&v)?);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_regex_size_limit() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.regex_size_limit);

    #[cfg(target_pointer_width = "64")]
    {
        let args = parse_low_raw(["--regex-size-limit", "9G"]).unwrap();
        assert_eq!(Some(9 * (1 << 30)), args.regex_size_limit);

        let args = parse_low_raw(["--regex-size-limit=9G"]).unwrap();
        assert_eq!(Some(9 * (1 << 30)), args.regex_size_limit);

        let args =
            parse_low_raw(["--regex-size-limit=9G", "--regex-size-limit=0"])
                .unwrap();
        assert_eq!(Some(0), args.regex_size_limit);
    }

    let args = parse_low_raw(["--regex-size-limit=0K"]).unwrap();
    assert_eq!(Some(0), args.regex_size_limit);

    let args = parse_low_raw(["--regex-size-limit=0M"]).unwrap();
    assert_eq!(Some(0), args.regex_size_limit);

    let args = parse_low_raw(["--regex-size-limit=0G"]).unwrap();
    assert_eq!(Some(0), args.regex_size_limit);

    let result =
        parse_low_raw(["--regex-size-limit", "9999999999999999999999"]);
    assert!(result.is_err(), "{result:?}");

    let result = parse_low_raw(["--regex-size-limit", "9999999999999999G"]);
    assert!(result.is_err(), "{result:?}");
}

/// -e/--regexp
#[derive(Debug)]
struct Regexp;

impl Flag for Regexp {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'e')
    }
    fn name_long(&self) -> &'static str {
        "regexp"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("PATTERN")
    }
    fn doc_category(&self) -> Category {
        Category::Input
    }
    fn doc_short(&self) -> &'static str {
        "flag-regexp-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-regexp-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let regexp = convert::string(v.unwrap_value())?;
        args.patterns.push(PatternSource::Regexp(regexp));
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_regexp() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<PatternSource>::new(), args.patterns);

    let args = parse_low_raw(["--regexp", "foo"]).unwrap();
    assert_eq!(vec![PatternSource::Regexp("foo".to_string())], args.patterns);

    let args = parse_low_raw(["--regexp=foo"]).unwrap();
    assert_eq!(vec![PatternSource::Regexp("foo".to_string())], args.patterns);

    let args = parse_low_raw(["-e", "foo"]).unwrap();
    assert_eq!(vec![PatternSource::Regexp("foo".to_string())], args.patterns);

    let args = parse_low_raw(["-efoo"]).unwrap();
    assert_eq!(vec![PatternSource::Regexp("foo".to_string())], args.patterns);

    let args = parse_low_raw(["--regexp", "-foo"]).unwrap();
    assert_eq!(vec![PatternSource::Regexp("-foo".to_string())], args.patterns);

    let args = parse_low_raw(["--regexp=-foo"]).unwrap();
    assert_eq!(vec![PatternSource::Regexp("-foo".to_string())], args.patterns);

    let args = parse_low_raw(["-e", "-foo"]).unwrap();
    assert_eq!(vec![PatternSource::Regexp("-foo".to_string())], args.patterns);

    let args = parse_low_raw(["-e-foo"]).unwrap();
    assert_eq!(vec![PatternSource::Regexp("-foo".to_string())], args.patterns);

    let args = parse_low_raw(["--regexp=foo", "--regexp", "bar"]).unwrap();
    assert_eq!(
        vec![
            PatternSource::Regexp("foo".to_string()),
            PatternSource::Regexp("bar".to_string())
        ],
        args.patterns
    );

    // While we support invalid UTF-8 arguments in general, patterns must be
    // valid UTF-8.
    #[cfg(unix)]
    {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

        let bytes = &[b'A', 0xFF, b'Z'][..];
        let result = parse_low_raw([
            OsStr::from_bytes(b"-e"),
            OsStr::from_bytes(bytes),
        ]);
        assert!(result.is_err(), "{result:?}");
    }

    // Check that combining -e/--regexp and -f/--file works as expected.
    let args = parse_low_raw(["-efoo", "-fbar"]).unwrap();
    assert_eq!(
        vec![
            PatternSource::Regexp("foo".to_string()),
            PatternSource::File(PathBuf::from("bar"))
        ],
        args.patterns
    );

    let args = parse_low_raw(["-efoo", "-fbar", "-equux"]).unwrap();
    assert_eq!(
        vec![
            PatternSource::Regexp("foo".to_string()),
            PatternSource::File(PathBuf::from("bar")),
            PatternSource::Regexp("quux".to_string()),
        ],
        args.patterns
    );
}

/// -r/--replace
#[derive(Debug)]
struct Replace;

impl Flag for Replace {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'r')
    }
    fn name_long(&self) -> &'static str {
        "replace"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("REPLACEMENT")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-replace-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-replace-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.replace = Some(convert::string(v.unwrap_value())?.into());
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_replace() {
    use bstr::BString;

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.replace);

    let args = parse_low_raw(["--replace", "foo"]).unwrap();
    assert_eq!(Some(BString::from("foo")), args.replace);

    let args = parse_low_raw(["--replace", "-foo"]).unwrap();
    assert_eq!(Some(BString::from("-foo")), args.replace);

    let args = parse_low_raw(["-r", "foo"]).unwrap();
    assert_eq!(Some(BString::from("foo")), args.replace);

    let args = parse_low_raw(["-r", "foo", "-rbar"]).unwrap();
    assert_eq!(Some(BString::from("bar")), args.replace);

    let args = parse_low_raw(["-r", "foo", "-r", ""]).unwrap();
    assert_eq!(Some(BString::from("")), args.replace);
}

/// -z/--search-zip
#[derive(Debug)]
struct SearchZip;

impl Flag for SearchZip {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'z')
    }
    fn name_long(&self) -> &'static str {
        "search-zip"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-search-zip")
    }
    fn doc_category(&self) -> Category {
        Category::Input
    }
    fn doc_short(&self) -> &'static str {
        "flag-search-zip-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-search-zip-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.search_zip = if v.unwrap_switch() {
            args.pre = None;
            true
        } else {
            false
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_search_zip() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.search_zip);

    let args = parse_low_raw(["--search-zip"]).unwrap();
    assert_eq!(true, args.search_zip);

    let args = parse_low_raw(["-z"]).unwrap();
    assert_eq!(true, args.search_zip);

    let args = parse_low_raw(["-z", "--no-search-zip"]).unwrap();
    assert_eq!(false, args.search_zip);

    let args = parse_low_raw(["--pre=foo", "--no-search-zip"]).unwrap();
    assert_eq!(Some(PathBuf::from("foo")), args.pre);
    assert_eq!(false, args.search_zip);

    let args = parse_low_raw(["--pre=foo", "--search-zip"]).unwrap();
    assert_eq!(None, args.pre);
    assert_eq!(true, args.search_zip);

    let args = parse_low_raw(["--pre=foo", "-z", "--no-search-zip"]).unwrap();
    assert_eq!(None, args.pre);
    assert_eq!(false, args.search_zip);
}

/// -S/--smart-case
#[derive(Debug)]
struct SmartCase;

impl Flag for SmartCase {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'S')
    }
    fn name_long(&self) -> &'static str {
        "smart-case"
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-smart-case-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-smart-case-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--smart-case flag has no negation");
        args.case = CaseMode::Smart;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_smart_case() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(CaseMode::Sensitive, args.case);

    let args = parse_low_raw(["--smart-case"]).unwrap();
    assert_eq!(CaseMode::Smart, args.case);

    let args = parse_low_raw(["-S"]).unwrap();
    assert_eq!(CaseMode::Smart, args.case);

    let args = parse_low_raw(["-S", "-s"]).unwrap();
    assert_eq!(CaseMode::Sensitive, args.case);

    let args = parse_low_raw(["-S", "-i"]).unwrap();
    assert_eq!(CaseMode::Insensitive, args.case);

    let args = parse_low_raw(["-s", "-S"]).unwrap();
    assert_eq!(CaseMode::Smart, args.case);

    let args = parse_low_raw(["-i", "-S"]).unwrap();
    assert_eq!(CaseMode::Smart, args.case);
}

/// --sort-files
#[derive(Debug)]
struct SortFiles;

impl Flag for SortFiles {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "sort-files"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-sort-files")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-sort-files-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-sort-files-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.sort = if v.unwrap_switch() {
            Some(SortMode { reverse: false, kind: SortModeKind::Path })
        } else {
            None
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_sort_files() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.sort);

    let args = parse_low_raw(["--sort-files"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: false, kind: SortModeKind::Path }),
        args.sort
    );

    let args = parse_low_raw(["--sort-files", "--no-sort-files"]).unwrap();
    assert_eq!(None, args.sort);

    let args = parse_low_raw(["--sort", "created", "--sort-files"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: false, kind: SortModeKind::Path }),
        args.sort
    );

    let args = parse_low_raw(["--sort-files", "--sort", "created"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: false, kind: SortModeKind::Created }),
        args.sort
    );

    let args = parse_low_raw(["--sortr", "created", "--sort-files"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: false, kind: SortModeKind::Path }),
        args.sort
    );

    let args = parse_low_raw(["--sort-files", "--sortr", "created"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: true, kind: SortModeKind::Created }),
        args.sort
    );

    let args = parse_low_raw(["--sort=path", "--no-sort-files"]).unwrap();
    assert_eq!(None, args.sort);

    let args = parse_low_raw(["--sortr=path", "--no-sort-files"]).unwrap();
    assert_eq!(None, args.sort);
}

/// --sort
#[derive(Debug)]
struct Sort;

impl Flag for Sort {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "sort"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("SORTBY")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-sort-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-sort-long"
    }
    fn doc_choices(&self) -> &'static [&'static str] {
        &["none", "path", "modified", "accessed", "created"]
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let kind = match convert::str(&v.unwrap_value())? {
            "none" => {
                args.sort = None;
                return Ok(());
            }
            "path" => SortModeKind::Path,
            "modified" => SortModeKind::LastModified,
            "accessed" => SortModeKind::LastAccessed,
            "created" => SortModeKind::Created,
            unk => anyhow::bail!(
                "{}",
                crate::i18n::t_args(
                    "err-choice-unrecognized",
                    &[("choice", unk)]
                )
            ),
        };
        args.sort = Some(SortMode { reverse: false, kind });
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_sort() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.sort);

    let args = parse_low_raw(["--sort", "path"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: false, kind: SortModeKind::Path }),
        args.sort
    );

    let args = parse_low_raw(["--sort", "path", "--sort=created"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: false, kind: SortModeKind::Created }),
        args.sort
    );

    let args = parse_low_raw(["--sort=none"]).unwrap();
    assert_eq!(None, args.sort);

    let args = parse_low_raw(["--sort", "path", "--sort=none"]).unwrap();
    assert_eq!(None, args.sort);
}

/// --sortr
#[derive(Debug)]
struct Sortr;

impl Flag for Sortr {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "sortr"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("SORTBY")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-sortr-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-sortr-long"
    }
    fn doc_choices(&self) -> &'static [&'static str] {
        &["none", "path", "modified", "accessed", "created"]
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let kind = match convert::str(&v.unwrap_value())? {
            "none" => {
                args.sort = None;
                return Ok(());
            }
            "path" => SortModeKind::Path,
            "modified" => SortModeKind::LastModified,
            "accessed" => SortModeKind::LastAccessed,
            "created" => SortModeKind::Created,
            unk => anyhow::bail!(
                "{}",
                crate::i18n::t_args(
                    "err-choice-unrecognized",
                    &[("choice", unk)]
                )
            ),
        };
        args.sort = Some(SortMode { reverse: true, kind });
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_sortr() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.sort);

    let args = parse_low_raw(["--sortr", "path"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: true, kind: SortModeKind::Path }),
        args.sort
    );

    let args = parse_low_raw(["--sortr", "path", "--sortr=created"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: true, kind: SortModeKind::Created }),
        args.sort
    );

    let args = parse_low_raw(["--sortr=none"]).unwrap();
    assert_eq!(None, args.sort);

    let args = parse_low_raw(["--sortr", "path", "--sortr=none"]).unwrap();
    assert_eq!(None, args.sort);

    let args = parse_low_raw(["--sort=path", "--sortr=path"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: true, kind: SortModeKind::Path }),
        args.sort
    );

    let args = parse_low_raw(["--sortr=path", "--sort=path"]).unwrap();
    assert_eq!(
        Some(SortMode { reverse: false, kind: SortModeKind::Path }),
        args.sort
    );
}

/// --stats
#[derive(Debug)]
struct Stats;

impl Flag for Stats {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "stats"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-stats")
    }
    fn doc_category(&self) -> Category {
        Category::Logging
    }
    fn doc_short(&self) -> &'static str {
        "flag-stats-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-stats-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.stats = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_stats() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.stats);

    let args = parse_low_raw(["--stats"]).unwrap();
    assert_eq!(true, args.stats);

    let args = parse_low_raw(["--stats", "--no-stats"]).unwrap();
    assert_eq!(false, args.stats);
}

/// --stop-on-nonmatch
#[derive(Debug)]
struct StopOnNonmatch;

impl Flag for StopOnNonmatch {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "stop-on-nonmatch"
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-stop-on-nonmatch-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-stop-on-nonmatch-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--stop-on-nonmatch has no negation");
        args.stop_on_nonmatch = true;
        args.multiline = false;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_stop_on_nonmatch() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.stop_on_nonmatch);

    let args = parse_low_raw(["--stop-on-nonmatch"]).unwrap();
    assert_eq!(true, args.stop_on_nonmatch);

    let args = parse_low_raw(["--stop-on-nonmatch", "-U"]).unwrap();
    assert_eq!(true, args.multiline);
    assert_eq!(false, args.stop_on_nonmatch);

    let args = parse_low_raw(["-U", "--stop-on-nonmatch"]).unwrap();
    assert_eq!(false, args.multiline);
    assert_eq!(true, args.stop_on_nonmatch);

    let args =
        parse_low_raw(["--stop-on-nonmatch", "--no-multiline"]).unwrap();
    assert_eq!(false, args.multiline);
    assert_eq!(true, args.stop_on_nonmatch);
}

/// -a/--text
#[derive(Debug)]
struct Text;

impl Flag for Text {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'a')
    }
    fn name_long(&self) -> &'static str {
        "text"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-text")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-text-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-text-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.binary = if v.unwrap_switch() {
            BinaryMode::AsText
        } else {
            BinaryMode::Auto
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_text() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(BinaryMode::Auto, args.binary);

    let args = parse_low_raw(["--text"]).unwrap();
    assert_eq!(BinaryMode::AsText, args.binary);

    let args = parse_low_raw(["-a"]).unwrap();
    assert_eq!(BinaryMode::AsText, args.binary);

    let args = parse_low_raw(["-a", "--no-text"]).unwrap();
    assert_eq!(BinaryMode::Auto, args.binary);

    let args = parse_low_raw(["-a", "--binary"]).unwrap();
    assert_eq!(BinaryMode::SearchAndSuppress, args.binary);

    let args = parse_low_raw(["--binary", "-a"]).unwrap();
    assert_eq!(BinaryMode::AsText, args.binary);

    let args = parse_low_raw(["-a", "--no-binary"]).unwrap();
    assert_eq!(BinaryMode::Auto, args.binary);

    let args = parse_low_raw(["--binary", "--no-text"]).unwrap();
    assert_eq!(BinaryMode::Auto, args.binary);
}

/// -j/--threads
#[derive(Debug)]
struct Threads;

impl Flag for Threads {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'j')
    }
    fn name_long(&self) -> &'static str {
        "threads"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM")
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-threads-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-threads-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let threads = convert::usize(&v.unwrap_value())?;
        args.threads = if threads == 0 { None } else { Some(threads) };
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_threads() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.threads);

    let args = parse_low_raw(["--threads", "5"]).unwrap();
    assert_eq!(Some(5), args.threads);

    let args = parse_low_raw(["-j", "5"]).unwrap();
    assert_eq!(Some(5), args.threads);

    let args = parse_low_raw(["-j5"]).unwrap();
    assert_eq!(Some(5), args.threads);

    let args = parse_low_raw(["-j5", "-j10"]).unwrap();
    assert_eq!(Some(10), args.threads);

    let args = parse_low_raw(["-j5", "-j0"]).unwrap();
    assert_eq!(None, args.threads);
}

/// --trace
#[derive(Debug)]
struct Trace;

impl Flag for Trace {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "trace"
    }
    fn doc_category(&self) -> Category {
        Category::Logging
    }
    fn doc_short(&self) -> &'static str {
        "flag-trace-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-trace-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--trace can only be enabled");
        args.logging = Some(LoggingMode::Trace);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_trace() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.logging);

    let args = parse_low_raw(["--trace"]).unwrap();
    assert_eq!(Some(LoggingMode::Trace), args.logging);

    let args = parse_low_raw(["--debug", "--trace"]).unwrap();
    assert_eq!(Some(LoggingMode::Trace), args.logging);
}

/// --trim
#[derive(Debug)]
struct Trim;

impl Flag for Trim {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "trim"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-trim")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-trim-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-trim-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.trim = v.unwrap_switch();
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_trim() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.trim);

    let args = parse_low_raw(["--trim"]).unwrap();
    assert_eq!(true, args.trim);

    let args = parse_low_raw(["--trim", "--no-trim"]).unwrap();
    assert_eq!(false, args.trim);
}

/// -t/--type
#[derive(Debug)]
struct Type;

impl Flag for Type {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b't')
    }
    fn name_long(&self) -> &'static str {
        "type"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("TYPE")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-type-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-type-long"
    }
    fn completion_type(&self) -> CompletionType {
        CompletionType::Filetype
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.type_changes.push(TypeChange::Select {
            name: convert::string(v.unwrap_value())?,
        });
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_type() {
    let select = |name: &str| TypeChange::Select { name: name.to_string() };

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<TypeChange>::new(), args.type_changes);

    let args = parse_low_raw(["--type", "rust"]).unwrap();
    assert_eq!(vec![select("rust")], args.type_changes);

    let args = parse_low_raw(["-t", "rust"]).unwrap();
    assert_eq!(vec![select("rust")], args.type_changes);

    let args = parse_low_raw(["-trust"]).unwrap();
    assert_eq!(vec![select("rust")], args.type_changes);

    let args = parse_low_raw(["-trust", "-tpython"]).unwrap();
    assert_eq!(vec![select("rust"), select("python")], args.type_changes);

    let args = parse_low_raw(["-tabcdefxyz"]).unwrap();
    assert_eq!(vec![select("abcdefxyz")], args.type_changes);
}

/// --type-add
#[derive(Debug)]
struct TypeAdd;

impl Flag for TypeAdd {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "type-add"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("TYPESPEC")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-type-add-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-type-add-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.type_changes
            .push(TypeChange::Add { def: convert::string(v.unwrap_value())? });
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_type_add() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<TypeChange>::new(), args.type_changes);

    let args = parse_low_raw(["--type-add", "foo"]).unwrap();
    assert_eq!(
        vec![TypeChange::Add { def: "foo".to_string() }],
        args.type_changes
    );

    let args = parse_low_raw(["--type-add", "foo", "--type-add=bar"]).unwrap();
    assert_eq!(
        vec![
            TypeChange::Add { def: "foo".to_string() },
            TypeChange::Add { def: "bar".to_string() }
        ],
        args.type_changes
    );
}

/// --type-clear
#[derive(Debug)]
struct TypeClear;

impl Flag for TypeClear {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_long(&self) -> &'static str {
        "type-clear"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("TYPE")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-type-clear-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-type-clear-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.type_changes.push(TypeChange::Clear {
            name: convert::string(v.unwrap_value())?,
        });
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_type_clear() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<TypeChange>::new(), args.type_changes);

    let args = parse_low_raw(["--type-clear", "foo"]).unwrap();
    assert_eq!(
        vec![TypeChange::Clear { name: "foo".to_string() }],
        args.type_changes
    );

    let args =
        parse_low_raw(["--type-clear", "foo", "--type-clear=bar"]).unwrap();
    assert_eq!(
        vec![
            TypeChange::Clear { name: "foo".to_string() },
            TypeChange::Clear { name: "bar".to_string() }
        ],
        args.type_changes
    );
}

/// --type-not
#[derive(Debug)]
struct TypeNot;

impl Flag for TypeNot {
    fn is_switch(&self) -> bool {
        false
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'T')
    }
    fn name_long(&self) -> &'static str {
        "type-not"
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("TYPE")
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-type-not-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-type-not-long"
    }
    fn completion_type(&self) -> CompletionType {
        CompletionType::Filetype
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.type_changes.push(TypeChange::Negate {
            name: convert::string(v.unwrap_value())?,
        });
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_type_not() {
    let select = |name: &str| TypeChange::Select { name: name.to_string() };
    let negate = |name: &str| TypeChange::Negate { name: name.to_string() };

    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Vec::<TypeChange>::new(), args.type_changes);

    let args = parse_low_raw(["--type-not", "rust"]).unwrap();
    assert_eq!(vec![negate("rust")], args.type_changes);

    let args = parse_low_raw(["-T", "rust"]).unwrap();
    assert_eq!(vec![negate("rust")], args.type_changes);

    let args = parse_low_raw(["-Trust"]).unwrap();
    assert_eq!(vec![negate("rust")], args.type_changes);

    let args = parse_low_raw(["-Trust", "-Tpython"]).unwrap();
    assert_eq!(vec![negate("rust"), negate("python")], args.type_changes);

    let args = parse_low_raw(["-Tabcdefxyz"]).unwrap();
    assert_eq!(vec![negate("abcdefxyz")], args.type_changes);

    let args = parse_low_raw(["-Trust", "-ttoml", "-Tjson"]).unwrap();
    assert_eq!(
        vec![negate("rust"), select("toml"), negate("json")],
        args.type_changes
    );
}

/// --type-list
#[derive(Debug)]
struct TypeList;

impl Flag for TypeList {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "type-list"
    }
    fn doc_category(&self) -> Category {
        Category::OtherBehaviors
    }
    fn doc_short(&self) -> &'static str {
        "flag-type-list-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-type-list-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--type-list has no negation");
        args.mode.update(Mode::Types);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_type_list() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(Mode::Search(SearchMode::Standard), args.mode);

    let args = parse_low_raw(["--type-list"]).unwrap();
    assert_eq!(Mode::Types, args.mode);
}

/// -u/--unrestricted
#[derive(Debug)]
struct Unrestricted;

impl Flag for Unrestricted {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'u')
    }
    fn name_long(&self) -> &'static str {
        "unrestricted"
    }
    fn doc_category(&self) -> Category {
        Category::Filter
    }
    fn doc_short(&self) -> &'static str {
        "flag-unrestricted-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-unrestricted-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--unrestricted has no negation");
        args.unrestricted = args.unrestricted.saturating_add(1);
        anyhow::ensure!(
            args.unrestricted <= 3,
            "{}",
            crate::i18n::t("err-flag-repeated-3-times")
        );
        if args.unrestricted == 1 {
            NoIgnore.update(FlagValue::Switch(true), args)?;
        } else if args.unrestricted == 2 {
            Hidden.update(FlagValue::Switch(true), args)?;
        } else {
            assert_eq!(args.unrestricted, 3);
            Binary.update(FlagValue::Switch(true), args)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_unrestricted() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.no_ignore_vcs);
    assert_eq!(false, args.hidden);
    assert_eq!(BinaryMode::Auto, args.binary);

    let args = parse_low_raw(["--unrestricted"]).unwrap();
    assert_eq!(true, args.no_ignore_vcs);
    assert_eq!(false, args.hidden);
    assert_eq!(BinaryMode::Auto, args.binary);

    let args = parse_low_raw(["--unrestricted", "-u"]).unwrap();
    assert_eq!(true, args.no_ignore_vcs);
    assert_eq!(true, args.hidden);
    assert_eq!(BinaryMode::Auto, args.binary);

    let args = parse_low_raw(["-uuu"]).unwrap();
    assert_eq!(true, args.no_ignore_vcs);
    assert_eq!(true, args.hidden);
    assert_eq!(BinaryMode::SearchAndSuppress, args.binary);

    let result = parse_low_raw(["-uuuu"]);
    assert!(result.is_err(), "{result:?}");
}

/// --version
#[derive(Debug)]
struct Version;

impl Flag for Version {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'V')
    }
    fn name_long(&self) -> &'static str {
        "version"
    }
    fn doc_category(&self) -> Category {
        Category::OtherBehaviors
    }
    fn doc_short(&self) -> &'static str {
        "flag-version-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-version-long"
    }

    fn update(&self, v: FlagValue, _: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--version has no negation");
        // Since this flag has different semantics for -V and --version and the
        // Flag trait doesn't support encoding this sort of thing, we handle it
        // as a special case in the parser.
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_version() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.special);

    let args = parse_low_raw(["-V"]).unwrap();
    assert_eq!(Some(SpecialMode::VersionShort), args.special);

    let args = parse_low_raw(["--version"]).unwrap();
    assert_eq!(Some(SpecialMode::VersionLong), args.special);

    let args = parse_low_raw(["-V", "--version"]).unwrap();
    assert_eq!(Some(SpecialMode::VersionLong), args.special);

    let args = parse_low_raw(["--version", "-V"]).unwrap();
    assert_eq!(Some(SpecialMode::VersionShort), args.special);
}

/// --vimgrep
#[derive(Debug)]
struct Vimgrep;

impl Flag for Vimgrep {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_long(&self) -> &'static str {
        "vimgrep"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-vimgrep-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-vimgrep-long"
    }
    fn doc_choices(&self) -> &'static [&'static str] {
        &[]
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--vimgrep has no negation");
        args.vimgrep = true;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_vimgrep() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(false, args.vimgrep);

    let args = parse_low_raw(["--vimgrep"]).unwrap();
    assert_eq!(true, args.vimgrep);
}

/// --with-filename
#[derive(Debug)]
struct WithFilename;

impl Flag for WithFilename {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'H')
    }
    fn name_long(&self) -> &'static str {
        "with-filename"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-with-filename-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-with-filename-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--with-filename has no defined negation");
        args.with_filename = Some(true);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_with_filename() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.with_filename);

    let args = parse_low_raw(["--with-filename"]).unwrap();
    assert_eq!(Some(true), args.with_filename);

    let args = parse_low_raw(["-H"]).unwrap();
    assert_eq!(Some(true), args.with_filename);
}

/// --no-filename
#[derive(Debug)]
struct WithFilenameNo;

impl Flag for WithFilenameNo {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'I')
    }
    fn name_long(&self) -> &'static str {
        "no-filename"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "flag-with-filename-no-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-with-filename-no-long"
    }
    fn doc_choices(&self) -> &'static [&'static str] {
        &[]
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--no-filename has no defined negation");
        args.with_filename = Some(false);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_with_filename_no() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.with_filename);

    let args = parse_low_raw(["--no-filename"]).unwrap();
    assert_eq!(Some(false), args.with_filename);

    let args = parse_low_raw(["-I"]).unwrap();
    assert_eq!(Some(false), args.with_filename);

    let args = parse_low_raw(["-I", "-H"]).unwrap();
    assert_eq!(Some(true), args.with_filename);

    let args = parse_low_raw(["-H", "-I"]).unwrap();
    assert_eq!(Some(false), args.with_filename);
}

/// -w/--word-regexp
#[derive(Debug)]
struct WordRegexp;

impl Flag for WordRegexp {
    fn is_switch(&self) -> bool {
        true
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'w')
    }
    fn name_long(&self) -> &'static str {
        "word-regexp"
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        "flag-word-regexp-short"
    }
    fn doc_long(&self) -> &'static str {
        "flag-word-regexp-long"
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--word-regexp has no negation");
        args.boundary = Some(BoundaryMode::Word);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_word_regexp() {
    let args = parse_low_raw(None::<&str>).unwrap();
    assert_eq!(None, args.boundary);

    let args = parse_low_raw(["--word-regexp"]).unwrap();
    assert_eq!(Some(BoundaryMode::Word), args.boundary);

    let args = parse_low_raw(["-w"]).unwrap();
    assert_eq!(Some(BoundaryMode::Word), args.boundary);

    let args = parse_low_raw(["-x", "-w"]).unwrap();
    assert_eq!(Some(BoundaryMode::Word), args.boundary);

    let args = parse_low_raw(["-w", "-x"]).unwrap();
    assert_eq!(Some(BoundaryMode::Line), args.boundary);
}

fn check_indexing_allowed() -> anyhow::Result<()> {
    if cfg!(feature = "unstable-index") {
        return Ok(());
    }
    anyhow::bail!("{}", crate::i18n::t("indexing-not-supported"))
}

mod convert {
    use std::ffi::{OsStr, OsString};

    use anyhow::Context;

    pub(super) fn str(v: &OsStr) -> anyhow::Result<&str> {
        let Some(s) = v.to_str() else {
            anyhow::bail!("{}", crate::i18n::t("err-value-not-utf8"))
        };
        Ok(s)
    }

    pub(super) fn string(v: OsString) -> anyhow::Result<String> {
        let Ok(s) = v.into_string() else {
            anyhow::bail!("{}", crate::i18n::t("err-value-not-utf8"))
        };
        Ok(s)
    }

    pub(super) fn usize(v: &OsStr) -> anyhow::Result<usize> {
        str(v)?.parse().context(crate::i18n::t("err-value-not-number"))
    }

    pub(super) fn u64(v: &OsStr) -> anyhow::Result<u64> {
        str(v)?.parse().context(crate::i18n::t("err-value-not-number"))
    }

    pub(super) fn human_readable_u64(v: &OsStr) -> anyhow::Result<u64> {
        grep::cli::parse_human_readable_size(str(v)?)
            .context(crate::i18n::t("err-invalid-size"))
    }

    pub(super) fn human_readable_usize(v: &OsStr) -> anyhow::Result<usize> {
        let size = human_readable_u64(v)?;
        let Ok(size) = usize::try_from(size) else {
            anyhow::bail!("{}", crate::i18n::t("err-size-too-big"))
        };
        Ok(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available_shorts() {
        let mut total = vec![false; 128];
        for byte in 0..=0x7F {
            match byte {
                b'.' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => {
                    total[usize::from(byte)] = true
                }
                _ => continue,
            }
        }

        let mut taken = vec![false; 128];
        for flag in FLAGS.iter() {
            let Some(short) = flag.name_short() else { continue };
            taken[usize::from(short)] = true;
        }

        for byte in 0..=0x7F {
            if total[usize::from(byte)] && !taken[usize::from(byte)] {
                eprintln!("{}", char::from(byte));
            }
        }
    }

    #[test]
    fn shorts_all_ascii_alphanumeric() {
        for flag in FLAGS.iter() {
            let Some(byte) = flag.name_short() else { continue };
            let long = flag.name_long();
            assert!(
                byte.is_ascii_alphanumeric() || byte == b'.',
                "\\x{byte:0X} is not a valid short flag for {long}",
            )
        }
    }

    #[test]
    fn longs_all_ascii_alphanumeric() {
        for flag in FLAGS.iter() {
            let long = flag.name_long();
            let count = long.chars().count();
            assert!(count >= 2, "flag '{long}' is less than 2 characters");
            assert!(
                long.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'),
                "flag '{long}' does not match ^[-0-9A-Za-z]+$",
            );
            for alias in flag.aliases() {
                let count = alias.chars().count();
                assert!(
                    count >= 2,
                    "flag '{long}' has alias '{alias}' that is \
                     less than 2 characters",
                );
                assert!(
                    alias
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-'),
                    "flag '{long}' has alias '{alias}' that does not \
                     match ^[-0-9A-Za-z]+$",
                );
            }
            let Some(negated) = flag.name_negated() else { continue };
            let count = negated.chars().count();
            assert!(
                count >= 2,
                "flag '{long}' has negation '{negated}' that is \
                 less than 2 characters",
            );
            assert!(
                negated.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'),
                "flag '{long}' has negation '{negated}' that \
                 does not match ^[-0-9A-Za-z]+$",
            );
        }
    }

    #[test]
    fn shorts_no_duplicates() {
        let mut taken = vec![false; 128];
        for flag in FLAGS.iter() {
            let Some(short) = flag.name_short() else { continue };
            let long = flag.name_long();
            assert!(
                !taken[usize::from(short)],
                "flag {long} has duplicate short flag {}",
                char::from(short)
            );
            taken[usize::from(short)] = true;
        }
    }

    #[test]
    fn longs_no_duplicates() {
        use std::collections::BTreeSet;

        let mut taken = BTreeSet::new();
        for flag in FLAGS.iter() {
            let long = flag.name_long();
            assert!(taken.insert(long), "flag {long} has a duplicate name");
            for alias in flag.aliases() {
                assert!(
                    taken.insert(alias),
                    "flag {long} has an alias {alias} that is duplicative"
                );
            }
            let Some(negated) = flag.name_negated() else { continue };
            assert!(
                taken.insert(negated),
                "negated flag {negated} has a duplicate name"
            );
        }
    }

    #[test]
    fn non_switches_have_variable_names() {
        for flag in FLAGS.iter() {
            if flag.is_switch() {
                continue;
            }
            let long = flag.name_long();
            assert!(
                flag.doc_variable().is_some(),
                "flag '{long}' should have a variable name"
            );
        }
    }

    #[test]
    fn switches_have_no_choices() {
        for flag in FLAGS.iter() {
            if !flag.is_switch() {
                continue;
            }
            let long = flag.name_long();
            let choices = flag.doc_choices();
            assert!(
                choices.is_empty(),
                "switch flag '{long}' \
                 should not have any choices but has some: {choices:?}",
            );
        }
    }

    #[test]
    fn choices_ascii_alphanumeric() {
        for flag in FLAGS.iter() {
            let long = flag.name_long();
            for choice in flag.doc_choices() {
                assert!(
                    choice.chars().all(|c| c.is_ascii_alphanumeric()
                        || c == '-'
                        || c == ':'
                        || c == '+'),
                    "choice '{choice}' for flag '{long}' does not match \
                     ^[-+:0-9A-Za-z]+$",
                )
            }
        }
    }
}
