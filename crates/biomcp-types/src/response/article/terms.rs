//! Plain meanings of article license codes. Creative Commons codes follow each
//! license's own deed at creativecommons.org; `tdm`, `public_domain` and `other`
//! follow PMC's Copyright Notice and its PMC Article Datasets page.
use crate::{License, Text};

/// What a license code allows, in plain words from the license's own source.
pub fn license_terms(license: License) -> Option<Text> {
    Text::new(match license {
        License::CcBy => {
            "Creative Commons Attribution: you may share and adapt it for any purpose, even \
             commercially, if you give credit, link the license and say what you changed."
        }
        License::CcBySa => {
            "Creative Commons Attribution-ShareAlike: you may share and adapt it for any \
             purpose, even commercially, with credit; adaptations you share must use the \
             same license."
        }
        License::CcByNd => {
            "Creative Commons Attribution-NoDerivatives: you may share it for any purpose, \
             even commercially, with credit, but you may not share a modified version."
        }
        License::CcByNc => {
            "Creative Commons Attribution-NonCommercial: you may share and adapt it with \
             credit, but not for commercial purposes."
        }
        License::CcByNcSa => {
            "Creative Commons Attribution-NonCommercial-ShareAlike: you may share and adapt \
             it with credit, not for commercial purposes; adaptations you share must use the \
             same license."
        }
        License::CcByNcNd => {
            "Creative Commons Attribution-NonCommercial-NoDerivatives: you may share it with \
             credit, not for commercial purposes, and you may not share a modified version."
        }
        License::Cc0 => {
            "CC0 public-domain dedication: you may copy, modify and share it, even \
             commercially, without asking permission."
        }
        License::PublicDomain => {
            "Public domain: you may copy, modify and share it, even commercially, without \
             asking permission; PMC notes that figures may still belong to other copyright \
             holders."
        }
        License::Tdm => {
            "PMC author manuscript terms: available for text mining and for uses consistent \
             with fair use under copyright law. This is not an open license: sharing or \
             reproducing it beyond that needs the copyright owner's permission."
        }
        License::Other => {
            "No Creative Commons license recognized. Free to read is not free to reuse: \
             assume standard copyright applies unless the article's own license statement \
             says otherwise, and reuse beyond fair use needs the copyright owner's permission."
        }
    })
    .ok()
}
