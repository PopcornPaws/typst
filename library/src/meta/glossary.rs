use crate::prelude::*;

/// A glossary of acronyms, etc. TODO
/// Display: Glossary
/// Category: meta
#[element(Locatable, Synthesize, Show, Finalize, LocalName)]
pub struct GlossaryElem {
    #[required]
    #[parse(
        let Spanned { v: mut path, span } =
            args.expect::<Spanned<BibPaths>>("path to bibliography file")?;
        for path in &mut paths.0 {
            // resolve paths
            *path = vm.locate(path).at(span)?.to_string_lossy().into();
        }
        // check that parsing works
        let _ = load(vm.world(), &paths).at(span)?;
        paths
    )]
    pub path: GlossaryPath,
    #[default(Some(Smart::Auto))]
    pub title: Option<Smart<Content>>,
}

pub struct GlossaryPath(EcoString);
