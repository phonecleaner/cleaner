/****************************************************************************
**
** svgcleaner could help you to clean up your SVG files
** from unnecessary data.
** Copyright (C) 2012-2016 Evgeniy Reizner
**
** This program is free software; you can redistribute it and/or modify
** it under the terms of the GNU General Public License as published by
** the Free Software Foundation; either version 2 of the License, or
** (at your option) any later version.
**
** This program is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
** GNU General Public License for more details.
**
** You should have received a copy of the GNU General Public License along
** with this program; if not, write to the Free Software Foundation, Inc.,
** 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
**
****************************************************************************/

use task::short::{EId, AId};

use svgdom::Document;

pub fn remove_dupl_radial_gradients(doc: &Document) {
    let attrs = [
        AId::Cx,
        AId::Cy,
        AId::R,
        AId::Fx,
        AId::Fy,
        AId::GradientUnits,
        AId::SpreadMethod,
    ];

    super::rm_loop(doc, EId::RadialGradient, &attrs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use svgdom::{Document, WriteToString};
    use task::resolve_attrs;

    macro_rules! test_rg {
        ($name:ident, $in_text:expr, $out_text:expr) => (
            #[test]
            fn $name() {
                let doc = Document::from_data($in_text).unwrap();
                resolve_attrs::radial_gradients(&doc);
                remove_dupl_radial_gradients(&doc);
                assert_eq!(doc_to_str_tests!(doc), $out_text);
            }
        )
    }

    test_rg!(rm_1,
b"<svg>
    <defs>
        <radialGradient id='rg1' cx='0' cy='0' fx='5' fy='5' r='10'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </radialGradient>
        <radialGradient id='rg2' cx='0' cy='0' fx='5' fy='5' r='10'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </radialGradient>
    </defs>
    <rect fill='url(#rg2)'/>
</svg>",
"<svg>
    <defs>
        <radialGradient id='rg1' cx='0' cy='0' fx='5' fy='5' r='10'>
            <stop offset='0' stop-color='#ff0000'/>
            <stop offset='1' stop-color='#0000ff'/>
        </radialGradient>
    </defs>
    <rect fill='url(#rg1)'/>
</svg>
");

    test_rg!(rm_2,
b"<svg>
    <defs>
        <radialGradient id='rg1' cx='0' cy='0' fx='5' fy='5' r='10'/>
        <radialGradient id='rg2' cx='0' cy='0' fx='5' fy='5' r='10'/>
    </defs>
    <rect fill='url(#rg2)'/>
</svg>",
"<svg>
    <defs>
        <radialGradient id='rg1' cx='0' cy='0' fx='5' fy='5' r='10'/>
    </defs>
    <rect fill='url(#rg1)'/>
</svg>
");

    test_rg!(rm_3,
b"<svg>
    <defs>
        <radialGradient id='rg1' cx='5' cy='5' fx='5' r='10'/>
        <radialGradient id='rg2' cx='5' cy='5' fy='5' r='10'/>
    </defs>
    <rect fill='url(#rg2)'/>
</svg>",
"<svg>
    <defs>
        <radialGradient id='rg1' cx='5' cy='5' fx='5' r='10'/>
    </defs>
    <rect fill='url(#rg1)'/>
</svg>
");

    test_rg!(rm_4,
b"<svg>
    <defs>
        <radialGradient id='rg1' cx='5' cy='5' fx='5' \
            gradientTransform='matrix(1 0 0 1 10 20)' r='10'/>
        <radialGradient id='rg2' cx='5' cy='5' fy='5' \
            gradientTransform='matrix(1 0 0 1 10 20)' r='10'/>
        <radialGradient id='rg3' cx='5' cy='5' fy='5' r='10'/>
    </defs>
    <rect fill='url(#rg2)'/>
</svg>",
"<svg>
    <defs>
        <radialGradient id='rg1' cx='5' cy='5' fx='5' \
            gradientTransform='matrix(1 0 0 1 10 20)' r='10'/>
        <radialGradient id='rg3' cx='5' cy='5' fy='5' r='10'/>
    </defs>
    <rect fill='url(#rg1)'/>
</svg>
");

    test_rg!(rm_5,
b"<svg>
    <radialGradient id='rg1'/>
    <radialGradient id='rg2' xlink:href='#rg1'/>
</svg>",
"<svg>
    <radialGradient id='rg1'/>
</svg>
");

// TODO: this

//     test_rg!(rm_6,
// b"<svg>
//     <radialGradient id='rg2' xlink:href='#rg1'/>
//     <radialGradient id='rg1'/>
// </svg>",
// "<svg>
//     <radialGradient id='rg1'/>
// </svg>
// ");

    test_rg!(rm_7,
b"<svg>
    <radialGradient id='rg1'/>
    <radialGradient id='rg2' xlink:href='#rg1'/>
    <radialGradient id='rg3' xlink:href='#rg2'/>
</svg>",
"<svg>
    <radialGradient id='rg1'/>
</svg>
");

    test_rg!(rm_8,
b"<svg>
    <lenearGradient id='lg1'/>
    <radialGradient id='rg1' xlink:href='#lg1'/>
    <radialGradient id='rg2' xlink:href='#lg1'/>
</svg>",
"<svg>
    <lenearGradient id='lg1'/>
    <radialGradient id='rg1'/>
</svg>
");
}