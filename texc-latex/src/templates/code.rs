use crate::set;
use tex_rs::*;

type E = Element;
const CODE: &str = r#"#include <stdio.h>
int main()
{
    printf("Hello World!");
    return 0;
}
/*Comments*/
"#;

const PKGEXTRA: &str = r#"\definecolor{codegreen}{rgb}{0,0.6,0}
\definecolor{codegray}{rgb}{0.5,0.5,0.5}
\definecolor{codepurple}{rgb}{0.58,0,0.82}
\definecolor{backcolour}{rgb}{0.95,0.95,0.92}
\lstdefinestyle{lang_style}{
    backgroundcolor=\color{backcolour},
    commentstyle=\color{codegreen},
    keywordstyle=\color{magenta},
    numberstyle=\tiny\color{codegray},
    stringstyle=\color{codepurple},
    basicstyle=\ttfamily\footnotesize,
    breakatwhitespace=false,
    breaklines=true,
    captionpos=b,
    keepspaces=true,
    numbers=left,
    numbersep=5pt,
    showspaces=false,
    showstringspaces=false,
    showtabs=false,
    tabsize=2
}
\lstset{language=c}
\lstset{style=lang_style}
"#;

/// Creates the code template in `tex_rs::Latex`
/// ```
/// use texc_latex::code;
// ///
// /// fn main(){
// ///     let code_latex = code::code(11, "letterpaper", "article", "author", "title", "Some day", &vec![]);
// ///     // You can write with the following:
// ///     // code_latex.write(...)
// ///     // code_latex.async_write(...)
// ///     // code_latex.split_write(...), used in texcreate
// /// }
/// ```


pub fn code(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str, packages: &Vec<String>) -> Latex {
    let mut latex = Latex::new();
    set(&mut latex, fs, ps, dc, author, title, date);
    // Meta
    let input = UserDefined::new("\\input{src/structure.tex}", Level::Meta);

    //Body
    let mut intro = Section::new("Intro");
    let mut example = Environment::new("lstlisting");
    example.attach_string(CODE.to_string());
    let c_one = UserDefined::new_comment(
        "To use external code, use the following command",
        Level::Body,
    );
    let c_two = UserDefined::new_comment("\\lstinputlisting{file.c}", Level::Body);

    intro.attach(E::from(example)).unwrap();
    intro.attach(E::from(c_one)).unwrap();
    intro.attach(E::from(c_two)).unwrap();

    // Packages
    latex.add_package("graphicx".to_string());
    latex.add_package("listings".to_string());
    latex.add_package("xcolor".to_string());
    latex.add_package("amsmath".to_string());

    for i in packages{
        latex.add_package(i.to_string());
    }

    latex.set_elements(elements![
        input,
        intro,
        UserDefined::new(PKGEXTRA, Level::Package)
    ]);
    latex
}
