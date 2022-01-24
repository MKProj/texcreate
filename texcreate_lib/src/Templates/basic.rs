pub const BASIC_MAIN: &str = r#"% Basic Template
% MKProjects | GPLv2 & MIT License 
% From TexCreate
\documentclass[{font_size}pt, {paper_size}]{{doc_class}}
% MetaData Here
\author{{author}}
\title{{title}}
\date{{date}}
%Packages goes in structure.tex 
\input{structure.tex}
\begin{document}
    \maketitle
    \pagenumbering{arabic}
    \newpage
    % Document code here
\end{document}
"#;

pub const BASIC_STRUCTURE: &str = r#"\usepackage{amsmath}
% Extra packages from config.toml goes under here
"#;