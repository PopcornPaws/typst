use crate::prelude::*;

/// A glossary of acronyms, etc.
/// Display: Glossary
/// Category: meta
#[element(Locatable, Synthesize, Show, Finalize, LocalName)]
pub struct GlossaryElem {
    #[required]
    #[parse(
        let Spanned { v: mut path, span } =
            args.expect::<Spanned<GlossaryPath>>("path to glossary file")?;
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
    // TODO style
}

pub struct GlossaryPath(EcoString);

impl GlossaryElem {
    pub fn find(introspector: Tracked<Introspector>) -> StrResult<Self> {
        let mut iter = introspector.query(&Self::func().select()).into_iter();
        let Some(elem) = iter.next() else {
            return Err("the document does not contain a bibliography".into());
        }
    }
}
